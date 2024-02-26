use crate::gdt;

/// Enable interrupts on the current CPU core, allowing the current workflow to be interrupted by
/// hardware interrupts. If interrupts are already enabled, this function will have no effect.
///
/// # Safety
/// This function is unsafe because this function make some assumptions about the state of the
/// system. The caller must ensure that the current CPU core is in a state where it is safe to
/// enable interrupts, and that the core can handle the interrupts that will be generated without
/// triple faulting.
/// Failing to meet these requirements can result in undefined behavior.
#[inline]
pub unsafe fn sti() {
    core::arch::asm!("sti");
}

/// Disable interrupts on the current CPU core. This will prevent the CPU from receiving any
/// interrupts until they are re-enabled. If the same interrupt is received multiple times while
/// interrupts are disabled, it will only be handled once interrupts are re-enabled.
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

/// Halt the CPU until the next interrupt is received. If interrupts are disabled, this will
/// effectively halt the CPU indefinitely.
#[inline]
pub fn hlt() {
    // SAFETY: This is safe because waiting for an interrupt should not break
    // Rust's safety guarantees. If interrupts are disabled, this will effectively
    // halt the CPU indefinitely, but again, this is not a memory safety issue.
    unsafe {
        core::arch::asm!("hlt");
    }
}

/// Write an 8 bit value from a port.
///
/// # Safety
/// This function is unsafe because writing to a port can have side effects, including causing
/// the hardware to do something unexpected and possibly violating memory safety.
#[inline]
pub unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") value);
}

/// Write an 16 bit value to a port.
///
/// # Safety
/// This function is unsafe because writing to a port can have side effects, including causing
/// the hardware to do something unexpected and possibly violating memory safety.
#[inline]
pub unsafe fn outw(port: u16, value: u16) {
    core::arch::asm!("out dx, ax", in("dx") port, in("ax") value);
}

/// Write an 32 bit value to a port.
///
/// # Safety
/// This function is unsafe because writing to a port can have side effects, including causing
/// the hardware to do something unexpected and possibly violating memory safety.
#[inline]
pub unsafe fn outd(port: u16, value: u32) {
    core::arch::asm!("out dx, eax", in("dx") port, in("eax") value);
}

/// Read an 8 bit value from a port.
///
/// # Safety
/// This function is unsafe because reading from a port can have side effects, including causing
/// the hardware to do something unexpected and possibly violating memory safety.
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
/// This function is unsafe because reading from a port can have side effects, including causing
/// the hardware to do something unexpected and possibly violating memory safety.
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
/// This function is unsafe because reading from a port can have side effects, including causing
/// the hardware to do something unexpected and possibly violating memory safety.
#[inline]
#[must_use]
pub unsafe fn ind(port: u16) -> u32 {
    let mut value: u32;
    core::arch::asm!("in eax, dx", in("dx") port, out("eax") value);
    value
}

/// Load the Global Descriptor Table (GDT) register with the provided GDT register value.
///
/// # Safety
/// The caller must ensure that the provided GDT reigster value is valid and reference a
/// valid GDT that must stay in memory while it is loaded into the GDT register.Failing
/// to meet these requirements can result in undefined behavior, memory unsafety or crashes.
///
/// However, the GDT register structure can be dropped as soon as the function returns
/// because the CPU will keep a copy of the GDT register in its internal state.
#[inline]
pub unsafe fn lgdt(gdtr: &gdt::Register) {
    core::arch::asm!("lgdt [{}]", in(reg) gdtr);
}
