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
#include <arch/x86/gdt.h>

static struct gdt_register gdtr;
static struct gdt_entry gdt[GDT_MAX_ENTRY];

void gdt_install_desc(
    const uint32_t index,
    const uint32_t base,
    const uint32_t limit,
    const uint32_t access,
    const uint32_t flags,
    const int is_tss)
{
    assert(index < GDT_MAX_ENTRY);
    gdt[index].base0_15 = (base & 0xFFFF);
    gdt[index].base16_23 = ((base >> 16) & 0xFF);
    gdt[index].base24_31 = ((base >> 24) & 0xFF);
    gdt[index].limit0_15 = (limit & 0xFFFF);
    gdt[index].limit16_19 = ((limit >> 16) & 0x0F);
    gdt[index].flags = (flags & 0x0F);
    gdt[index].access = (is_tss) ? (access) : (access | 0x10);
}

_init void gdt_flush(void)
{
    gdtr.base = (uint32_t)gdt;
    gdtr.size = GDT_MAX_ENTRY * sizeof(gdt_entry_t);
    asm volatile("lgdt %0" ::"m"(gdtr));
    asm volatile(" mov ax, 0x10     \n\
                    mov ss, ax      \n\
                    mov ds, ax      \n\
                    mov es, ax      \n\
                    mov fs, ax      \n\
                    mov gs, ax      \n\
                    ljmp 0x08:1f    \n\
                    1:" ::
                     : "eax");
}

_init void gdt_install(void)
{
    gdt_install_desc(0, 0, 0, 0, 0, 0);
    gdt_install_desc(1, 0, 0xFFFFFFFF, GDT_IS_CODE_SEGMENT | GDT_SEGMENT_PRESENT | GDT_RING0,
                     GDT_BLOCK_SIZE_4_KO | GDT_SEGMENT_32BITS, 0);
    gdt_install_desc(2, 0, 0xFFFFFFFF, GDT_SEGMENT_PRESENT | GDT_DATA_CAN_WRITE | GDT_RING0,
                     GDT_BLOCK_SIZE_4_KO | GDT_SEGMENT_32BITS, 0);
    gdt_install_desc(3, 0, 0xFFFFFFFF, GDT_IS_CODE_SEGMENT | GDT_SEGMENT_PRESENT | GDT_RING3,
                     GDT_BLOCK_SIZE_4_KO | GDT_SEGMENT_32BITS, 0);
    gdt_install_desc(4, 0, 0xFFFFFFFF, GDT_SEGMENT_PRESENT | GDT_DATA_CAN_WRITE | GDT_RING3,
                     GDT_BLOCK_SIZE_4_KO | GDT_SEGMENT_32BITS, 0);
    gdt_flush();
}