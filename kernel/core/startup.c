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
#include <arch/x86/cpu.h>
#include <core/mm/page.h>

extern const char _init_start;
extern const char _init_end;

_init void free_init_sections(void)
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
    cpu_stop();
}

_init void startup()
{
    info("Boot completed !");
    free_init_sections();
}
