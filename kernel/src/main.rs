#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(panic_info_message)]
#![feature(const_mut_refs)]
#![feature(negative_impls)]
#![feature(new_uninit)]

extern crate alloc;

pub mod arch;
pub mod mm;
pub mod scheduler;

/// The entry point for the kernel. This function call the architecture specific setup
/// function, print a message to the console and then halts the CPU.
///
/// # Safety
/// This function is marked as unsafe because it must be called only once at the start
/// of the kernel. Failing to do so will result in undefined behavior.
#[no_mangle]
#[macros::init]
#[cfg(not(test))]
pub unsafe extern "C" fn _start() -> ! {
    // Call the architecture specific setup function
    let info = arch::setup();

    // Setup the memory management system
    mm::setup(&info);

    // Log that the kernel has successfully booted
    log::info!("Silicium booted successfully");

    // Create five kernel threads and add them to the scheduler
    let a = arch::thread::Thread::kernel(a);
    let b = arch::thread::Thread::kernel(b);
    let c = arch::thread::Thread::kernel(c);
    let d = arch::thread::Thread::kernel(d);
    let e = arch::thread::Thread::kernel(e);

    scheduler::SCHEDULER.enqueue_ready(scheduler::Task::new(a));
    scheduler::SCHEDULER.enqueue_ready(scheduler::Task::new(b));
    scheduler::SCHEDULER.enqueue_ready(scheduler::Task::new(c));
    scheduler::SCHEDULER.enqueue_ready(scheduler::Task::new(d));
    scheduler::SCHEDULER.enqueue_ready(scheduler::Task::new(e));
    scheduler::enter();
}

fn a() -> ! {
    loop {
        arch::irq::without(|| {
            arch::log::write("a");
        });
        arch::irq::wait();
    }
}

fn b() -> ! {
    loop {
        arch::irq::without(|| {
            arch::log::write("b");
        });
        arch::irq::wait();
    }
}

fn c() -> ! {
    loop {
        arch::irq::without(|| {
            arch::log::write("c");
        });
        arch::irq::wait();
    }
}

fn d() -> ! {
    loop {
        arch::irq::without(|| {
            arch::log::write("d");
        });
        arch::irq::wait();
    }
}

fn e() -> ! {
    loop {
        arch::irq::without(|| {
            arch::log::write("e");
        });
        arch::irq::wait();
    }
}
