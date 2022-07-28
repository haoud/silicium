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
#include <kernel.h>

#define DECLARE_SPINLOCK(name) \
    spinlock_t name = {0}

#define __spin_lock(spin) ({ \
    spin_lock(spin);         \
    spin;                    \
})

// Please use bracket when using this macro for better readability
#define spin_acquire(spin)                                                  \
    for (spinlock_t *__spin _cleanup(__spin_unlock) = (__spin_lock(spin)),  \
                             *__i = (spin);                                 \
         __i == (spin);                                                     \
         __i++)

typedef struct spinlock {
    atomic_t lock;
} spinlock_t;

void spin_init(spinlock_t *const spin);
void spin_lock(spinlock_t *const spin);
void spin_unlock(spinlock_t *const spin);
int spin_trylock(spinlock_t *const spin);

static inline void __spin_unlock(spinlock_t *const *const spin)
{
    spin_unlock(*spin);
}
