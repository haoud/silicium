pub use crate::arch::x86_64::irq::*;
use crate::arch::x86_64::pic::{IRQ_BASE, IRQ_COUNT};

/// An interrupt vector. This is a number that represents an interrupt, and is
/// used to index into the IDT when an interrupt occurs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct InterruptVector(pub u8);

impl InterruptVector {
    /// Create a new interrupt vector from a number.
    #[must_use]
    pub const fn new(number: u8) -> Self {
        Self(number)
    }
}

impl From<IrqNumber> for InterruptVector {
    fn from(irq: IrqNumber) -> Self {
        Self(irq.0 + IRQ_BASE)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IrqNumber(pub u8);

impl From<InterruptVector> for IrqNumber {
    fn from(vector: InterruptVector) -> Self {
        Self(vector.0 - IRQ_BASE)
    }
}

/// A trait for interrupt handlers. This trait is implemented for closures that
/// take no arguments and return nothing, but you can also implement it for your
/// own types if you want to.
pub trait IrqHandler: Send + Sync + 'static {
    /// Handle the interrupt. This function is called when the interrupt occurs
    /// and should do whatever is necessary to handle the interrupt.
    ///
    /// # Requirements
    /// - The is called with interrupts disabled, and must not enable them.
    /// - The handler must be as fast as possible and non-blocking.
    /// - The handler must correctly handle any spurious interrupts because it
    ///   is possible for the hardware to send an interrupt signal that is not
    ///   related to the interrupt vector that was registered (for example, if
    ///   the hardware sends an interrupt signal for a different device that
    ///   shares the same interrupt line).
    ///
    /// # Guarantees
    /// - Interrupts are guaranteed to be disabled when this function is called.
    fn handle(&mut self);
}

impl<T: Fn() + Send + Sync + 'static> IrqHandler for T {
    fn handle(&mut self) {
        self();
    }
}

/// A registered interrupt handler.
struct RegisteredHandler<'a> {
    /// The interrupt handler.
    handler: Box<dyn IrqHandler>,

    /// The name of the handler. This is used to identify the handler, and must
    /// be unique for each interrupt vector.
    name: &'a str,
}

/// A list of registered interrupt handlers for each interrupt vector.
static IRQ_HANDLERS: spin::Mutex<[Vec<RegisteredHandler>; IRQ_COUNT]> =
    spin::Mutex::new([const { Vec::new() }; IRQ_COUNT]);

/// Register an interrupt handler for a specific interrupt vector. The handler
/// will be called whenever the interrupt occurs. The name is used to identify
/// the handler, and must be unique for each interrupt vector. If a handler with
/// the same name is already registered for the same interrupt vector, this
/// function will print a warning in the log and will not register the handler.
pub fn register(irq: IrqNumber, handler: impl IrqHandler, name: &'static str) {
    let mut handlers = IRQ_HANDLERS.lock();
    let handlers = &mut handlers[irq.0 as usize];

    if handlers.iter().any(|h| h.name == name) {
        log::warn!(
            "Handler with name '{}' already registered for IRQ {}: ignoring...",
            name,
            irq.0
        );
        return;
    }

    handlers.push(RegisteredHandler {
        handler: Box::new(handler),
        name,
    });
}

/// Unregister an interrupt handler by its name and the interrupt vector it
/// is registered for. If the handler is not found, this function does nothing,
/// but prints a warning in the log.
pub fn unregister(irq: IrqNumber, name: &str) {
    let mut handlers = IRQ_HANDLERS.lock();
    let handlers = &mut handlers[irq.0 as usize];

    if let Some(index) = handlers.iter().position(|h| h.name == name) {
        handlers.swap_remove(index);
    } else {
        log::warn!("No handler with name '{}' found for IRQ {}", name, irq.0);
    }
}

/// Handle an IRQ by calling all registered handlers for that IRQ. This function
/// is called by the interrupt handler in the IDT, and should not be called
/// directly.
///
/// This function will print a warning in the log if no handler is registered
/// for the given IRQ. This may indicate a bug in the code, or an hardware
/// failure.
pub fn handle(irq: IrqNumber) {
    let mut handlers = IRQ_HANDLERS.lock();
    let handlers = &mut handlers[irq.0 as usize];

    if handlers.is_empty() {
        // TODO: Stop printing this warning if this is reached too often.
        log::warn!("No handler registered for IRQ {}", irq.0);
    } else {
        for irq in handlers {
            irq.handler.handle();
        }
    }
}
