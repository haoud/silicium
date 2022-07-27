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
#include <lib/memory.h>
#include <arch/x86/gdt.h>
#include <arch/x86/tss.h>

static struct tss tss;

_init
void tss_install(void)
{
    memzero(&tss, sizeof(tss));
    gdt_install_desc(TSS_GDT_ENTRY, (uint32_t) &tss, sizeof(tss_t),
        GDT_SEGMENT_PRESENT | GDT_ACCESSED | GDT_IS_CODE_SEGMENT,
        GDT_SEGMENT_32BITS,
        true);

    tss.ss0 = GDT_KDATA_SELECTOR;
    tss.iomap = sizeof(tss);
    
    asm volatile("ltr ax" :: "a"(TSS_GDT_SELECTOR) : "memory");
}

tss_t *tss_get_current(void)
{
    return &tss;
}
