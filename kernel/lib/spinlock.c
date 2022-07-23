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
#include <lib/spinlock.h>

void spin_init(spinlock_t *const spin)
{
	spin->lock = 0;
}

void spin_lock(spinlock_t *const spin)
{
#ifdef CONFIG_SMP
	while (__sync_lock_test_and_set(&spin->lock, 1)) {
		while (spin->lock)
			__builtin_ia32_pause();
	}
#else
	spin->lock = get_eflags() & EFLAGS_IF;
	cli();
#endif
}

void spin_unlock(spinlock_t *const spin)
{
#ifdef CONFIG_SMP
	__sync_lock_release(&spin->lock);
#else
	if (spin->lock) {
		sti();
	}
#endif
}

int spin_trylock(spinlock_t *const spin)
{
#ifdef CONFIG_SMP
	if (__sync_lock_test_and_set(&spin->lock, 1)) {
		return 0;
	}
#else
	spin_lock(spin);
#endif
	return 1;
}