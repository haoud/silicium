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
#include <core/preempt.h>

// TODO: Make this variable per-cpu
static unsigned int preemt_count = 0;

/**
 * @brief Enable preemption on the current CPU. This function use
 * a per-cpu counter to check if the preemption is disabled so the 
 * preemption remains disabled until the counter is equal to 0: don't
 * assume that the preemption is enabled right after the call to this
 * function.
 */
void preempt_enable(void)
{
    assert(preemt_count);
    preemt_count--;
}

/**
 * @brief Disable the preemption on the current CPU. This function use
 * a per-cpu counter to check if the preemption is disabled so it is safe
 * to call this function several times : The preemption remains disabled until
 * the counter is equal to 0.
 */
void preempt_disable(void)
{
    preemt_count++;
}

/**
 * @brief Check if the preemption is enabled on the current CPU
 * 
 * @return true If the preemption is enabled
 * @return false If the preemption is disabled
 */
bool preempt_enabled(void)
{
    return !preemt_count;
}
