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
#include <kernel.h>
#include <mm/page.h>
#include <mm/malloc.h>
#include <core/ustar.h>
#include <core/module.h>
#include <arch/x86/cpu.h>

extern const char _init_start;
extern const char _init_end;

_init void load_module(char *initrd, char *name)
{
    ustar_entry_t *module = ustar_lookup(initrd, name);
    if (module == NULL)
        error("Failed to find module %s", name);
    if (module_load(module->data, module->length) < 0)
        warn("Failed to load module %s", name);
}

_init void load_modules(char *initrd)
{
    // TODO: Use a config file to load modules and to configure the kernel 
    load_module(initrd, "test.kmd");
    module_unload("test");
    free(initrd);
}

_init _noreturn void free_init_sections(void)
{
    // Here we free the physical pages used only for the initialization
    // of the kernel. However, we do not unmap them because this will 
    // complicate the code of this function. So for a short time the
    // kernel will use pages marked as free. This is why we must take
    // precautions: the other processors must not allocate pages before
    // this function is completely finished.
    // On a uniprocessor computer this should not be a problem
    for (vaddr_t addr = (vaddr_t) &_init_start;
        addr < (vaddr_t) &_init_end;
        addr += PAGE_SIZE) {
        page_free(addr - KERNEL_BASE);
    }

    info("Boot completed !");
    cpu_stop();
}

_init _noreturn void startup(char *initrd)
{
    load_modules(initrd);
    free_init_sections();
}
