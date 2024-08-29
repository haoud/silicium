use crate::arch::{
    self,
    irq::IrqNumber,
    x86_64::{io::Port, pic::IRQ_BASE},
};
use core::{
    pin::Pin,
    task::{Context, Poll, Waker},
};
use crossbeam::queue::SegQueue;
use futures::Stream;

#[allow(dead_code)]
static COMMAND: Port<u8> = Port::new(0x64);
static DATA: Port<u8> = Port::new(0x60);

/// A list of wakers waiting for a scancode to be available.
static WAITING: spin::Mutex<Vec<Waker>> = spin::Mutex::new(Vec::new());

/// The keyboard scancode stream.
pub static QUEUE: SegQueue<u8> = SegQueue::new();

pub const VECTOR: u8 = IRQ_BASE + IRQ.0;
pub const IRQ: IrqNumber = IrqNumber(1);

/// Setup the keyboard driver and enable the keyboard IRQ.
pub fn setup() {
    arch::irq::register(IRQ, handle_irq, "i8042 PS/2 Keyboard");
    // SAFETY: Enable the keyboard IRQ is safe and should not cause
    // any memory unsafety or undefined behavior.
    unsafe {
        arch::x86_64::apic::io::enable_irq(VECTOR);
    }
}

/// Handle the keyboard IRQ. It reads the scancode from the keyboard and pushes
/// it to the stream. If the stream contains more than 64 scancodes, the
/// current scancode is dropped and a warning is logged to the console.
pub fn handle_irq() {
    // TODO: Verify if the keyboard is ready to send a scancode. If not,
    // this is probably a spurious interrupt and we should ignore it.
    let scancode = unsafe { DATA.read() };
    if QUEUE.len() > 64 {
        log::warn!(
            "keyboard buffer full, dropping scancode: {:#02x}",
            scancode
        );
    } else {
        QUEUE.push(scancode);
    }

    // Wake up all wakers waiting for a scancode and remove them
    // from the waiting list.
    for waker in WAITING.lock().drain(..) {
        waker.wake_by_ref();
    }
}

/// The keyboard scancode stream. It is used to receive scancodes from the
/// keyboard driver. All instance of this struct share the same stream, so
/// calling `next` on one instance will consume the scancode for all instances.
/// This can lead to unexpected behavior if not handled correctly. In most
/// cases, you should only use one instance of this struct across the entire
/// kernel.
#[derive(Default, Debug)]
pub struct KeyboardScancodeStream {}

impl KeyboardScancodeStream {
    /// Create a new keyboard stream.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Stream for KeyboardScancodeStream {
    type Item = u8;

    /// Poll the next scancode from the stream and return it if available. If
    /// no scancode is available, the function will add the current task to the
    /// waiting list and return `Poll::Pending`. The task will be woken up when
    /// a scancode will be available.
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Self::Item>> {
        if let Some(scancode) = QUEUE.pop() {
            Poll::Ready(Some(scancode))
        } else {
            arch::irq::without(|| {
                WAITING.lock().push(cx.waker().clone());
            });
            Poll::Pending
        }
    }
}
