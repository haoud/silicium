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
#include <lib/list.h>
#include <lib/spinlock.h>
#include <arch/x86/fpu.h>
#include <arch/x86/tss.h>
#include <process/schedule.h>

static DECLARE_SPINLOCK(run_queue_lock);
static DECLARE_LIST(run_queue);

static thread_t *current = NULL;

/**
 * @brief The file contains the scheduler implementation. For now, it is
 * a simple round robin scheduler with awful performance: IO thread can
 * be starved if the CPU is very busy, and the algorithm is not scalable
 * because we use a lock to avoid race conditions.
 * 
 * It is largely suffisant for now, but it should be improved in the future.
 */

/**
 * @brief Redisitrubte a quantum for all thread. This function does not lock
 * the run queue, it is to the caller to do so.
 */
static void scheduler_redistribute(void)
{
    list_foreach(&run_queue, entry) {
        thread_t *thread = list_entry(entry, thread_t, scheduler_node);
        if (thread->tid != THREAD_IDLE_TID)
            thread->quantum = SCHEDULER_DEFAULT_QUANTUM;
    }
}

/**
 * @brief 
 * 
 * @return thread_t* The next thread to run: cannot be NULL.
 */
static thread_t* schedule_next(void)
{
    spin_acquire(&run_queue_lock) {
        list_foreach(&run_queue, entry) {
            thread_t *thread = list_entry(entry, thread_t, scheduler_node);
            if (thread->quantum > 0 && thread->state == THREAD_READY)
                return thread;
        }

        scheduler_redistribute();
        list_foreach(&run_queue, entry) {
            thread_t *thread = list_entry(entry, thread_t, scheduler_node);
            if (thread->quantum > 0 && thread->state == THREAD_READY)
                return thread;
        }

        // No threads ready to run, return the idle thread
        list_foreach(&run_queue, entry) {
            thread_t *thread = list_entry(entry, thread_t, scheduler_node);
            if (thread->tid == THREAD_IDLE_TID)
                return thread;
        }
    }
    _unreachable();
}

void schedule(cpu_state_t *state)
{
    thread_t *next = schedule_next();
    if (current == NULL || current == next)
        return;
    
    set_task_switched();
    if (current->state == THREAD_RUNNING)
        current->state = THREAD_READY;
    if (current->fpu_loaded) {
        fpu_save(current->fpu_state);
        current->fpu_loaded = false;
    }
    
    // If the next thread does not have a MM context (ie kernel thread), we 
    // borrow the current one.
    if (next->mm_context == NULL) {
        if (current->mm_context == NULL) {
            next->mm_context_borrowed = current->mm_context_borrowed;
        } else {
            next->mm_context_borrowed = current->mm_context;
        }
        mm_context_set(next->mm_context_borrowed);
    } else {
        mm_context_use(next->mm_context);
        mm_context_set(next->mm_context);
        mm_context_drop(current->mm_context);
    }
    
    current->reschedule = false;
    current->cpu_state = state;

    next->state = THREAD_RUNNING;
    if (next->type == THREAD_USER)
        tss_get_current()->esp0 = next->kstack.top;

    current = next;
    switch_to(next->cpu_state);
}

/**
 * @brief 
 * 
 * @return int 
 */
int schedule_tick(void)
{
    if (current->tid == THREAD_IDLE_TID) {
        current->reschedule = true;
        return 0;
    }
    
    current->quantum--;
    if (current->quantum == 0)
        current->reschedule = true;
    return 0;
}

/**
 * @brief 
 * 
 * @param thread 
 * @return int 
 */
int scheduler_add_thread(thread_t *thread)
{
    assert(list_empty(&thread->scheduler_node));
    thread->quantum = SCHEDULER_DEFAULT_QUANTUM;
    spin_acquire(&run_queue_lock) {
        list_add_tail(&run_queue, &thread->scheduler_node);
    }
    return 0;
}

/**
 * @brief 
 * 
 * @param thread 
 * @return int 
 */
int scheduler_remove_thread(thread_t *thread)
{
    assert(!list_empty(&thread->scheduler_node));
    spin_acquire(&run_queue_lock) {
        list_remove(&thread->scheduler_node);
    }
    return 0;
}