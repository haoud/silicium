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
#include <lib/log.h>
#include <lib/maths.h>
#include <lib/memory.h>
#include <core/mm/slub.h>
#include <core/mm/paging.h>
#include <core/mm/vmalloc.h>

/**
 * @brief This file contains the code that manages the kernel space allocations.
 * The code here is very simple, not very powerful and incomplete but sufficient
 * for now.
 * TODO: Use a red-black tree to limit the number of iterations to find an area
 * TODO: Merge adjacent free memory areas
 * TODO: Make the algorithm lock free or more scalable
 */

static slub_allocator_t *allocator;
static DECLARE_LIST(free_list);
static DECLARE_LIST(used_list);
static DECLARE_SPINLOCK(lock);

static vmarea_t *vmarea_allocate(void)
{
    vmarea_t *vma = slub_allocate(allocator);
    if (vma != NULL)
        list_entry_init(&vma->node);
    return vma;
}

_init void vmalloc_setup(void)
{
    const vaddr_t start = VMALLOC_START - 8192;
    const vaddr_t end = VMALLOC_START;
    allocator = creat_slub_allocator(
        sizeof(vmarea_t),
        VMALLOC_VMAREA_ALIGN,
        VMALLOC_VMAREA_MIN_FREE,
        VMALLOC_VMAREA_PER_SLUB,
        0,
        SLUB_LAZY);

    // We can't creat a slub normally because vmalloc is not initialized yet.
    paging_map_interval(start, end, PAGING_READ | PAGING_WRITE);
    slub_add_memory(allocator, start, end);

    vmarea_t *vma = vmarea_allocate();   
    vma->length = VMALLOC_END - VMALLOC_START;
    vma->base = VMALLOC_START;
    vma->mapped = 0;
    list_add_tail(&free_list, &vma->node);
}

/**
 * @brief Allocates a virtual memory area of the given size.
 * 
 * @param size Size of the area to allocate, must be a multiple of PAGE_SIZE
 * @param flags Flags to control the allocation
 * @return The memory allocated, or 0 if the request cannot be done
 */
_export vaddr_t vmalloc(size_t size, int flags)
{
#ifndef CONFIG_DISABLE_CHECKS
    size = align(size, PAGE_SIZE);
#endif

    // Find the first free area that is big enough
    spin_lock(&lock);
    vmarea_t *vma = NULL;
    list_foreach(&free_list, entry) {
        vma = list_entry(entry, vmarea_t, node);
        if (vma->length >= size) 
            break;
    }
    if(vma == NULL) {
        spin_unlock(&lock);
        return 0;
    }

    list_remove(&vma->node);
    list_add_tail(&used_list, &vma->node);

    // Split the area if necessary
    if (vma->length > size) {
        vmarea_t *const new_vma = vmarea_allocate();
        if (new_vma == NULL) {
            // We can't split the area, so we put it back in the free list
            list_remove(&vma->node);
            list_add_tail(&free_list, &vma->node);
            spin_unlock(&lock);
            return 0;
        }
        new_vma->length = vma->length - size;
        new_vma->base = vma->base + size;
        vma->length = size;
        list_add_tail(&free_list, &new_vma->node);
    }

    if (flags & VMALLOC_MAP) {
        const int ret = paging_map_interval(
                            vma->base,
                            vma->base + vma->length,
                            PAGING_READ | PAGING_WRITE);
        if (ret < 0) {
            // We can't map the area, so we put it back in the free list
            list_remove(&vma->node);
            list_add_tail(&free_list, &vma->node);
            spin_unlock(&lock);
            return 0;
        }
        vma->mapped = 1;
    }

    spin_unlock(&lock);
    return vma->base;
}

/**
 * @brief Free a memory area allocated by vmalloc.
 * TOOD Merge adjacent free memory areas
 * 
 * @param ptr Base address of the memory area to free.
 */
_export void vmfree(vaddr_t addr)
{
    spin_lock(&lock);
    list_foreach(&used_list, entry) {
        vmarea_t *const vma = list_entry(entry, vmarea_t, node);
        if (vma->base == addr) {
            list_remove(&vma->node);
            if (vma->mapped) {
                paging_unmap_interval(vma->base, vma->base + vma->length);
                vma->mapped = 0;
            }
            list_add_head(&free_list, &vma->node);
            spin_unlock(&lock);
            return;
        }
    }
    spin_unlock(&lock);
    warn("vmfree(): impossible to free the memory"
        " because the area doesn't exist");
}
