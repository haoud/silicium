use crate::arch::x86_64::io::Port;
use macros::init;

static MASTER_PIC_CMD: Port<u8> = Port::new(0x20);
static MASTER_PIC_DATA: Port<u8> = Port::new(0x21);
static SLAVE_PIC_CMD: Port<u8> = Port::new(0xA0);
static SLAVE_PIC_DATA: Port<u8> = Port::new(0xA1);

/// The base IRQ number for the PICs. We remap the PICs IRQs right after the
/// CPU exceptions, which are the first 32 IRQs (0-31).
pub const IRQ_BASE: u8 = 32;

/// The number of IRQs that each PIC supports.
pub const IRQ_PER_PIC: u8 = 8;

/// The number of PICs in the system.
pub const PIC_COUNT: u8 = 2;

/// The number of IRQs supported by the PICs.
pub const IRQ_COUNT: usize = 16;

/// Remap the PICs from their default IRQs (0-15) to the given base IRQ. This
/// is necessary because the default IRQs conflict with the CPU exceptions,
/// which are the first 32 IRQs (0-31). The master PIC will use IRQs [base,
/// base + 7] and the slave PIC will use IRQs [base + 8, base + 15]. After
/// remapping, all interrupts are unmasked, but no interrupts will occur until
/// the interrupts are enabled with the `sti` instruction.
///
/// # Safety
/// This function is unsafe because it writes to the PICs with I/O ports, which
/// can cause undefined behavior if the PICs do not exist or are not in the
/// expected state. Furthermore, this function must only be called once and
/// only during the initialization of the kernel.
#[init]
pub unsafe fn remap_and_disable() {
    // ECW1: Cascade mode, ICW4 needed
    MASTER_PIC_CMD.write_and_pause(0x11);
    SLAVE_PIC_CMD.write_and_pause(0x11);

    // ICW2: Write the base IRQs for the PICs
    MASTER_PIC_DATA.write_and_pause(IRQ_BASE);
    SLAVE_PIC_DATA.write_and_pause(IRQ_BASE + 8);

    // ICW3: Connect the PICs to each other
    // The master PIC is connected to the slave PIC on IRQ2
    // The slave PIC is connected to the master PIC on IRQ4
    MASTER_PIC_DATA.write_and_pause(4);
    SLAVE_PIC_DATA.write_and_pause(2);

    // ICW4: Request 8086 mode
    MASTER_PIC_DATA.write_and_pause(0x01);
    SLAVE_PIC_DATA.write_and_pause(0x01);

    // OCW1: Disable all interrupts
    MASTER_PIC_DATA.write_and_pause(0xFF);
    SLAVE_PIC_DATA.write_and_pause(0xFF);
}
