/**
 * Copyright (C) 2022 Romain CADILHAC
 *
 * This file is part of Silicium.
 *
 * Silicium is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Silicium is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Silicium. If not, see <http://www.gnu.org/licenses/>.
 */
#include <arch/x86/io.h>
#include <arch/x86/pic.h>

void pic_remap(void)
{
    //TODO: Spurious interrupt handling
    outb(PIC_MASTER_CMD, PIC_ICW1_NEED_ICW4 | PIC_ICW1_INIT_REQUIRED);
    outb(PIC_SLAVE_CMD, PIC_ICW1_NEED_ICW4 | PIC_ICW1_INIT_REQUIRED);
    outb(PIC_MASTER_DATA, IRQ_BASE);
    outb(PIC_SLAVE_DATA, IRQ_BASE + IRQ_PER_PIC);
    outb(PIC_MASTER_DATA, 4);
    outb(PIC_SLAVE_DATA, 2);
    outb(PIC_MASTER_DATA, PIC_ICW4_8086);
    outb(PIC_SLAVE_DATA, PIC_ICW4_8086);
}

void pic_send_eoi(const uint32_t irq)
{
    assert(irq < PIC_TOTAL_IRQ);
    if (irq >= IRQ_PER_PIC)
        outb(PIC_SLAVE_CMD, PIC_EOI);
    outb(PIC_MASTER_CMD, PIC_EOI);
}

void pic_enable(const uint32_t irq)
{
    assert(irq < PIC_TOTAL_IRQ);
    const uint8_t pic = (irq >= IRQ_PER_PIC) ? PIC_SLAVE_DATA : PIC_MASTER_DATA; 
    const uint8_t mask = inb(pic);
    outb(pic, mask & ~(1 << (irq % 8)));
}

void pic_disable(const uint32_t irq)
{
    assert(irq < PIC_TOTAL_IRQ);
    const uint8_t pic = (irq >= IRQ_PER_PIC) ? PIC_SLAVE_DATA : PIC_MASTER_DATA; 
    const uint8_t mask = inb(pic);
    outb(pic, mask | (1 << (irq % 8)));
}

void pic_enable_all(void)
{
    outb(PIC_MASTER_DATA, 0x00);
    outb(PIC_SLAVE_DATA, 0x00);
}

void pic_disable_all(void)
{
    outb(PIC_MASTER_DATA, 0xFF);
    outb(PIC_SLAVE_DATA, 0xFF);
}
