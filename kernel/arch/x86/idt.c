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
#include <assert.h>
#include <arch/x86/idt.h>

static struct idt_entry idt[IDT_MAX_ENTRY];
static struct idt_register idtr;

void idt_install_handler(
    const uint32_t offset,
    const uint32_t handler,
    const uint16_t cs,
    const uint32_t dlp,
    const uint32_t type,
    const int present)
{
    assert(offset < IDT_MAX_ENTRY);
    assert(dlp <= 3);

    idt[offset].reserved = 0;
    idt[offset].selector = cs;
    idt[offset].offset0_15 = (handler & 0xFFFF);
    idt[offset].offset16_31 = ((handler >> 16) & 0xFFFF);
    idt[offset].flags = (dlp << 5) | (type) | ((present) ? 0x80 : 0x00);
}

void _init idt_flush(void)
{
    idtr.base = (uint32_t)&idt;
    idtr.size = IDT_MAX_ENTRY * sizeof(idt_entry_t);
    asm volatile("lidt %0" ::"m"(idtr));
}

void _init idt_install(void)
{
    extern void default_int(void);
    for (unsigned int i = 0; i < IDT_MAX_ENTRY; i++)
        set_interrupt_gate(i, &default_int);
    idt_flush();
}
