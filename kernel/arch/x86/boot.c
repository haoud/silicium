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
#include <multiboot.h>
#include <core/mm/page.h>
#include <core/mm/slub.h>
#include <core/mm/malloc.h>
#include <core/mm/vmalloc.h>
#include <arch/x86/gdt.h>
#include <arch/x86/idt.h>
#include <arch/x86/paging.h>
#include <arch/x86/exception.h>

extern void startup(void);

void start(struct mb_info *info)
{
    gdt_install();
    idt_install();
    exception_install();
    page_setup(info);
    paging_remap_kernel();
    page_map_table();
    slub_setup();
    kmalloc_setup();
    vmalloc_setup();
    startup();
}
