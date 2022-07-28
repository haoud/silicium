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
#include <mm/paging.h>
#include <mm/malloc.h>
#include <mm/vmalloc.h>
#include <mm/context.h>

#define assert_context_is_valid(context) \
    assert(!null(context));              \
    assert(context->pd != 0);            \
    assert(kernel_space(context->pd));   \
    assert(PAGE_ALIGNED(context->pd));   \
    assert(context->usage > 0);

/**
 * @brief Allocate a memory context, allocate a area to access
 * the page directory and initialize the usage counter to 1.
 * 
 * @return struct mm_context* The mm_context_t created
 * @return NULL If the allocation failed
 */
static struct mm_context *mm_context_allocate(void)
{
    struct mm_context *context = malloc(sizeof(struct mm_context));
    if (context == NULL)
        return NULL;
    context->pd = vmalloc(PAGE_SIZE, VMALLOC_MAP);
    if (context->pd == 0) {
        free(context);
        return NULL;
    }
    context->usage = 1;
    return context;
}

/**
 * @brief Clone a memory context.
 * 
 * @param context The mm_context_t to clone
 * @return struct mm_context* The mm_context_t cloned
 * @return NULL If the allocation failed
 */
struct mm_context *mm_context_clone(struct mm_context *context)
{
    assert_context_is_valid(context);
    struct mm_context *clone = mm_context_allocate();
    if (clone == NULL)
        return NULL;
    paging_clone_pd(clone->pd, context->pd);
    return clone;
}

/**
 * @brief Create a new memory context.
 * 
 * @return struct mm_context* The memory context created
 * @return NULL If the allocation failed
 */
struct mm_context *mm_context_create(void)
{
    struct mm_context *context = mm_context_allocate();
    if (context == NULL)
        return NULL;
    paging_creat_pd(context->pd);
    return context;
}

/**
 * @brief Increment the usage counter of a context.
 * 
 * @param context The context to use.
 */
void mm_context_use(struct mm_context *context)
{
    assert_context_is_valid(context);
    context->usage++;
}

/**
 * @brief Set the current context on the CPU.
 * 
 * @param context The context to set.
 */
void mm_context_set(struct mm_context *context)
{
    assert_context_is_valid(context);
    set_cr3(paging_get_paddr(context->pd));
}

/**
 * @brief Drop a context. The context is destroyed only if it is not
 * used anymore, otherwise the usage counter is simply decremented.
 * 
 * This function MUST be called when the context is currently loaded
 * into the CPU : otherwise the contexte destroyed will be the current
 * context !
 * 
 * @param context The context to destroy
 */
void mm_context_drop(struct mm_context *context)
{
    assert_context_is_valid(context);
    if (--context->usage != 0)
        return;
    paging_destroy_userspace();
    paging_use_kernel_pd();
    vmfree(context->pd);
    free(context);
}
