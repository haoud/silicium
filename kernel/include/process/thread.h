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
#include <lib/list.h>
#include <mm/context.h>
#include <arch/x86/cpu.h>
#include <arch/x86/fpu.h>

#define THREAD_IDLE_TID 0

#define THREAD_KERNEL   0
#define THREAD_USER     1

#define THREAD_CREATED          0
#define THREAD_READY            1
#define THREAD_RUNNING          2
#define THREAD_STOPPED          3
#define THREAD_SLEEPING         4
#define THREAD_DEEP_SLEEPING    5
#define THREAD_UNRUNNABLE       7
#define THREAD_ZOMBIE           6

#define THREAD_MAX      (PID_MAX - 1)

#define THREAD_STACK_TOP    0xBFFFF000
#define THREAD_STACK_SIZE   8192
#define THREAD_STACK_BASE   (THREAD_STACK_TOP - THREAD_STACK_SIZE)

typedef struct kstack {
    vaddr_t base;
    vaddr_t top;
    size_t size;
} kstack_t;

typedef struct thread {
    int exit_code;
    int quantum;
    int state;
    int type;

    pid_t tid;

    int fpu_used : 1;
    int fpu_loaded : 1;
    int reschedule : 1;

    struct kstack kstack;
    struct fpu_state *fpu_state;
    struct cpu_state *cpu_state;
    struct mm_context *mm_context;
    struct mm_context *mm_context_borrowed;

    struct list_head thread_node;
    struct list_head scheduler_node;
} thread_t;

thread_t *thread_allocate(void);
int thread_kernel_creat(thread_t *thread);
int thread_user_creat(thread_t *thread);
int thread_clone(thread_t *clone,
    const thread_t *thread,
    const cpu_state_t *cpu_state);

void thread_set_entry(thread_t *thread, const vaddr_t entry);
void thread_zombify(thread_t *thread, const int code);
void thread_destroy(thread_t *thread);
thread_t *thread_get_by_tid(const pid_t tid);