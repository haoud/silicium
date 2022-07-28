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
#include <arch/x86/gdt.h>
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
 * @brief Find the next thread to run. The algorithm is simple: we try to
 * find a ready thread with a quantum left. If none is found, we redistribute
 * the quantum to all threads and we try again. And if we still don't find
 * any thread, we return the idle thread.
 * 
 * @return thread_t* The next thread to run: cannot be NULL. If there is no
 * thread to run, it returns the idle thread.
 */
static thread_t* schedule_next(void)
{
    spin_acquire(&run_queue_lock) {
        // Find the next thread to run
        list_foreach(&run_queue, entry) {
            thread_t *thread = list_entry(entry, thread_t, scheduler_node);
            if (thread->tid != THREAD_IDLE_TID &&
                thread->state == THREAD_READY && 
                thread->quantum > 0)
                return thread;
        }

        // All thread are blocked, we need to redistribute the quantum.
        scheduler_redistribute();
        list_foreach(&run_queue, entry) {
            thread_t *thread = list_entry(entry, thread_t, scheduler_node);
            if (thread->tid != THREAD_IDLE_TID &&
                thread->state == THREAD_READY && 
                thread->quantum > 0)
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

/**
 * @brief Do I really need to explain ?
 * 
 * @param thread The thread to add.
 */
_init void scheduler_set_current(thread_t *thread)
{
    current = thread;
}

/**
 * @brief Schedule the current thread. If a saved state is provided, it is
 * the state that will be restored when the thread will be resumed.
 * If the current thread is the idle thread and there is no thread to run, 
 * this function will return, so do not assume that the function will never
 * return.
 * 
 * @param state The saved state of the thread. If NULL, the current state
 * is saved in this function and the thread will be resumed to the caller
 * function (TODO).
 */
_no_inline
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
        if (current->mm_context == NULL)
            next->mm_context_borrowed = current->mm_context_borrowed;
        else
            next->mm_context_borrowed = current->mm_context;
        mm_context_set(next->mm_context_borrowed);
    } else {
        mm_context_use(next->mm_context);
        mm_context_set(next->mm_context);
        mm_context_drop(current->mm_context);
    }

    current->reschedule = false;
    current->cpu_state = state;
    scheduler_run(next, !state);
}

/**
 * @brief This function is called every tick. It is used to update the quantum
 * of the current thread. If the quantum is 0 or if it is the idle processus,
 * the reschedule flag is set.
 */
void schedule_tick(void)
{
    if (current->tid == THREAD_IDLE_TID) {
        current->reschedule = true;
    } else {
        current->quantum--;
        if (current->quantum == 0)
            current->reschedule = true;
    }
}

/**
 * @brief Run the given thread. This function is called by the scheduler
 * internally and to run the first thread during the boot.
 * 
 * @param thread The thread to run
 * @param save A flag to indicate if the current thread state must be saved.
 * If set, the current thread state is saved in the thread structure and will
 * resume to the caller function. If not, the thread state must be already
 * staved in the thread structure.
 */
void scheduler_run(thread_t *thread, const bool save)
{
    thread_t *prev = current;

    current = thread;
    current->state = THREAD_RUNNING;
    if (current->type == THREAD_USER)
        tss_get_current()->esp0 = current->kstack.top;

    if(save)
        save_switch_to(&prev->cpu_state, current->cpu_state);
    else
        switch_to(current->cpu_state);
}

/**
 * @brief Add a thread to the run queue and set the thread state to ready.
 * The thread added is given a quantum of SCHEDULER_DEFAULT_QUANTUM.
 * 
 * @param thread The thread to add.
 * @return int Always 0.
 */
int scheduler_add_thread(thread_t *thread)
{
    assert(list_empty(&thread->scheduler_node));
    thread->quantum = SCHEDULER_DEFAULT_QUANTUM;
    thread->state = THREAD_READY;
    spin_acquire(&run_queue_lock) {
        list_add_tail(&run_queue, &thread->scheduler_node);
    }
    return 0;
}

/**
 * @brief Remove a thread from the run queue and set its state to UNRUNNABLE.
 * 
 * @param thread The thread to remove.
 * @return int Always 0.
 */
int scheduler_remove_thread(thread_t *thread)
{
    assert(!list_empty(&thread->scheduler_node));
    spin_acquire(&run_queue_lock) {
        list_remove(&thread->scheduler_node);
    }
    thread->state = THREAD_UNRUNNABLE;
    return 0;
}

/**
 * @brief Return the current thread on the current CPU.
 * 
 * @return thread_t* The current thread: can be NULL during initialization.
 */
thread_t *scheduler_get_current_thread(void)
{
    return current;
}
