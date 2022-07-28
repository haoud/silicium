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
#include <mm/vmalloc.h>
#include <lib/memory.h>
#include <arch/x86/gdt.h>
#include <process/thread.h>

static DECLARE_SPINLOCK(tid_lock);
static DECLARE_SPINLOCK(lock);
static DECLARE_LIST(threads);
static pid_t tid = 0;
static atomic_t thread_count = 0;

/**
 * @brief Check if the given TID is free for use. This function does not
 * lock the thread list, it is up to the caller to do so.
 * 
 * @param id The TID to check.
 * @return int true if the TID is free, false otherwise.
 */
static int thread_is_tid_free(pid_t id)
{
    list_foreach (&threads, entry) {
        thread_t *thread = list_entry(entry, thread_t, thread_node);
        if (thread->tid == id)
            return false;
    }
    return true;
}

/**
 * @brief Generate a new TID and add the thread to the global thread list. 
 * This function id not designed to fail, so if there is no free TID, it
 * will hang the system...
 * So be careful when using this function, and dont call it if there
 * are no free TID (ie MAX_PID threads).
 * 
 * @param thread The thread to assign the TID to.
 */
static void thread_generate_tid(struct thread *thread)
{
    thread->tid = -1;
    spin_acquire(&lock) {
        list_add_tail(&threads, &thread->thread_node);
    }
    spin_acquire(&tid_lock) {
        do {
            if (tid == PID_MAX)
                tid = 0;
            if (thread_is_tid_free(tid))
                thread->tid = tid;
            tid++;
        } while (thread->tid == -1);
    }
}

/**
 * @brief Initialize a thread structure. This function initializes the thread
 * partialy (ie the fields that are common for all threads, kernel or user) and
 * gives it a new TID before adding it to the global thread list.
 * 
 * @param thread The thread to initialize. This function does not allocate
 * any memory, so the caller must allocate it with thread_allocate() before
 * calling this function.
 * @return int 0 on success, or
 *  -EAGAIN if there is no free TID.
 */
static int thread_creat(thread_t *thread)
{
    // If there are too many threads, we can't create a new one
    if (++thread_count >= THREAD_MAX) {
        thread_count--;
        return -EAGAIN;
    }

    // Place the cpu state in the stack
    const uintptr_t cpu_state = thread->kstack.top - sizeof(struct cpu_state);
    thread->cpu_state = (struct cpu_state *) (cpu_state & 0xFFFFFFF0);

    list_init(&thread->scheduler_node);
    list_init(&thread->thread_node);
    thread->state = THREAD_CREATED;
    thread->reschedule = false;
    thread->fpu_loaded = false;
    thread->fpu_used = false;

    thread_generate_tid(thread);    // Cannot fail
    return 0;
}

/**
 * @brief Allocate memory for a thread descritor, and allocate memory for the
 * kernel stack and the fpu state. Other fields are uninitialized.
 * 
 * @return thread_t* The new thread, or NULL if the kernel ran out of memory.
 */
thread_t *thread_allocate(void)
{
    thread_t *thread = malloc(sizeof(thread_t));
    if (thread == NULL)
        return NULL;

    thread->fpu_state = malloc(sizeof(struct fpu_state));
    if (thread->fpu_state == NULL) {
        free(thread);
        return NULL;
    }
    
    thread->kstack.base = vmalloc(KSTACK_SIZE, VMALLOC_MAP);
    thread->kstack.size = KSTACK_SIZE;
    if (thread->kstack.base == 0) {
        free(thread->fpu_state);
        free(thread);
        return NULL;
    }
    thread->kstack.top = thread->kstack.base + thread->kstack.size;
    return thread;
}

/**
 * @brief Creat a new kernel thread. It will initialize the CPU state but does
 * not create a memory context: kernel threads are not user-space threads and
 * does not need their own memory context: they can use any memory context.
 * 
 * @param thread The thread to initialize. This function does not allocate
 * any memory, so the caller must allocate it with thread_allocate() before
 * calling this function.
 * @return int 0 on success, or
 *  -EAGAIN if there is no free TID.
 */
int thread_kernel_creat(thread_t *thread)
{
    int ret = thread_creat(thread);
    if (ret < 0)
        return ret;

    thread->type = THREAD_KERNEL;
    thread->mm_context = NULL;
    thread->mm_context_borrowed = NULL;
    thread->cpu_state->cs = GDT_KCODE_SELECTOR;
    thread->cpu_state->ds = GDT_KDATA_SELECTOR;
    thread->cpu_state->es = GDT_KDATA_SELECTOR;
    thread->cpu_state->fs = GDT_KDATA_SELECTOR;
    thread->cpu_state->gs = GDT_KDATA_SELECTOR;
    thread->cpu_state->ss = GDT_KSTACK_SELECTOR;
    thread->cpu_state->eflags = EFLAGS_IF;
    return 0;
}

/**
 * @brief Creat a new user thread. It will allocate a memory context for the
 * thread, and initialize the CPU state.
 * 
 * @param thread The thread to initialize. This function does not allocate
 * any memory, so the caller must allocate it with thread_allocate() before
 * calling this function.
 * @return int 0 on success, or
 *  -EAGAIN if there is no free TID.
 *  -ENOMEM if the kernel ran out of memory.
 */
int thread_user_creat(thread_t *thread)
{
    int ret = thread_creat(thread);
    if (ret < 0)
        return ret;

    thread->type = THREAD_KERNEL;
    thread->mm_context = mm_context_create();
    thread->mm_context_borrowed = NULL;
    if (thread->mm_context == NULL) {
        thread_destroy(thread);
        return -ENOMEM;
    }

    thread->cpu_state->cs = GDT_UCODE_SELECTOR;
    thread->cpu_state->ds = GDT_UDATA_SELECTOR;
    thread->cpu_state->es = GDT_UDATA_SELECTOR;
    thread->cpu_state->fs = GDT_UDATA_SELECTOR;
    thread->cpu_state->gs = GDT_UDATA_SELECTOR;
    thread->cpu_state->ss = GDT_KSTACK_SELECTOR;
    thread->cpu_state->ss3 = GDT_USTACK_SELECTOR;
    thread->cpu_state->esp3 = THREAD_STACK_TOP - 16;
    thread->cpu_state->eflags = EFLAGS_IF;
    // TODO: Map the stack
    return 0;
}

/**
 * @brief Clone a thread. This function will create a new thread, and copy the
 * CPU state and memory context from the source thread.
 * Currently, kernel threads cannot be cloned.
 * 
 * @param clone The thread to initialize. This function does not allocate
 * any memory, so the caller must allocate it with thread_allocate() before
 * calling this function.
 * @param thread The thread to clone: must be a user thread.
 * @param cpu_state The CPU state of the thread to clone.
 * @return int 0 on success, or
 *  -EAGAIN if there is no free TID.
 *  -EINVAL if the thread is a kernel thread.
 *  -ENOMEM if the kernel ran out of memory.
 */
int thread_clone(
    thread_t *clone,
    const thread_t *thread,
    const cpu_state_t *cpu_state)
{
    if (thread->type == THREAD_KERNEL)
        return -EINVAL;

    int ret = thread_creat(clone);
    if (clone == NULL)
        return ret;

    // Clone the MM context if necessary
    if (thread->mm_context) {
        clone->mm_context = mm_context_clone(thread->mm_context);
        if (clone->mm_context == NULL) {
            thread_destroy(clone);
            return -ENOMEM;
        }
    }

    // Copy the cpu state and the FPU state
    memcpy(clone->fpu_state, thread->fpu_state, sizeof(struct fpu_state));
    memcpy(clone->cpu_state, cpu_state, sizeof(struct cpu_state));

    clone->fpu_used = thread->fpu_used;
    clone->state = thread->state;
    clone->type = thread->type;
    if (clone->type == THREAD_RUNNING)
        clone->type = THREAD_READY;

    // TODO: Setup Copy on Write
    return 0;
}

/**
 * @brief Set the entry point of the thread. This function is very simple
 * but exists to make an abstraction from the architecture for some other
 * part of the kernel. 
 * 
 * @param thread The thread to set the entry point of.
 * @param entry The entry point of the thread.
 */
void thread_set_entry(thread_t *thread, const vaddr_t entry)
{
    thread->cpu_state->eip = entry;
}

/**
 * @brief Make a thread a zombie : Its memory context will be destroyed but
 * cannot be removed from the thread list because it has not been joined by
 * its parent, so we need to keep some information about the thread.
 * Before calling this function, the thread must be removed from the scheduler
 * 
 * @param thread The thread to zombify. It must be the current thread in
 * the current CPU in order to be able to destroy it easily.
 * @param code The exit code given by the thread 
 */
void thread_zombify(thread_t *thread, const int code)
{
    assert(list_empty(&thread->scheduler_node));
    if (thread->mm_context)
        mm_context_drop(thread->mm_context);
    thread->state = THREAD_ZOMBIE;
    thread->exit_code = code;
}

/**
 * @brief Destroy a thread: free all its memory and remove it from the thread
 * list
 * 
 * @param thread The thread to destroy: it must have been zombified before
 * calling this function.
 */
void thread_destroy(thread_t *thread)
{
    // Remove the thread from the thread list
    spin_acquire(&lock) {
        list_remove(&thread->scheduler_node);
    }

    // Free the thread structure
    vmfree(thread->kstack.base);
    free(thread->fpu_state);
    free(thread);
    thread_count--;
}

/**
 * @brief Get the thread associated with the given TID.
 * 
 * @param id The TID of the thread to get.
 * @return thread_t* The thread with the given TID, or NULL if no thread
 * with the given TID exists.
 */
thread_t *thread_get_by_tid(const pid_t id)
{
    spin_acquire(&lock) {
        list_foreach(&threads, entry) {
            thread_t *thread = list_entry(entry, thread_t, thread_node);
            if (thread->tid == id)
                return thread;
        }
    }
    return NULL;
}
