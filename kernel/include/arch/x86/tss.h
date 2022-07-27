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

#define TSS_GDT_ENTRY       5
#define TSS_GDT_SELECTOR    (TSS_GDT_ENTRY * 8)

typedef struct tss {
    uint16_t __link, link;
    uint32_t esp0;
    uint16_t ss0, __ss0;
    uint32_t esp1;
    uint16_t ss1, __ss1;
    uint32_t esp2;
    uint16_t ss2, __ss2;

    uint32_t cr3;
    uint32_t eip;
    uint32_t eflags;
    uint32_t eax;
    uint32_t ecx;
    uint32_t edx;
    uint32_t ebx;
    uint32_t esp;
    uint32_t ebp;
    uint32_t esi;
    uint32_t edi;

    uint16_t es, __es;
    uint16_t cs, __cs;
    uint16_t ss, __ss;
    uint16_t ds, __ds;
    uint16_t fs, __fs;
    uint16_t gs, __gs;
    uint16_t ldt, __ldt;
    uint32_t debug, io_map;
} tss_t;

_init void tss_install(void);
tss_t *tss_get_current(void);