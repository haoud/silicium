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
#pragma once
#include <kernel.h>
#include <arch/x86/gdt.h>

#define IDT_MAX_ENTRY 256

#define IDT_TRAP_GATE_16 0x07
#define IDT_INTERRUPT_GATE_16 0x06
#define IDT_TRAP_GATE_32 0x0F
#define IDT_TASK_GATE_32 0x05
#define IDT_INTERRUPT_GATE_32 0x0E

#define set_trap_gate(i, handler) \
    idt_install_handler(i, (uint32_t) (handler), \
        GDT_KCODE_SELECTOR, 0, IDT_TRAP_GATE_32, 1)

#define set_system_gate(i, handler) \
    idt_install_handler(i, (uint32_t) (handler), \
        GDT_KCODE_SELECTOR, 3, IDT_INTERRUPT_GATE_32, 1)

#define set_interrupt_gate(i, handler) \
    idt_install_handler(i, (uint32_t) (handler), \
        GDT_KCODE_SELECTOR, 0, IDT_INTERRUPT_GATE_32, 1)

typedef struct idt_entry {
    uint16_t offset0_15;
    uint16_t selector;
    uint8_t reserved;
    uint8_t flags;
    uint16_t offset16_31;
} _packed idt_entry_t;

typedef struct idt_register {
    uint16_t size;
    uint32_t base;
} _packed idt_register_t;

_init void idt_install(void);
void idt_install_handler(
    const uint32_t offset,
    const uint32_t handler,
    const uint16_t cs,
    const uint32_t dlp,
    const uint32_t type,
    const int present);
