use addr::Virtual;
use macros::init;

pub mod timer;

/// The base address of the LAPIC MMIO
pub const LAPIC_BASE: Virtual = Virtual::new(0xFFFF_8000_FEE0_0000);

/// A register in the local APIC. The register value must be added to the
/// LAPIC base address in order to access the register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Register(usize);

impl Register {
    pub const LAPIC_ID: Register = Register(0x20);
    pub const LAPIC_VERSION: Register = Register(0x30);
    pub const TASK_PRIORITY: Register = Register(0x80);
    pub const ARBITRATION_PRIORITY: Register = Register(0x90);
    pub const PROCESSOR_PRIORITY: Register = Register(0xA0);
    pub const EOI: Register = Register(0xB0);
    pub const REMOTE_READ: Register = Register(0xC0);
    pub const LOGICAL_DESTINATION: Register = Register(0xD0);
    pub const DESTINATION_FORMAT: Register = Register(0xE0);
    pub const SPURIOUS_INTERRUPT_VECTOR: Register = Register(0xF0);

    pub const IN_SERVICE_0: Register = Register(0x100);
    pub const IN_SERVICE_1: Register = Register(0x110);
    pub const IN_SERVICE_2: Register = Register(0x120);
    pub const IN_SERVICE_3: Register = Register(0x130);
    pub const IN_SERVICE_4: Register = Register(0x140);
    pub const IN_SERVICE_5: Register = Register(0x150);
    pub const IN_SERVICE_6: Register = Register(0x160);
    pub const IN_SERVICE_7: Register = Register(0x170);

    pub const TRIGGER_MODE_0: Register = Register(0x180);
    pub const TRIGGER_MODE_1: Register = Register(0x190);
    pub const TRIGGER_MODE_2: Register = Register(0x1A0);
    pub const TRIGGER_MODE_3: Register = Register(0x1B0);
    pub const TRIGGER_MODE_4: Register = Register(0x1C0);
    pub const TRIGGER_MODE_5: Register = Register(0x1D0);
    pub const TRIGGER_MODE_6: Register = Register(0x1E0);
    pub const TRIGGER_MODE_7: Register = Register(0x1F0);

    pub const INTERRUPT_REQUEST_0: Register = Register(0x200);
    pub const INTERRUPT_REQUEST_1: Register = Register(0x210);
    pub const INTERRUPT_REQUEST_2: Register = Register(0x220);
    pub const INTERRUPT_REQUEST_3: Register = Register(0x230);
    pub const INTERRUPT_REQUEST_4: Register = Register(0x240);
    pub const INTERRUPT_REQUEST_5: Register = Register(0x250);
    pub const INTERRUPT_REQUEST_6: Register = Register(0x260);
    pub const INTERRUPT_REQUEST_7: Register = Register(0x270);

    pub const ERROR_STATUS: Register = Register(0x280);

    pub const LVT_CMCI: Register = Register(0x2F0);
    pub const INTERRUPT_COMMAND0: Register = Register(0x300);
    pub const INTERRUPT_COMMAND1: Register = Register(0x310);

    pub const LVT_TIMER: Register = Register(0x320);
    pub const LVT_THERMAL_SENSOR: Register = Register(0x330);
    pub const LVT_PERFORMANCE_MONITORING_COUNTERS: Register = Register(0x340);
    pub const LVT_LINT0: Register = Register(0x350);
    pub const LVT_LINT1: Register = Register(0x360);
    pub const LVT_ERROR: Register = Register(0x370);

    pub const INITIAL_COUNT: Register = Register(0x380);
    pub const CURRENT_COUNT: Register = Register(0x390);

    pub const DIVIDE_CONFIGURATION: Register = Register(0x3E0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IpiDestination {
    /// Send the IPI to all cores, including the current core
    AllIncludingSelf,

    /// Send the IPI to all cores, excluding the current core
    AllExcludingSelf,

    /// Send the IPI to the current core only
    SelfOnly,

    /// Send the IPI to a specific core, identified by its APIC ID
    Single(u8),
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IpiPriority {
    /// Send the IPI with a fixed priority
    #[default]
    Fixed = 0,
    /// Send the IPI with a low priority
    Low = 1,
    /// Send an NMI: the interrupt vector is ignored in this case
    Nmi = 2,
    /// Send a SMI
    Smi = 4,
}

/// Initialize the local APIC
///
/// # Safety
/// This function is unsafe because it should only be called once per core, and
/// should only be called after the LAPIC MMIO has been remapped to its correct
/// virtual address. The caller also ensure that this function is only called
/// during the kernel initialization.
#[init]
pub unsafe fn setup() {
    // Enable the local APIC and set the spurious interrupt vector to 255
    write(Register::SPURIOUS_INTERRUPT_VECTOR, 0x1FF);
}

/// Send an Inter-Processor Interrupt (IPI) to the given destination.
///
/// An IPI is a message sent between cores in a multi-core system. It can be
/// used to trigger an interrupt on another core, or to perform other inter
/// core communication.
///
/// # Safety
/// This function is unsafe because triggering an IPI can have side effects
/// that could cause undefined behavior or memory usafety. The caller must
/// ensure that triggering an IPI is safe and will not cause any undefined
/// behavior (for example, by ensuring that the destination core is valid
/// and have a proper interrupt handler).
pub unsafe fn send_ipi(
    destination: IpiDestination,
    priority: IpiPriority,
    vector: u8,
) {
    let cmd = match destination {
        IpiDestination::Single(core) => (
            u32::from(core) << 24,
            u32::from(vector) | (priority as u32) << 8,
        ),
        IpiDestination::AllIncludingSelf => {
            (0, u32::from(vector) | ((priority as u32) << 8) | 2 << 18)
        }
        IpiDestination::AllExcludingSelf => {
            (0, u32::from(vector) | ((priority as u32) << 8) | 3 << 18)
        }
        IpiDestination::SelfOnly => {
            (0, u32::from(vector) | ((priority as u32) << 8) | 1 << 18)
        }
    };

    // Send the IPI
    write(Register::INTERRUPT_COMMAND1, cmd.0);
    write(Register::INTERRUPT_COMMAND0, cmd.1);

    // Wait for the IPI to be sent
    while read(Register::INTERRUPT_COMMAND0) & (1 << 12) != 0 {
        core::hint::spin_loop();
    }
}

/// Send an EOI to the local APIC.
///
/// When an interrupt is received, the local APIC will not trigger another
/// interrupt until an EOI has been sent.
///
/// This function signals the end of the interrupt to the local APIC, allowing
/// it to trigger another interrupt if necessary.
pub fn end_of_interrupt() {
    // SAFETY: Writing to the EOI register is safe because it should not
    // have any side effects that could cause undefined behavior.
    unsafe {
        write(Register::EOI, 0);
    }
}

/// Writes a 32-bit value to the given register
///
/// # Safety
/// This function is unsafe because it writes to a memory-mapped I/O register.
/// This could cause unexpected side effects depending on the register being
/// written to, and could lead to undefined behavior or memory unsafety in the
/// rest of the program. The caller must ensure that writing to the given
/// register is valid and will not cause any undefined behavior.
pub unsafe fn write(reg: Register, value: u32) {
    let ptr = (usize::from(LAPIC_BASE) + reg.0) as *mut u32;
    ptr.write_volatile(value);
}

/// Reads a 32-bit value from the given register
///
/// # Safety
/// This function is unsafe because it reads from a memory-mapped I/O register.
/// This could cause unexpected side effects depending on the register being
/// read, and could lead to undefined behavior or memory unsafety in the rest
/// of the program. The caller must ensure that reading from the given register
/// is valid and will not cause any undefined behavior.
#[must_use]
pub unsafe fn read(reg: Register) -> u32 {
    let ptr = (usize::from(LAPIC_BASE) + reg.0) as *const u32;
    ptr.read_volatile()
}
