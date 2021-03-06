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
#include <process/thread.h>

#define SCHEDULER_DEFAULT_QUANTUM   25

_init void scheduler_set_current(thread_t *thread);

_no_inline void schedule(cpu_state_t *state);

void schedule_tick(void);
void scheduler_run(thread_t *thread, const bool save);

int scheduler_add_thread(thread_t *thread);
int scheduler_remove_thread(thread_t *thread);
thread_t *scheduler_get_current_thread(void);
