use crate::opcode;

core::arch::global_asm!(include_str!("asm/selectors.asm"));

extern "C" {
    /// Reloads the default GDT selectors in the current CPU core. This function
    /// set the ds`, `es` and `ss` selectors to 0x10, and the `cs` selector to
    /// 0x08.
    ///
    /// # Safety
    /// The caller must ensure that the GDT used by the kernel was loaded in the
    /// current CPU core. This function assumes that the second entry of the GDT
    /// is the 64-bits kernel code segment and the third entry is the 64-bits
    /// kernel data segment.
    /// Failing to meet this requirement will result in undefined behavior, most
    /// likely a triple fault and a immediate reboot of the system.
    fn reload_selectors();
}

/// The Global Descriptor Table (GDT) used by the kernel. It is a very standard GDT
/// that looks the same across most operating systems. It contains the following
/// entries:
/// 1. Null entry
/// 2. 64-bit kernel code segment
/// 3. 64-bit kernel data segment
/// 4. 32-bit user code segment
/// 5. 64-bit user data segment
/// 6. 64-bit user code segment
///
/// The disposition of the entries must not be changed as it is expected by the
/// rest of the kernel, and especially by the `syscall` and `sysret` instructions
/// that require an exact layout of the GDT to work properly.
static TABLE: [Entry; 6] = [
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
];

/// A GDT entry. It is a simple wrapper around a 64-bits integer that represents
/// an entry in the GDT. I don't think it is necessary to provide a constructor
/// for an entry since the GDT is very static and is almost always the same across
/// all operating systems. We can simply use 'magic numbers' to represent the
/// entries in the GDT.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry(u64);

/// A GDT register. It is used to load the GDT in the current CPU core using the
/// `lgdt` instruction. It is a simple wrapper around a 16-bits limit and a 64-bits
/// base address that represents the GDT in memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, packed)]
pub struct Register {
    limit: u16,
    base: u64,
}

impl Register {
    /// Creates a new GDT register with the provided table. It will set the limit
    /// to the size of the table minus one and the base to the address of the table.
    pub fn new(table: &'static [Entry; 6]) -> Self {
        Self {
            limit: (core::mem::size_of::<[Entry; 6]>() - 1) as u16,
            base: table.as_ptr() as u64,
        }
    }

    /// Creates a new GDT register that is uninitialized.
    pub const fn uninitialized() -> Self {
        Self { limit: 0, base: 0 }
    }

    /// Loads the GDT register with the current table in the current CPU core using
    /// the `lgdt` instruction.
    ///
    /// # Safety
    /// The caller must ensure that the GDT provided is valid and will remain valid
    /// for the entire lifetime of the kernel.
    pub unsafe fn load(&self) {
        opcode::lgdt(self);
    }
}

/// Initializes the GDT on the current CPU core and reloads the
/// default GDT selectors.
pub fn setup() {
    // SAFETY: This is safe because the GDT is valid and will remain valid and in
    // the memory for the entire lifetime of the kernel.
    unsafe {
        let register = Register::new(&TABLE);
        register.load();
    }

    // SAFETY: This is safe because the GDT has the expected layout required by the
    // rest of the kernel: the second entry is the 64-bits kernel code segment and
    // the third entry is the 64-bits kernel data segment.
    unsafe {
        reload_selectors();
    }
}
