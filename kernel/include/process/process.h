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
#include <lib/list.h>
#include <lib/spinlock.h>
#include <process/thread.h>

#define PROCESS_IDLE_PID    0
#define PROCESS_INIT_PID    1

typedef uint32_t umask_t;

typedef struct process {
    pid_t pid, sid, pgid;
    pid_t uid, gid;
    pid_t euid, egid;
    pid_t fsuid, fsgid;
    umask_t umask;

    struct process *parent;
    struct mm_context *mm_context;

    struct spinlock spin;
    struct list_head node;
    struct list_head threads;
    struct list_head siblings;
    struct list_head children;
} process_t;

_noreturn void process_start(void);
_init void process_init(void);

process_t *process_allocate(void);
int process_creat(process_t *process);
int process_destroy(process_t *process);
void process_add_system_thread(thread_t *thread);
int process_clone(process_t *process, process_t *parent);

int process_abandoned(process_t *process);
int process_add_thread(process_t *process, thread_t *thread);
int process_remove_thread(process_t *process, thread_t *thread);
process_t *process_get_by_pid(const pid_t pid);
