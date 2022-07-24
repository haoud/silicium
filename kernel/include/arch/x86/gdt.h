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

#define GDT_MAX_ENTRY 10

#define GDT_KCODE_SELECTOR 0x08
#define GDT_KDATA_SELECTOR 0x10
#define GDT_KSTACK_SELECTOR 0x18

#define GDT_UCODE_SELECTOR 0x20
#define GDT_UDATA_SELECTOR 0x28
#define GDT_USTACK_SELECTOR 0x30

#define GDT_UCODE_SELECTOR_R3 (GDT_UCODE_SELECTOR + 3)
#define GDT_UDATA_SELECTOR_R3 (GDT_UDATA_SELECTOR + 3)
#define GDT_USTACK_SELECTOR_R3 (GDT_USTACK_SELECTOR + 3)

#define GDT_ACCESSED 0x01
#define GDT_CODE_CAN_READ 0x02
#define GDT_DATA_CAN_WRITE 0x02
#define GDT_DATA_GROW_DOWN 0x04
#define GDT_CODE_CONFORMING 0x04
#define GDT_IS_CODE_SEGMENT 0x08
#define GDT_RING0 0x00
#define GDT_RING1 0x20
#define GDT_RING2 0x40
#define GDT_RING3 0x60
#define GDT_SEGMENT_PRESENT 0x80

#define GDT_SEGMENT_32BITS 0x04
#define GDT_BLOCK_SIZE_4_KO 0x08

typedef struct gdt_register {
    uint16_t size;
    uint32_t base;
} _packed gdt_register_t;

typedef struct gdt_entry {
    uint16_t limit0_15;
    uint16_t base0_15;
    uint8_t base16_23;
    uint8_t access;
    uint8_t limit16_19 : 4;
    uint8_t flags : 4;
    uint8_t base24_31;
} _packed gdt_entry_t;

_init void gdt_install(void);
void gdt_install_desc(
    const uint32_t index,
    const uint32_t base,
    const uint32_t limit,
    const uint32_t access,
    const uint32_t flags,
    const bool is_tss);