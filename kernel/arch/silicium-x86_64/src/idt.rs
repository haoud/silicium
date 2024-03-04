use crate::{
    apic,
    cpu::{self, InterruptFrame},
    opcode,
};
use macros::init;

core::arch::global_asm!(include_str!("asm/interrupt.asm"));

extern "C" {
    /// The interrupt handlers table. This table is defined in the `interrupt.asm`, and
    /// this symbol is used to get the address of the table in the Rust code.
    static interrupt_handlers: [usize; 0];
}

/// The Interrupt Descriptor Table (IDT) used by the kernel. This is an uninitialized
/// table that will be filled with valid descriptors during the initialization of the
/// kernel.
static mut TABLE: [Descriptor; 256] = [Descriptor::new(0); 256];

/// An IDT descriptor. An IDT descriptor is a 16 bytes structure that contains the
/// address of the handler, the segment selector and the descriptor flags. For more
/// details, see the Intel manual (Volume 3, Chapter 6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, packed)]
pub struct Descriptor {
    /// The low 16 bits of the handler address.
    offset_low: u16,

    /// The segment selector that will be loaded in the `cs` register if the
    /// interrupt is triggered from user mode.
    selector: u16,

    /// A set of flags that define the handler type and the privilege level.
    /// In Silicium, we use the `0x8E00` flags that define a 64 bits interrupt
    /// gate with a privilege level of 0, meaning that only the kernel can manually
    /// trigger the interrupt (interrupts triggers by the hardware will still work
    /// if it happens in user mode) and interrupt handler will be executed with
    /// interrupts disabled.
    flags: u16,

    /// The middle 16 bits of the handler address.
    offset_middle: u16,

    /// The high 32 bits of the handler address.
    offset_high: u32,

    /// A reserved field that must be set to 0.
    zero: u32,
}

impl Descriptor {
    /// Create a new IDT descriptor with the provided handler address. It will set
    /// the segment selector to 0x08, the flags to 0x8E00 (interrupt gate with a
    /// privilege level of 0) and the handler address to the provided address.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn new(handler: usize) -> Self {
        Self {
            offset_low: handler as u16,
            selector: 0x08,
            flags: 0x8E00,
            offset_middle: (handler >> 16) as u16,
            offset_high: (handler >> 32) as u32,
            zero: 0,
        }
    }

    /// Create a new uninitialized IDT descriptor. If this descriptor is read
    /// by the CPU, it will result in a general protection fault, and a triple
    /// fault and a reboot of the computer if the CPU doesn't have a proper
    /// exception handler set up.
    #[must_use]
    pub const fn uninitialized() -> Self {
        Self::new(0)
    }
}

/// An IDT register. It is used to load the IDT in the current CPU core using the
/// `lidt` instruction. It is a simple wrapper around a 16-bits limit and a 64-bits
/// base address that represents the IDT in memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, packed)]
pub struct Register {
    /// The size of the IDT in bytes minus one (because the limit is inclusive).
    limit: u16,

    /// The base address of the IDT in memory.
    base: u64,
}

impl Register {
    /// Creates a new uninitialized IDT register. Loading this IDT register in the
    /// CPU will result in a general protection fault, and a triple fault and a
    /// reboot of the computer if the CPU doesn't have a proper exception handler
    /// set up.
    #[must_use]
    pub const fn uninitialized() -> Self {
        Self { limit: 0, base: 0 }
    }

    /// Creates a new IDT register with the provided table. It will set the limit
    /// to the size of the table minus one and the base to the address of the table.
    /// The given table MUST stay in memory while it is loaded in the CPU !
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn new(descriptors: *const [Descriptor; 256]) -> Self {
        Self {
            limit: (core::mem::size_of::<[Descriptor; 256]>() - 1) as u16,
            base: descriptors as u64,
        }
    }

    /// Load the IDT register with the current table in the current CPU core using
    /// the `lidt` instruction.
    ///
    /// # Safety
    /// The caller must ensure that the IDT provided is valid and will remain valid
    /// for the entire lifetime of the kernel.
    pub unsafe fn load(&self) {
        opcode::lidt(self);
    }
}

#[inline]
pub fn setup() {
    // SAFETY: This is safe because we are initializing the IDT table with valid
    // descriptors. Since we use static mut, we must ensure that we have exclusive
    // access to the IDT table (that is the case during the boot process where only
    // one CPU are active) and that we doesn't make multiple mutable references to
    // the IDT table.
    unsafe {
        // The interrupt handlers table is defined in the `asm/interrupt.asm` file. It
        // is a table of simple code stubs that will be called by the CPU when an
        // interrupt is triggered. Each stub has a size of 16 bytes, and the address
        // of the first stub is the address of the `interrupt_handlers` symbol.
        let start = core::ptr::addr_of!(interrupt_handlers) as usize;
        for (i, descriptor) in TABLE.iter_mut().enumerate() {
            *descriptor = Descriptor::new(start + i * 16);
        }
    }
}

/// Load the IDT in the current CPU core using the `lidt` instruction.
///
/// # Safety
/// The caller must ensure that the IDT provided is initialized, valid and
/// must remain valid for the entire lifetime of the kernel. This function
/// should also only called only during the initialization of the kernel.
/// Failure to do so will result in a general protection fault, and a triple
/// fault and a reboot of the computer if the CPU doesn't have a proper
/// exception handler set up.
#[init]
pub unsafe fn load() {
    let register = Register::new(core::ptr::addr_of!(TABLE));
    register.load();
}

#[no_mangle]
#[allow(clippy::missing_panics_doc)]
pub extern "C" fn irq_handler(frame: &mut InterruptFrame) {
    // If the interrupt is raised by the I/O APIC, we must send
    // an EOI to the APIC, otherwise no more interrupts will be
    // received from the local APIC.
    if apic::io::own_irq((frame.irq & 0xFF) as u8) {
        apic::local::end_of_interrupt();
    }
}
