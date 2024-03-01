use crate::{apic, cpu, gdt, idt, paging, percpu, simd, tss};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use macros::init;

/// The SMP request to Limine. This will order Limine to fetch information about
/// the APs and start them for us. We will only need to write their entry point
/// at a specific address and they will automatically jump to it. God bless the
/// authors of Limine.
static SMP_REQUEST: limine::request::SmpRequest = limine::request::SmpRequest::new();

/// The number of CPUs in the system
static CPU_COUNT: AtomicUsize = AtomicUsize::new(1);

/// A flag to check if the APs has been booted or not
static AP_BOOTED: AtomicBool = AtomicBool::new(false);

/// Setup the SMP environment and start the APs
///
/// # Panics
/// Panics if the SMP response from Limine is not received
#[inline]
pub fn setup() {
    // Get the response from Limine
    let response = SMP_REQUEST
        .get_response()
        .expect("No SMP response from Limine");

    // Start the APs
    response
        .cpus()
        .iter()
        .filter(|cpu| cpu.lapic_id != 0)
        .for_each(|cpu| {
            cpu.goto_address.write(ap_start);
        });

    // Wait for the APs to finish their setup
    while CPU_COUNT.load(Ordering::Relaxed) != response.cpus().len() {
        core::hint::spin_loop();
    }

    // The APs have been booted
    AP_BOOTED.store(true, Ordering::Relaxed);
}

/// Check if the APs have been booted or not. This is useful to check if the
/// system is ready for SMP operations such has sending IPIs. This function
/// will return true after all the APs have been booted and has finished their
/// setup (see [`ap_start`]).
#[must_use]
pub fn ap_booted() -> bool {
    AP_BOOTED.load(Ordering::Relaxed)
}

/// Get the number of active cores in the system.
#[must_use]
pub fn core_count() -> usize {
    CPU_COUNT.load(Ordering::Relaxed)
}

/// Get the current core id
#[must_use]
pub fn core_id() -> u64 {
    let id: u64;
    // SAFETY: This is safe because the gs points to the per-cpu data, and gs:8
    // contains the lapic_id of the current core
    unsafe {
        core::arch::asm!("mov {}, gs:8", out(reg) id);
    }
    id
}

/// The entry point for the APs.
#[init]
unsafe extern "C" fn ap_start(info: &limine::smp::Cpu) -> ! {
    percpu::setup(u64::from(info.lapic_id));
    idt::load();
    gdt::setup();
    tss::setup();
    simd::setup();
    apic::local::setup();
    paging::load_kernel_pml4();

    CPU_COUNT.fetch_add(1, Ordering::SeqCst);

    log::info!("AP {} correctly booted", info.lapic_id);
    cpu::halt();
}
