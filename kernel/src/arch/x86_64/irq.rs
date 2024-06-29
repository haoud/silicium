use super::{apic, paging};
use crate::{
    arch::x86_64::{
        cpu::{self, rflags::Flags},
        opcode,
    },
    time,
};

/// The state of the interrupts.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct State(bool);

/// Enable interrupts on the current core.
///
/// # Safety
/// This function is unsafe because enabling interrupts, contrary to disabling
/// them, can lead to unexpected behavior, memory unsafety, data races and
/// other issues if not used correctly.
/// To correctly use this function, it is required to ensure that the caller
/// has correctly set up the interrupt. Then, the caller must ensure that the
/// portion of code that is executed with interrupts enabled is safe to be
/// executed concurrently with other code that may also have interrupts
///
/// Even if this doesn't lead to memory unsafety, it can still lead to
/// deadlocks if the code that is executed with interrupts enabled is not
/// reentrant and is called from a context where interrupts are disabled.
#[inline]
pub unsafe fn enable() {
    opcode::sti();
}

/// Disable interrupts on the current core.
#[inline]
pub fn disable() {
    opcode::cli();
}

/// Halt the CPU until the next interrupt occurs. If interrupts are disabled,
/// this will result in an infinite loop.
#[inline]
pub fn wait() {
    opcode::hlt();
}

/// Check if interrupts are enabled. Returns true if interrupts are enabled,
/// false otherwise.
#[inline]
#[must_use]
pub fn enabled() -> bool {
    cpu::rflags::read().contains(Flags::IF)
}

/// Save the current state of the interrupts and return it. This state can be
/// restored later using the `restore` function.
#[inline]
#[must_use]
pub fn save() -> State {
    State(enabled())
}

/// Save the current state of the interrupts, disable them and return the saved
/// state. This state can be restored later using the `restore` function.
#[inline]
#[must_use]
pub fn save_and_disable() -> State {
    let state = save();
    disable();
    state
}

/// Restore the previous state of the interrupts. If `enabled` is true, then
/// interrupts will be enabled, otherwise they will be disabled.
#[inline]
#[allow(clippy::needless_pass_by_value)]
pub fn restore(state: State) {
    if state.0 {
        // SAFETY: Enabling interrupts is safe in this contexte because they
        // were enabled before calling this function and we simply restore the
        // previous state. This is safe because the caller is responsible for
        // managing its own code, and are not our problem here. If the code was
        // unsound before calling this function, we can't do anything about it.
        unsafe {
            enable();
        }
    } else {
        disable();
    }
}

/// Execute a closure with interrupts disabled. After the closure is executed,
/// the previous state of the interrupts is restored.
pub fn without<T, F: FnOnce() -> T>(f: F) -> T {
    let enabled = enabled();
    disable();
    let object = f();
    if enabled {
        // SAFETY: Enabling interrupts is safe in this contexte because they
        // were enabled before calling this function and we simply restore the
        // previous state. This is safe because the caller is responsible for
        // managing its own code, and are not our problem here. If the code was
        // unsound before calling this function, we can't do anything about it.
        unsafe {
            enable();
        }
    }
    object
}

/// The interrupt handler. This function is called by the CPU when an interrupt
/// is triggered. It will call the appropriate interrupt handler for the given
/// interrupt.
#[no_mangle]
pub extern "C" fn irq_handler(irq: u8) {
    if apic::io::is_irq(irq) {
        apic::local::end_of_interrupt();
        if apic::local::timer::own_irq(irq) {
            apic::local::timer::handle_irq();
        }
    } else if paging::tlb::own_irq(irq) {
        paging::tlb::flush();
    } else {
        log::warn!("Unhandled interrupt: {:?}", irq);
    }

    time::timer::handle();
}
