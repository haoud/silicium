use bitflags::bitflags;

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Flags : u64 {
        /// Carry flag. Set if an arithmetic operation generates a carry or a
        /// borrow out of the most significant bit of the result; cleared
        /// otherwise. This flag can be armed with the `stc` instruction and
        /// disarmed with the `clc` instruction.
        const CF = 1 << 0;

        /// Parity flag. Set if the least-significant byte of the result
        /// contains an even number of 1 bits; cleared otherwise.
        const PF = 1 << 2;

        /// Adjust flag. Set if an arithmetic operation generates a carry or a
        /// borrow out of bit 3 into bit 4 of the result, cleared otherwise.
        /// This flag is used by the BCD (binary-coded decimal) arithmetic, and
        /// that's probably all it's used for.
        const AF = 1 << 4;

        /// Zero flag. Set if the result of an operation is zero; cleared
        /// otherwise.
        const ZF = 1 << 6;

        /// Sign flag. Set equal to the most-significant bit of the result,
        /// which is the sign bit of a signed integer. (0 indicates a positive
        /// value and 1 indicates a negative value).
        const SF = 1 << 7;

        /// Trap flag. Set to enable single-step mode for debugging; cleared
        /// otherwise (normal functioning).
        const TF = 1 << 8;

        /// Interrupt enable flag. Set to enable external maskable interrupts;
        /// cleared to disable them. This flags can be armed with the `sti`
        /// instruction and disarmed with the `cli` instruction.
        const IF = 1 << 9;

        /// Direction flag. Set to enable automatic decrementing of the `rdi`
        /// and `rsi` registers during string instructions; cleared to enable
        /// automatic incrementing of those registers. The system V ABI
        /// require s this flag to be cleared on function entry (used by
        /// the kernel).
        const DF = 1 << 10;

        /// Overflow flag. Set if an signed arithmetic operation generates a
        /// result too large to be represented in the destination operand,
        /// cleared otherwise.
        const OF = 1 << 11;

        /// I/O privilege bit 0. This flags is used in conjunction with the I/O
        /// privilege bit 1 to form a 2-bit field that specifies the minimum
        /// privilege level required to execute I/O instructions.
        const IOPL0 = 1 << 12;

        /// I/O privilege bit 1. This flags is used in conjunction with the I/O
        /// privilege bit 0 to form a 2-bit field that specifies the minimum
        /// privilege level required to execute I/O instructions.
        const IOPL1 = 1 << 13;

        /// Nested task flag. Controls the chaining of interrupts. If set, the
        /// processor disables external interrupts until the next IRET
        /// instruction is executed. This provides a mechanism for the nesting
        /// of interrupts. This flag is cleared when an interrupt is generated
        /// and set when an IRET instruction is executed.
        const NT = 1 << 14;

        /// Resume flag. Set to indicate that the processor is executing in a
        /// debug environment.
        const RF = 1 << 16;

        /// Virtual 8086 mode flag. Set to enable the virtual 8086 mode,
        /// cleared to return to protected mode. This flags is not supported
        /// on `x86_64`.
        const VM = 1 << 17;

        /// Alignment check. Set if the alignment check is enabled; cleared
        /// otherwise. To work properly, the AM flag in the CR0 register must
        /// also be set.
        const AC = 1 << 18;

        /// Virtual interrupt flag. Set to enable the virtual interrupt flag;
        /// cleared to disable it.
        const VIF = 1 << 19;

        /// Virtual interrupt pending flag. Set to indicate that an interrupt
        /// is pending; cleared when no interrupt is pending.
        const VIP = 1 << 20;

        /// Identification flag. The ability of a program to set or clear this
        /// flag indicates support for the CPUID instruction. On `x86_64`, this
        /// flag is always set.
        const ID = 1 << 21;
    }
}

/// Read the RFLAGS register and return its value.
#[inline]
#[must_use]
pub fn read() -> Flags {
    let rflags;
    // SAFETY: This is safe because we are only reading the RFLAGS register and
    // this should not have any side effects nor cause any unsafety.
    unsafe {
        core::arch::asm!("
            pushfq
            pop {0}",
            out(reg) rflags
        );
    }
    Flags::from_bits_truncate(rflags)
}

/// Enable the specified flags in the RFLAGS register.
///
/// # Safety
/// This is unsafe because modifying the RFLAGS register can have unexpected
/// side effects on the CPU and the system as a whole. Therefore, the caller
/// must ensure that the value being written is valid and safe to use in his
/// context.
#[inline]
pub unsafe fn enable(flags: Flags) {
    let current = read();
    core::arch::asm!("
        push {0}
        popfq",
        in(reg) (current | flags).bits()
    );
}

/// Disable the specified flags in the RFLAGS register.
///
/// # Safety
/// This is unsafe because modifying the RFLAGS register can have unexpected
/// side effects on the CPU and the system as a whole. Therefore, the caller
/// must ensure that the value being written is valid and safe to use in his
/// context.
#[inline]
pub unsafe fn disable(flags: Flags) {
    let current = read();
    core::arch::asm!("
        push {0}
        popfq",
        in(reg) (current & !flags).bits()
    );
}
