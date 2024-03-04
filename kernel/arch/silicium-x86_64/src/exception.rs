use crate::cpu::{self, InterruptFrame};

/// Divide by zero exception vector. This exception is triggered when the CPU tries to
/// divide a number by zero.
pub const DE_VECTOR: u8 = 0;

/// Debug exception vector. This exception is triggered when the CPU is single-stepping
/// through the code (when the TF flag is set in the RFLAGS register).
pub const DB_VECTOR: u8 = 1;

/// Non-maskable interrupt vector. This exception is triggered by the hardware when a
/// unrecoverable hardware error occurs. This exception cannot be masked by the IF flag
/// in the RFLAGS register.
pub const NMI_VECTOR: u8 = 2;

/// Breakpoint exception vector. This exception is triggered when the `int3` instruction
/// is executed. This instruction is used by the debugger to set a breakpoint in the code.
pub const BP_VECTOR: u8 = 3;

/// Overflow exception vector. This exception is triggered when the `into` instruction is
/// executed and the OF flag is set in the RFLAGS register.
pub const OF_VECTOR: u8 = 4;

/// Bound range exceeded exception vector. This exception is triggered when the `bound`
/// instruction is executed and the value is not within the specified range.
pub const BR_VECTOR: u8 = 5;

/// Invalid opcode exception vector. This exception is triggered when the CPU tries to
/// execute an invalid opcode. This can happen when the CPU tries to execute a opcode
/// that is not implemented.
pub const UD_VECTOR: u8 = 6;

/// Device not available exception vector. This exception is triggered when an FPU
/// instruction is executed and the EM or TS flags are set in the CR0 register, or when
/// there is trully no FPU in the system (this should never happen in `x86_64`).
pub const NM_VECTOR: u8 = 7;

/// Double fault exception vector. This exception is triggered when an exception is
/// triggered while the CPU is handling another exception. This exception is used to
/// prevent infinite loops of exceptions. If an exception is triggered while the CPU
/// try to invoke the double fault handler, the CPU will trigger a triple fault and
/// reboot the computer.
pub const DF_VECTOR: u8 = 8;

/// Invalid TSS exception vector. This exception is triggered when the CPU tries to
/// load a invalid TSS segment.
pub const TS_VECTOR: u8 = 10;

/// Segment not present exception vector. This exception is triggered when the CPU tries
/// to load a segment that is not present in the GDT.
pub const NP_VECTOR: u8 = 11;

/// Stack-segment fault exception vector. This exception can be triggered when:
/// - Loading a stack segment that is not present in the GDT.
/// - Any instruction using the stack while the stack address is not canonical.
/// - When the stack limit is exceeded.
pub const SS_VECTOR: u8 = 12;

/// General protection fault exception vector. This exception can be triggered for many,
/// many reasons, including:
/// - When the CPU tries to access a non-canonical address (an address where the 48-63
/// bits are not matching the 47th bit).
/// - Setting a reserved bit in a register or a field, or an invalid combination of
///  fields.
/// - When the CPU tries to execute a privileged instruction in user mode.
/// - Segment errors (privilege level, type, not present...).
pub const GP_VECTOR: u8 = 13;

/// Page fault exception vector. This exception can be triggered when:
/// - The CPU tries to access a page that is not present in the page table.
/// - The CPU tries to write to a read-only page.
/// - The CPU tries to execute a page that is not executable.
/// - The CPU tries to access a page with the wrong privilege level.
/// - A reserved bit is set in the page table entry.
pub const PF_VECTOR: u8 = 14;

/// Floating-point exception vector. This exception is triggered when using any
/// waiting FPU instruction (including `fwait` and `wait`), when the `NE` flag is
/// set in the CR0 register and an unmasked x87 FPU exception is pending.
pub const MF_VECTOR: u8 = 16;

/// Alignment check exception vector. This exception is triggered when the CPU detect
/// an unaligned memory access while the `AC` flag is set in the `EFLAGS` register and
/// the `AM` flag is set in the `CR0` register.
pub const AC_VECTOR: u8 = 17;

/// Machine check exception vector. This exception is triggered when the CPU detect a
/// hardware error that cannot be corrected by the CPU.
pub const MC_VECTOR: u8 = 18;

/// SIMD floating-point exception vector. This exception is triggered when an unmasked
/// SIMD floating-point exception is pending and the `OSXMMEXCPT` flag is set in the
/// `XCR0` register. If this flag is not set, the exception will be an invalid opcode
/// exception (#UD) instead of this exception.
pub const XF_VECTOR: u8 = 19;

/// Handle an exception. This function should be called when an exception is triggered.
///
/// # Panics
/// Panics if the exception cannot be handled by the kernel or if the exception is not
/// an exception ([`own_interrupt`] returns false).
pub fn handle(exception: u8, _frame: &mut InterruptFrame) {
    match exception {
        DE_VECTOR => {
            panic!("Unhandled divide by zero exception");
        }
        DB_VECTOR => {
            panic!("Unhandled debug exception");
        }
        NMI_VECTOR => {
            panic!("Unhandled non-maskable interrupt");
        }
        BP_VECTOR => {
            panic!("Unhandled breakpoint exception");
        }
        OF_VECTOR => {
            panic!("Unhandled overflow exception");
        }
        BR_VECTOR => {
            panic!("Unhandled bound range exceeded exception");
        }
        UD_VECTOR => {
            panic!("Unhandled invalid opcode exception");
        }
        NM_VECTOR => {
            panic!("Unhandled device not available exception");
        }
        DF_VECTOR => {
            panic!("Unhandled double fault exception");
        }
        TS_VECTOR => {
            panic!("Unhandled invalid TSS exception");
        }
        NP_VECTOR => {
            panic!("Unhandled segment not present exception");
        }
        SS_VECTOR => {
            panic!("Unhandled stack-segment fault exception");
        }
        GP_VECTOR => {
            panic!("Unhandled general protection fault");
        }
        PF_VECTOR => {
            let cr2 = cpu::cr2::read();
            panic!("Unhandled page fault for address: {:#x}", cr2);
        }
        MF_VECTOR => {
            panic!("Unhandled floating-point exception");
        }
        AC_VECTOR => {
            panic!("Unhandled alignment check exception");
        }
        MC_VECTOR => {
            panic!("Unhandled machine check exception");
        }
        XF_VECTOR => {
            panic!("Unhandled SIMD floating-point exception");
        }
        _ => {
            panic!("Unhandled exception: {}", exception);
        }
    }
}

/// Return true if the interrupt is an exception, false otherwise.
#[must_use]
pub const fn own_interrupt(vector: u8) -> bool {
    vector < 32
}
