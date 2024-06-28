use crate::arch::x86_64::{opcode, tss::TaskStateSegment};
use bitfield::{BitMut, BitRangeMut};
use macros::per_cpu;

core::arch::global_asm!(include_str!("asm/selectors.asm"));

extern "C" {
    /// Reloads the default GDT selectors in the current CPU core. This
    /// function set the ds`, `es` and `ss` selectors to 0x10, and the `cs`
    /// selector to 0x08.
    ///
    /// # Safety
    /// The caller must ensure that the GDT used by the kernel was loaded in
    /// the current CPU core. This function assumes that the second entry of
    /// the GDT is the 64-bits kernel code segment and the third entry is the
    /// 64-bits kernel data segment.
    /// This function should also be called only during the initialization of
    /// the kernel.
    /// Failing to meet this requirement will result in undefined behavior,
    /// most likely a triple fault and a immediate reboot of the system.
    fn reload_selectors();
}

/// The Global Descriptor Table (GDT) used by the kernel. It is a very standard
/// GDT that looks the same across most operating systems. It contains the
/// following entries:
/// 1. Null entry
/// 2. 64-bit kernel code segment
/// 3. 64-bit kernel data segment
/// 4. 32-bit user code segment
/// 5. 64-bit user data segment
/// 6. 64-bit user code segment
/// 7. TSS entry
/// 8. TSS entry
///
/// The disposition of the entries must not be changed as it is expected by
/// the rest of the kernel, and especially by the `syscall` and `sysret`
/// instructions that require an exact layout of the GDT to work properly.
///
/// Each CPU core has its own GDT, so this table is not shared between CPU
/// cores. This allow to have the same identifier for the TSS entry in the
/// GDT for all CPU
#[per_cpu]
static mut TABLE: [Entry; 8] = [
    // Null entry
    Entry(0),
    // 64-bit kernel code segment
    Entry(0x00af_9b00_0000_ffff),
    // 64-bit kernel data segment
    Entry(0x00af_9300_0000_ffff),
    // 32-bit user code segment
    Entry(0x008f_fb00_0000_ffff),
    // 64-bit user data segment
    Entry(0x00af_f300_0000_ffff),
    // 64-bit user code segment
    Entry(0x00af_fb00_0000_ffff),
    // TSS entry
    Entry(0),
    Entry(0),
];

/// A GDT entry. It is a simple wrapper around a 64-bits integer that
/// represents an entry in the GDT. I don't think it is necessary to
/// provide a constructor for an entry since the GDT is very static
/// and is almost always the same across all operating systems. We can
/// simply use 'magic numbers' to represent the entries in the GDT.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry(u64);

/// A GDT register. It is used to load the GDT in the current CPU core using
/// the `lgdt` instruction. It is a simple wrapper around a 16-bits limit and
/// a 64-bits base address that represents the GDT in memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, packed)]
pub struct Register {
    limit: u16,
    base: u64,
}

impl Register {
    /// Creates a new GDT register with the provided table. It will set the
    /// limit to the size of the table minus one and the base to the address
    /// of the table. The given table MUST stay in memory while it is loaded
    /// in the CPU !
    #[must_use]
    pub fn new(table: *const [Entry; 8]) -> Self {
        Self {
            #[allow(clippy::cast_possible_truncation)]
            limit: (core::mem::size_of::<[Entry; 8]>() - 1) as u16,
            base: table as u64,
        }
    }

    /// Creates a new GDT register that is uninitialized.
    #[must_use]
    pub const fn uninitialized() -> Self {
        Self { limit: 0, base: 0 }
    }

    /// Loads the GDT register with the current table in the current CPU core
    /// using the `lgdt` instruction.
    ///
    /// # Safety
    /// The caller must ensure that the GDT provided is valid and will remain
    /// valid for the entire lifetime of the kernel.
    pub unsafe fn load(&self) {
        opcode::lgdt(self);
    }
}

/// Initializes the GDT on the current CPU core and reloads the
/// default GDT selectors.
#[inline]
pub fn setup() {
    // SAFETY: This is safe because the GDT is valid and will remain valid
    // and in the memory for the entire lifetime of the kernel.
    unsafe {
        let register = Register::new(TABLE.local().as_ptr());
        register.load();
    }

    // SAFETY: This is safe because the GDT has the expected layout required
    // by the rest of the kernel: the second entry is the 64-bits kernel code
    // segment and the third entry is the 64-bits kernel data segment.
    unsafe {
        reload_selectors();
    }
}

/// Loads the provided Task State Segment (TSS) in the GDT. It will set the TSS
/// entries in the GDT to the provided TSS (the TSS entry needs to be split in
/// two parts). The TSS entry is the 6th and 7th entries, so the index of the
/// TSS in the GDT is 6 and its selector are 0x28
///
/// # Safety
/// The caller must ensure that the TSS provided will remain valid until the
/// TSS entry in the GDT is removed. Currently, this means that the TSS must
/// remain in memory for the entire lifetime of the kernel. The caller must
/// also ensure that the memory provided is accessible and readable. Failing
/// to meet these requirements will result in undefined behavior, and probably
/// a triple fault and an immediate reboot of the system.
#[inline]
pub unsafe fn load_tss(tss: *const TaskStateSegment) {
    let address = tss as u64;
    let mut low = 0;

    // Set the limit to the size of the TSS minus 1 (inclusive limit)
    low.set_bit_range(
        15,
        0,
        (core::mem::size_of::<TaskStateSegment>() - 1) as u64,
    );

    // Set the low 32 bits of the base address
    low.set_bit_range(63, 56, (address >> 24) & 0xFF);
    low.set_bit_range(39, 16, address & 0xFF_FFFF);

    // Set the type to 0b1001 (x86_64 available TSS)
    low.set_bit_range(43, 40, 0b1001);

    // Set the present bit to 1
    low.set_bit(47, true);

    // SAFETY: This is safe because the TSS is valid and will remain valid and
    // in the memory for the entire lifetime of the kernel. Also, there is no
    // other thread that can access the GDT at the same time, and we doesn't
    // not create multiple mutable references to the GDT.
    TABLE.local_mut()[6] = Entry(low);
    TABLE.local_mut()[7] = Entry(address >> 32);
}
