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
#include <config.h>
#include <lib/maths.h>
#include <core/mm/page.h>
#include <core/mm/slub.h>
#include <core/mm/vmalloc.h>

static slub_allocator_t slub_allocator_allocator;
static slub_allocator_t *slub_allocator;
static slub_t second_slub;
static slub_t first_slub;

/**
 * @brief Create a list node at a free object emplacement. This is not a problem
 * because the location is free by definition. When we allocate the object, this
 * node will be emoved from the list. This saves us from allocating extra memory
 * to store the list of free objects
 * 
 * @param slub The slub where the object is located
 * @param object Location of the object to create the node at
 */
static void slub_add_object_to_free_list(
    slub_t *const slub,
    const vaddr_t object)
{
    struct list_head *node = (struct list_head *)object;
    list_entry_init(node);
    list_add_tail(&slub->free_objects, node);
}

/**
 * @brief This function releases an object from a slub. It takes care of adding
 * the newly freed slot to the list of free slots and updates some statistics on
 * the use of the slub
 * 
 * @param allocator The slub allocator
 * @param slub The slub where the object is located
 * @param object Location of the object to release
 */
static void slub_free_object(
    slub_allocator_t *const allocator,
    slub_t *const slub,
    const vaddr_t object)
{
    slub->objects_used--;
    allocator->free_count++;
    if (slub->objects_used == 0) {
        list_remove(&slub->slub_list);
        list_add_tail(&allocator->free_slubs, &slub->slub_list);
    }
    else if (slub->objects_used == slub->objects_max - 1) {
        list_remove(&slub->slub_list);
        list_add_tail(&allocator->used_slubs, &slub->slub_list);
    }
    slub_add_object_to_free_list(slub, object);
}

/**
 * @brief This function simply adds all locations of a newly created slub to the
 * list of free sobjects
 * 
 * @param slub The newly created slub
 */
static void slub_init_free_list(slub_t *const slub)
{
    // Use free memory to store the linked list node
    const size_t obj_size = align(slub->object_size, slub->object_align);
    for (vaddr_t addr = slub->start;
        addr + obj_size < slub->end;
        addr += obj_size)
        slub_add_object_to_free_list(slub, addr);
}

/**
 * @brief Add a slub to an allocator
 * 
 * @param allocator Allocator to add the slub to
 * @param slub The slub to add
 */
static void slub_add_slub(slub_allocator_t *allocator, slub_t *slub)
{
    list_add(&allocator->free_slubs, &slub->slub_list);
    allocator->total_count += slub->objects_max;
    allocator->free_count += slub->objects_max;
}

/**
 * @brief Allocate a new slub allocator and initialize it: only linked list
 * nodes and spinlock are initialized, other fields are undefined
 * 
 * @return slub_t* The newly allocated slub allocator or NULL if the allocation
 * failed
 */
static slub_allocator_t *slub_allocate_allocator(void)
{
    slub_allocator_t *allocator = slub_allocate(&slub_allocator_allocator);
    if (allocator == NULL)
        return allocator;
    list_init(&allocator->free_slubs);
    list_init(&allocator->used_slubs);
    list_init(&allocator->full_slubs);
    spin_init(&allocator->lock);
    return allocator;
}

/**
 * @brief Allocate a new slub and initialize it: only linked list nodes and
 * spinlock are initialized, other fields are undefined
 * 
 * @return slub_t* The newly allocated slub or NULL if the allocation failed
 */
static slub_t *slub_allocate_slub(void)
{
    slub_t *slub = slub_allocate(slub_allocator);
    if (slub == NULL)
        return slub;
    list_entry_init(&slub->slub_list);
    list_init(&slub->free_objects);
    spin_init(&slub->lock);
    return slub;
}

/**
 * @brief Create a new slub
 * 
 * @param allocator The slub allocator of the slub to create
 * @param length Length of the slub, must be page aligned
 * @return The newly created slub, or NULL if the allocation failed
 */
static slub_t *slub_creat(slub_allocator_t *allocator, size_t length)
{
    assert(PAGE_ALIGNED(length));
    assert(!null(allocator));

    const vaddr_t start = vmalloc(length, VMALLOC_MAP);
    if (start == 0)
        return NULL;

    slub_t *slub = slub_allocate_slub();
    if (slub == NULL) {
        vmfree((void *) start);
        return slub;
    }

    slub->object_align = allocator->object_align;
    slub->object_size = allocator->object_size;
    slub->objects_max = length / align(slub->object_size, slub->object_align);
    slub->objects_used = 0;

    slub->start = start;
    slub->end = start + length;
    slub_init_free_list(slub);
    return slub;
}

/**
 * @brief Create a new slub and add it to the list of free slubs of the
 * allocator passed as parameter
 * @param allocator The slub allocator to add the slub to
 * @return 0 on success, -1 on failure
 */
static int slub_creat_and_add(slub_allocator_t *allocator) 
{
    assert(!null(allocator));
    const size_t length = align(
        allocator->object_per_slub * allocator->object_size,
        PAGE_SIZE);

    slub_t *slub = slub_creat(allocator, length);
    if (slub == NULL)
        return -1;
    slub_add_slub(allocator, slub);
    return 0;
}

/**
 * @brief Setup the slub allocator and create allocators for slub_allocator_t
 * and slub_t
 * TODO: Make this code cleaner
 */
_init void slub_setup(void)
{
    static char buffer1[SLUB_DEFAULT_LENGTH] _align(PAGE_SIZE);
    static char buffer2[SLUB_DEFAULT_LENGTH] _align(PAGE_SIZE);

    /* Setup initial slub for slub_allocator_allocator */
    first_slub.object_align = SLUB_DEFAULT_ALIGN;
    first_slub.object_size = sizeof(slub_allocator_t);
    first_slub.objects_max = PAGE_SIZE / align(
        first_slub.object_size,
        first_slub.object_align);
    first_slub.start = (vaddr_t) buffer1;
    first_slub.end = first_slub.start + SLUB_DEFAULT_LENGTH;
    first_slub.objects_used = 0;
    list_entry_init(&first_slub.slub_list);
    list_init(&first_slub.free_objects);
    spin_init(&first_slub.lock);

    /* Setup slub allocator for slub_allocator_t */
    slub_allocator_allocator.object_align = first_slub.object_align;
    slub_allocator_allocator.object_size = first_slub.object_size;
    slub_allocator_allocator.total_count = first_slub.objects_max;
    slub_allocator_allocator.free_count = first_slub.objects_max;
    slub_allocator_allocator.min_free = 2;        // Safety margin
    list_init(&slub_allocator_allocator.free_slubs);
    list_init(&slub_allocator_allocator.used_slubs);
    list_init(&slub_allocator_allocator.full_slubs);
    spin_init(&slub_allocator_allocator.lock);

    list_add(&slub_allocator_allocator.free_slubs, &first_slub.slub_list);
    slub_init_free_list(&first_slub);

    /* Setup inital slub for slub_allocator */
    second_slub.object_align = SLUB_DEFAULT_ALIGN;
    second_slub.object_size = sizeof(slub_t);
    second_slub.objects_max = PAGE_SIZE / align(
        second_slub.object_size,
        second_slub.object_align);
    second_slub.start = (vaddr_t) buffer2;
    second_slub.end = second_slub.start + SLUB_DEFAULT_LENGTH;
    second_slub.objects_used = 0;
    list_entry_init(&second_slub.slub_list);
    list_init(&second_slub.free_objects);
    spin_init(&second_slub.lock);

    /* Setup slub allocator for slub_t */
    slub_allocator = slub_allocate_allocator();
    slub_allocator->object_per_slub = SLUB_MIN_OBJECT_PER_SLUB * 8;
    slub_allocator->object_align = second_slub.object_align;
    slub_allocator->object_size = second_slub.object_size;
    slub_allocator->total_count = second_slub.objects_max;
    slub_allocator->free_count = second_slub.objects_max;
    slub_allocator->min_free = 2;       // Safety margin

    list_add(&slub_allocator->free_slubs, &second_slub.slub_list);
    slub_init_free_list(&second_slub);
}

_init int slub_add_memory(
    slub_allocator_t *allocator,
    const vaddr_t start,
    const vaddr_t end)
{
    slub_t *slub = slub_allocate_slub();
    if (slub == NULL)
        return -1;

    const size_t length = end - start;
    slub->object_align = allocator->object_align;
    slub->object_size = allocator->object_size;
    slub->objects_max = length / align(slub->object_size, slub->object_align);
    slub->objects_used = 0;
    slub->start = start;
    slub->end = end;

    slub_init_free_list(slub);
    slub_add_slub(allocator, slub);
    return 0;
}

/**
 * @brief Free a object from a slub. This function will never block or remove
 * unused slubs.
 * 
 * @param allocator The slub allocator used to allocate the object
 * @param object The object to free
 * @return 1 if the object was freed, 0 if the object was not found or null, -1
 * on failure
 */
_export int slub_free(slub_allocator_t *allocator, void *object)
{
    assert(!null(allocator));
    if (null(object))
        return 0;

    const vaddr_t obj = (vaddr_t) object;
    if (!slub_is_aligned(allocator, object))
        return 0;

    list_foreach (&allocator->full_slubs, entry) {
        slub_t *const s = list_entry(entry, slub_t, slub_list);
        if (slub_is_in(s, obj)) {
            slub_free_object(allocator, s, obj);
            return 1;
        }
    }

    list_foreach (&allocator->used_slubs, entry) {
        slub_t *const s = list_entry(entry, slub_t, slub_list);
        if (slub_is_in(s, obj)) {
            slub_free_object(allocator, s, obj);
            return 1;
        }
    }
    return 0;
}

/**
 * @brief Allocate a new object from the slub allocator
 * 
 * @param allocator The slub allocator to allocate from
 * @return The allocated object, or NULL if the allocation failed
 */
_export void *slub_allocate(slub_allocator_t *allocator)
{
    assert(!null(allocator));
    
    slub_t *slub = NULL;
    do {
        spin_lock(&allocator->lock);
        struct list_head *slub_pool = &allocator->used_slubs;
        if (list_empty(slub_pool)) {
            slub_pool = &allocator->free_slubs;
        }
        if (list_empty(slub_pool)) {
            if (slub_creat_and_add(allocator) < 0) {
                spin_unlock(&allocator->lock);
                return NULL;
            }
            slub_pool = &allocator->free_slubs;
        }

        // If we need to allocate a slub to respect the min_free count, do it
        if (allocator->free_count == allocator->min_free) {
            if (slub_creat_and_add(allocator) < 0) {
                spin_unlock(&allocator->lock);
                return NULL;
            }
        }

        assert(!list_empty(slub_pool));
        
        slub = list_entry(slub_pool->next, slub_t, slub_list);
        spin_unlock(&allocator->lock);
        spin_lock(&slub->lock);

        // Verify if the slub is still avaible after the locking
        if (!list_empty(&slub->free_objects))
            break;

        spin_unlock(&slub->lock);
    } while (1);

    struct list_head *node = slub->free_objects.next;
    list_remove(node);

    // If the slub is no longer, empty, move it to the used slub list
    if (slub->objects_used == 0) {
        list_remove(&slub->slub_list);
        list_add(&allocator->used_slubs, &slub->slub_list);
    }

    // If the slub is full, move it to the full slub list
    slub->objects_used++;
    if (slub->objects_used == slub->objects_max) {
        list_remove(&slub->slub_list);
        list_add(&allocator->full_slubs, &slub->slub_list);
    }

    spin_unlock(&slub->lock);
    allocator->free_count--;
    return (void *) node;
}

/**
 * @brief Create a new slub allocator
 * 
 * @param obj_size Size of the objects to allocate
 * @param obj_align Minimal alignement of the objects to allocate, must be a
 * power of 2
 * @param min_free Minimal number of free objects in a allocator before adding
 * a new slub
 * @param obj_per_slub Number of objects in a slub. If < 0, the number of
 * objects is automatically computed.
 * @param slub_count Number of slubs to create.
 * @param flags Flags for the slub allocator
 *
 * @return slub_allocator_t* The new slub allocator, NULL on failure
 */
_export slub_allocator_t *creat_slub_allocator(
    size_t obj_size,
    size_t obj_align,
    size_t min_free,
    uint_t obj_per_slub,
    uint_t slub_count,
    uint_t flags)
{
    slub_allocator_t *allocator = slub_allocate_allocator();
    if(allocator == NULL)
        return allocator;

#ifndef CONFIG_DISABLE_CHECKS
    obj_per_slub = max(obj_per_slub, (uint_t) SLUB_MIN_OBJECT_PER_SLUB);
    slub_count = max(slub_count, (uint_t) SLUB_DEFAULT_SLUB_COUNT);
    obj_align = max(obj_align, (uint_t) SLUB_MIN_OBJECT_ALIGN);
    obj_size = max(obj_size, (uint_t) SLUB_MIN_OBJECT_LENGTH);
#endif
    if (flags & SLUB_LAZY)
        slub_count = 0;

    allocator->object_per_slub = obj_per_slub;
    allocator->object_align = obj_align;
    allocator->object_size = obj_size;
    allocator->min_free = min_free;
    allocator->total_count = 0;
    allocator->free_count = 0;

    for (uint_t i = 0; i < slub_count; i++)
        slub_creat_and_add(allocator);
    while (allocator->free_count < allocator->min_free)
        slub_creat_and_add(allocator);
    return allocator;
}
