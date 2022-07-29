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
#include <mm/malloc.h>
#include <mm/context.h>
#include <process/thread.h>
#include <process/process.h>
#include <process/schedule.h>

static struct process *system_process;
static struct thread *system_idle;

static DECLARE_SPINLOCK(list_lock);
static DECLARE_LIST(processes);

_noreturn
static void process_idle(void)
{
    for(;;)
        hlt();
}

_noreturn
void process_start(void)
{
    scheduler_run(system_idle, false);
    _unreachable();
}

_init
void process_init(void)
{
    system_process = process_allocate();
    system_idle = thread_allocate();

    // Creat the idle process
    thread_kernel_creat(system_idle);
    thread_set_entry(system_idle, (vaddr_t) process_idle);
    scheduler_set_current(system_idle);
    scheduler_add_thread(system_idle);

    // Creat the system process
    process_creat(system_process);
    process_add_thread(system_process, system_idle);

    // TODO: Load the init process
    // TODO: Creat the init process
}

/**
 * @brief Allocate memory for a process descriptor, and initialize its list
 * node and spinlock. Other fields are left uninitialized.
 * 
 * @return process_t* The newly allocated process descriptor, or NULL if
 * there is no memory available to allocate a process descriptor.
 */
process_t *process_allocate(void)
{
    process_t *process = malloc(sizeof(process_t));
    if (process == NULL)
        return NULL;

    spin_init(&process->spin);
    list_init(&process->node);
    list_init(&process->threads);
    list_init(&process->siblings);
    list_init(&process->children);
    return process;
}

/**
 * @brief Initialize a process descriptor and create a new memory context for
 * the process. The process descriptor contains no threads and no PID : it
 * will be initialized later, when a thread will be attached to the process.
 * 
 * @param process The process to initialize.
 * @return int 0 on success, or
 *  -ENOMEM if there is no memory available.
 */
int process_creat(process_t *process)
{
    assert(!null(process));
    process->pid = -1;
    process->sid = 0;
    process->uid = 0;
    process->gid = 0;
    process->pgid = 0;
    process->euid = 0;
    process->egid = 0;
    process->fsuid = 0;
    process->fsgid = 0;
    process->umask = 0;
    process->parent = NULL;
    process->mm_context = mm_context_create();
    if (process->mm_context == NULL)
        return -ENOMEM;
    return 0;
}

/**
 * @brief Destroy a process descriptor and its memory context.
 * For destroying a process, all thread must have been removed from the
 * process (destroyed or zombified) before calling this function.
 * 
 * @param process The process to destroy.
 * @return int Always 0.
 */
int process_destroy(process_t *process)
{
    assert(list_empty(&process->threads));
    spin_acquire(&list_lock) {
        list_remove(&process->node);
    }
    mm_context_drop(process->mm_context);
    free(process);
    return 0;
}

/**
 * @brief Add a thread to the system process (PID 0). All kernel threads sould
 * be added to this process.
 * 
 * @param thread Kernel thread to add to the system process.
 */
void process_add_system_thread(thread_t *thread)
{
    assert(!null(thread));
    assert(thread->type == THREAD_KERNEL);
    process_add_thread(system_process, thread);
}

/**
 * @brief Clone a process: Copy its memory context and its metadata (uid,
 * open files...).
 * Threads are not copied to the child process ! This is because
 * we cannot copy a thread without its saved CPU state. It is the 
 * responsability of the caller to copy threads if needed.
 * Because of this, the created process does not have a PID : it will be
 * updated when a thread will be added to the process
 * 
 * @param process The process to copy to : it will be allocated with
 * process_allocate() before calling this function.
 * @param parent The process to copy.
 * @return int 0 on success or
 *  -ENOMEM if the process cannot be created due to out of memory error.
 */
int process_clone(process_t *process, process_t *parent)
{
    process->mm_context = mm_context_clone(parent->mm_context);
    if (process->mm_context == NULL)
        return -ENOMEM;

    process->pid = -1;
    process->parent = parent;
    process->sid = parent->sid;
    process->uid = parent->uid;
    process->gid = parent->gid;
    process->pgid = parent->pgid;
    process->euid = parent->euid;
    process->egid = parent->egid;
    process->fsgid = parent->fsgid;
    process->fsgid = parent->fsgid;
    process->umask = parent->umask;
    return 0;
}

/**
 * @brief Add a thread to a process. If it is the first thread of the process,
 * the process PID will be updated and set to the thread TID.
 * 
 * @param process The process to add the thread to.
 * @param thread The thread to add to the process.
 * @return int Always 0.
 */
int process_add_thread(process_t *process, thread_t *thread)
{
    assert(!null(process));
    assert(!null(thread));
    assert(list_empty(&thread->process_node));

    thread->process = process;
    if (process->pid < 0)
        process->pid = thread->tid;

    spin_acquire(&process->spin) {
        list_add_tail(&process->threads, &thread->process_node);
    }
    return 0;
}

/**
 * @brief Remove a thread from a process.
 * 
 * @param process The process to remove the thread from.
 * @param thread The thread to remove from the process.
 * @return int Always 0
 */
int process_remove_thread(process_t *process, thread_t *thread)
{
    assert(!null(process));
    assert(!null(thread));
    assert(!list_empty(&thread->process_node));

    spin_acquire(&process->spin) {
        list_remove(&thread->process_node);
    }
    thread->process = NULL;
    return 0;
}

/**
 * @brief This function is called when a process is a abandoned (its parent
 * has died). It will update the parent of the process to the init process
 * which will take care of the process until his death.
 * 
 * @param process The process that is abandoned.
 * @return int Always 0.
 */
int process_abandoned(process_t *process)
{
    process_t *parent = process_get_by_pid(PROCESS_INIT_PID);
    assert(!null(process));
    assert(!null(parent));

    spin_acquire(&process->spin) {
        process->parent = parent;
        list_remove(&process->siblings);
        list_add(&parent->children, &process->siblings);
    }
    return 0;
}

/**
 * @brief Get a process by its PID.
 * 
 * @param pid The PID of the process to get.
 * @return process_t* The process with the given PID or NULL if not found.
 */
process_t *process_get_by_pid(const pid_t pid)
{
    spin_acquire(&list_lock) {
        list_foreach(&processes, entry) {
            process_t *process = list_entry(entry, process_t, node);
            if (process->pid == pid)
                return process;
        }
    }
    return NULL;
}
