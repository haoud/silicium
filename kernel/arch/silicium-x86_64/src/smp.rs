use crate::{cpu, gdt, idt, paging, percpu, tss};
use macros::init;

/// The SMP request to Limine. This will order Limine to fetch information about
/// the APs and start them for us. We will only need to write their entry point
/// at a specific address and they will automatically jump to it. God bless the
/// authors of Limine.
static SMP_REQUEST: limine::request::SmpRequest = limine::request::SmpRequest::new();

/// Setup the SMP environment and start the APs
///
/// # Panics
/// Panics if the SMP response from Limine is not received
#[inline]
pub fn setup() {
    let response = SMP_REQUEST
        .get_response()
        .expect("No SMP response from Limine");

    response
        .cpus()
        .iter()
        .filter(|cpu| cpu.lapic_id != 0)
        .for_each(|cpu| {
            cpu.goto_address.write(ap_start);
        });
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
    paging::load_kernel_pml4();

    log::info!("AP {} correctly booted", info.lapic_id);
    cpu::halt();
}
