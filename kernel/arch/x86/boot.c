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
#include <lib/string.h>
#include <lib/memory.h>
#include <core/symbol.h>
#include <mm/page.h>
#include <mm/slub.h>
#include <mm/malloc.h>
#include <mm/vmalloc.h>
#include <arch/x86/fpu.h>
#include <arch/x86/gdt.h>
#include <arch/x86/idt.h>
#include <arch/x86/pic.h>
#include <arch/x86/pit.h>
#include <arch/x86/paging.h>
#include <arch/x86/exception.h>

extern void startup(const char *initrd);

_init void start(struct mb_info *info)
{
    pic_remap();
    gdt_install();
    idt_install();
    exception_install();
    fpu_setup();
    pit_configure();
    page_setup(info);
    paging_remap_kernel();
    page_map_table();
    slub_setup();
    vmalloc_setup();
    kmalloc_setup();
    symbol_init(info);

    // Find the initrd inside the multiboot info structure module
    struct mb_module *module = mb_get_module(info, "initrd");

    // Allocate the initrd memory and copy it to the kernel memory
    const char *initrd = NULL;
    if (module != NULL) {
        const size_t length = module->mod_end - module->mod_start;
        initrd = malloc(length);
        if (initrd == NULL)
            panic("Failed to allocate memory for initrd");
        memcpy(initrd, module->mod_start, length);
    } else {
        warn("No initrd found");
    }

    paging_clear_userspace();
    startup(initrd);
}
