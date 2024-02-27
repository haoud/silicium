use crate::{self as arch, gdt, opcode};
use arch_macros::per_cpu;
use macros::init;

/// The Task State Segment (TSS) for the current CPU core. It is initialized to an
/// uninitialized TSS and should be initialized before being used.
#[per_cpu]
static TSS: TaskStateSegment = TaskStateSegment::uninitialized();

/// The selector used to load the TSS in the current CPU core.
pub const LTR_SELECTOR: u16 = 0x30;

/// The index of the TSS entry in the local GDT.
pub const GDT_INDEX: usize = 6;

/// The Task State Segment (TSS) is a structure used by the x86 architecture to
/// store information about a task. On `x86_64`, the TSS is only used to store the
/// stack pointers for the different privilege levels, the Interrupt Stack Table
/// (IST) and the I/O port permissions.
#[repr(C, packed)]
pub struct TaskStateSegment {
    reserved0: u32,
    rsp0: u64,
    rsp1: u64,
    rsp2: u64,
    reserved1: u64,
    ist1: u64,
    ist2: u64,
    ist3: u64,
    ist4: u64,
    ist5: u64,
    ist6: u64,
    ist7: u64,
    reserved2: u64,
    reserved3: u16,
    iomap_base: u16,
}

impl TaskStateSegment {
    /// Creates a new Task State Segment (TSS) with uninitialized fields and without
    /// any I/O port permissions.
    #[must_use]
    pub const fn uninitialized() -> Self {
        Self {
            reserved0: 0,
            rsp0: 0,
            rsp1: 0,
            rsp2: 0,
            reserved1: 0,
            ist1: 0,
            ist2: 0,
            ist3: 0,
            ist4: 0,
            ist5: 0,
            ist6: 0,
            ist7: 0,
            reserved2: 0,
            reserved3: 0,
            iomap_base: 104,
        }
    }
}

/// Initializes the TSS, put it in the GDT and load it in the current CPU core
/// using the `ltr` instruction.
///
/// # Safety
/// This function should only called during the initialization of the kernel and
/// after the per-CPU data has been initialized. Failing to do so will result in
/// undefined behavior.
#[init]
pub unsafe fn setup() {
    gdt::load_tss(TSS.local().as_ptr());
    opcode::ltr(LTR_SELECTOR);
}
