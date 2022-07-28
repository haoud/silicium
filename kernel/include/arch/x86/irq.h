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
#include <arch/x86/cpu.h>
#include <arch/x86/pic.h>

#define IRQ_MAX PIC_TOTAL_IRQ

/**
 * @brief Disable the interrupt during the scope of the for loop. After the
 * for loop, the interrupt will be restored to its previous state before the
 * macro was called.
 * TODO: Modify the interrupt flags only instead of restoring eflags entirely.
 * 
 * Usage:
 *  irq_acquire() {
 *    ...   // Insert code here
 *  }
 */
#define irq_acquire()                                                         \
    for (uint32_t __e _cleanup(__set_eflags) = get_eflags(), __i = __cli();   \
         __i == 0;                                                            \
         __i++)

typedef void (*irq_handler_t)(cpu_state_t *);

_init void irq_install(void);
_export int irq_request(
    const unsigned int irq,
    const irq_handler_t handler,
    const char *name,
    const int flags);
