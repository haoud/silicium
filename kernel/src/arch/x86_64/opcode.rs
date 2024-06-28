use crate::arch::x86_64::{gdt, idt};

/// Enable interrupts on the current CPU core, allowing the current workflow
/// to be interrupted by hardware interrupts. If interrupts are already
/// enabled, this function will have no effect.
///
/// # Safety
/// This function is unsafe because this function make some assumptions about
/// the state of the system. The caller must ensure that the current CPU core
/// is in a state where it is safe to enable interrupts, and that the core can
/// handle the interrupts that will be generated without triple faulting.
/// Failing to meet these requirements can result in undefined behavior.
#[inline]
pub unsafe fn sti() {
    core::arch::asm!("sti");
}

/// Disable interrupts on the current CPU core. This will prevent the CPU from
/// receiving any interrupts until they are re-enabled. If the same interrupt
/// is received multiple times while interrupts are disabled, it will only be
/// handled once interrupts are re-enabled.
///
/// If interrupts are already disabled, this function will have no effect.
#[inline]
pub fn cli() {
    // SAFETY: This is safe because disabling interrupts should not break
    // Rust's safety guarantees, unlike enabling them.
    unsafe {
        core::arch::asm!("cli");
    }
}

/// Halt the CPU until the next interrupt is received. If interrupts are
/// disabled, this will effectively halt the CPU indefinitely.
#[inline]
pub fn hlt() {
    // SAFETY: This is safe because waiting for an interrupt should not
    // break Rust's safety guarantees. If interrupts are disabled, this
    // will effectively halt the CPU indefinitely, but again, this is not
    // a memory safety issue.
    unsafe {
        core::arch::asm!("hlt");
    }
}

/// Write an 8 bit value from a port.
///
/// # Safety
/// This function is unsafe because writing to a port can have side effects,
/// including causing the hardware to do something unexpected and possibly
/// violating memory safety.
#[inline]
pub unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") value);
}

/// Write an 16 bit value to a port.
///
/// # Safety
/// This function is unsafe because writing to a port can have side effects,
/// including causing the hardware to do something unexpected and possibly
/// violating memory safety.
#[inline]
pub unsafe fn outw(port: u16, value: u16) {
    core::arch::asm!("out dx, ax", in("dx") port, in("ax") value);
}

/// Write an 32 bit value to a port.
///
/// # Safety
/// This function is unsafe because writing to a port can have side effects,
/// including causing the hardware to do something unexpected and possibly
/// violating memory safety.
#[inline]
pub unsafe fn outd(port: u16, value: u32) {
    core::arch::asm!("out dx, eax", in("dx") port, in("eax") value);
}

/// Read an 8 bit value from a port.
///
/// # Safety
/// This function is unsafe because reading from a port can have side effects,
/// including causing the hardware to do something unexpected and possibly
/// violating memory safety.
#[inline]
#[must_use]
pub unsafe fn inb(port: u16) -> u8 {
    let mut value: u8;
    core::arch::asm!("in al, dx", in("dx") port, out("al") value);
    value
}

/// Read an 16 bit value from a port.
///
/// # Safety
/// This function is unsafe because reading from a port can have side effects,
/// including causing the hardware to do something unexpected and possibly violating memory safety.
#[inline]
#[must_use]
pub unsafe fn inw(port: u16) -> u16 {
    let mut value: u16;
    core::arch::asm!("in ax, dx", in("dx") port, out("ax") value);
    value
}

/// Read an 32 bit value from a port.
///
/// # Safety
/// This function is unsafe because reading from a port can have side effects,
/// including causing the hardware to do something unexpected and possibly
/// violating memory safety.
#[inline]
#[must_use]
pub unsafe fn ind(port: u16) -> u32 {
    let mut value: u32;
    core::arch::asm!("in eax, dx", in("dx") port, out("eax") value);
    value
}

/// Load the Global Descriptor Table (GDT) register with the provided GDT
/// register value.
///
/// # Safety
/// The caller must ensure that the provided GDT reigster value is valid and
/// reference a valid GDT that must stay in memory while it is loaded into the
/// GDT register.Failing to meet these requirements can result in undefined
/// behavior, memory unsafety or crashes.
///
/// However, the GDT register structure can be dropped as soon as the function
/// returns because the CPU will keep a copy of the GDT register in its
/// internal state.
#[inline]
pub unsafe fn lgdt(gdtr: &gdt::Register) {
    core::arch::asm!("lgdt [{}]", in(reg) gdtr);
}

/// Load the Interrupt Descriptor Table (IDT) register with the provided IDT
/// register value.
///
/// # Safety
/// The caller must ensure that the provided IDT reigster value is valid and
/// reference a valid IDT that must stay in memory while it is loaded into the
/// IDT register. Failing to meet these requirements can result in undefined
/// behavior, memory unsafety or crashes.
///
/// However, the IDT register structure can be dropped as soon as the function
/// returns because the CPU will keep a copy of the IDT register in its
/// internal state.
#[inline]
pub unsafe fn lidt(idtr: &idt::Register) {
    core::arch::asm!("lidt [{}]", in(reg) idtr);
}

/// Load the Task State Segment (TSS) register with the provided TSS selector.
///
/// # Safety
/// The caller must ensure that the provided TSS selector is valid and
/// reference a valid TSS inside the GDT. The TSS entry and the TSS itself
/// must stay in memory while it is loaded into the TSS register. Failing
/// to meet these requirements can result in undefined behavior, memory
/// unsafety or crashes.
#[inline]
pub unsafe fn ltr(selector: u16) {
    core::arch::asm!("ltr ax", in("ax") selector);
}

/// Invalidate the Translation Lookaside Buffer (TLB) entry for the
/// provided virtual address. This should be used only when needed
/// because it will cause a performance penalty when the CPU will
/// translate the virtual address to a physical address.
#[inline]
pub fn invlpg(address: usize) {
    // SAFETY: This is safe because invalidating a TLB entry should not cause
    // memory unsafety. It will cause a performance penalty, but this is not
    // a memory safety issue.
    unsafe {
        core::arch::asm!("invlpg [{}]", in(reg) address);
    }
}

/// Set the value of the Extended Control Register (XCR0) to the provided
/// value.
///
/// # Safety
/// The caller must ensure that the `xsetbv` instruction is supported by the
/// CPU, and that the provided value is valid and will not cause UB or memory
/// unsafety.
#[inline]
pub unsafe fn xsetbv(index: u32, value: u64) {
    core::arch::asm!(
        "xsetbv",
        in("ecx") index,
        in("eax") (value & 0xFFFF_FFFF) as u32,
        in("edx") (value >> 32) as u32
    );
}

/// Get the value of the Extended Control Register (XCR0) for the provided
/// index.
///
/// # Safety
/// The caller must ensure that the `xgetbv` instruction is supported by the
/// CPU.
#[inline]
#[must_use]
pub unsafe fn xgetbv(index: u32) -> u64 {
    let (low, high): (u32, u32);
    core::arch::asm!(
        "xgetbv",
        in("ecx") index,
        out("eax") low,
        out("edx") high,
    );
    u64::from(high) << 32 | u64::from(low)
}

/// Save the extended state of the CPU into the provided buffer.
///
/// # Safety
/// The caller must ensure that the provided buffer is valid and has enough
/// space to store the extended state of the CPU. The caller must also ensure
/// that the `xsave` instruction is supported by the CPU.
#[inline]
pub unsafe fn xsave(buffer: *mut u8) {
    core::arch::asm!(
        "xsave [{}]",
        in(reg) buffer,
        in("eax") u32::MAX,
        in("edx") u32::MAX
    );
}

/// Restore the extended state of the CPU from the provided buffer using
/// features specified in the `xCR0`
///
/// # Safety
/// The caller must ensure that the provided buffer is valid and contains the
/// extended state of the CPU. The caller must also ensure that the `xrstor`
/// instruction is supported by the CPU, and that the data inside the buffer
/// was previously saved using the `xsave` instruction.
#[inline]
pub unsafe fn xrstor(buffer: *const u8) {
    core::arch::asm!(
        "xrstor [{}]",
        in(reg) buffer,
        in("eax") u32::MAX,
        in("edx") u32::MAX
    );
}

/// Read the Time Stamp Counter (TSC) from the CPU. The TSC is a 64-bit
/// register that counts the number of cycles since the CPU was last reset.
#[inline]
#[must_use]
pub fn rdtsc() -> u64 {
    let low: u32;
    let high: u32;
    // SAFETY: This is safe because the rdtsc instruction always exists on
    // x86_64 and should not cause any side effects that could lead to memory
    // unsafety or UB.
    unsafe {
        core::arch::asm!(
            "rdtsc",
            out("eax") low,
            out("edx") high
        );
    }
    u64::from(high) << 32 | u64::from(low)
}
