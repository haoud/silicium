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
#include <core/date.h>
#include <core/ustar.h>
#include <core/module.h>
#include <arch/x86/cpu.h>

#include <process/thread.h>
#include <process/schedule.h>

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

    info("Boot completed !");
}

static void idle(void)
{
    while (1)
        hlt();
}

static void tic(void)
{
    while (1) {
        info("tic");
        schedule(NULL);
    }
}

static void tac(void)
{
    while (1) {
        info("tac");
        schedule(NULL);
    }
}

_init _noreturn void startup(char *initrd)
{
    date_setup();
    load_modules(initrd);
    thread_t * thread0 = thread_allocate();
    thread_t * thread1 = thread_allocate();
    thread_t * thread2 = thread_allocate();
    thread_kernel_creat(thread0);
    thread_kernel_creat(thread1);
    thread_kernel_creat(thread2);
    thread0->mm_context_borrowed = mm_context_create();

    thread_set_entry(thread0, (vaddr_t) idle);
    thread_set_entry(thread1, (vaddr_t) tic);
    thread_set_entry(thread2, (vaddr_t) tac);
    scheduler_add_thread(thread0);
    scheduler_add_thread(thread1);
    scheduler_add_thread(thread2);

    free_init_sections();
    scheduler_run(thread0, false);
    _unreachable();
}
