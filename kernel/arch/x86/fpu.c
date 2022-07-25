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
#include <arch/x86/cpu.h>
#include <arch/x86/fpu.h>

_init void fpu_setup(void)
{
    // Assume SSE is available (Silicium cannot boot before P4 
    // because it use large page to setup the kernel)

    // Enable SSE and disable FPU emulation
    asm volatile(
        "mov eax, cr0   \n"
        "and eax, %0    \n"
        "or eax, %1     \n"
        "mov cr0, eax   \n"
         :: "i"(~CR0_COPROCESSOR_EMU), "i"(CR0_COPROCESSOR_MON) : "eax");
    asm volatile(
        "mov eax, cr4   \n"
        "or eax, %0     \n"
        "mov cr4, eax   \n"
         :: "i"(CR4_OSFXRS | CR4_OSMXMME): "eax");
    set_task_switched();
}

/**
 * @brief Initialize the current FPU state
 * 
 */
void fpu_init(void)
{
    asm volatile("finit");
}

/**
 * @brief Save the current FPU state. The FPU must be initialized before.
 * 
 * @param state The location where to save the FPU state: must be aligned 
 * on a 16 bytes boundary
 */
void fpu_save(fpu_state_t *state)
{
    asm volatile("fxsave [%0]" :: "r"(state->data) : "memory");
}

/**
 * @brief Restore the FPU state
 * 
 * @param state The FPU state to restore: must be already initialized
 * and aligned on a 16 bytes boundary.
 */
void fpu_restore(fpu_state_t *state)
{
    asm volatile("fxrstor [%0]" :: "r"(state->data));
}