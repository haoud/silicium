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

#define SLUB_MIN_OBJECT_PER_SLUB    4
#define SLUB_MIN_OBJECT_LENGTH      16
#define SLUB_MIN_OBJECT_ALIGN       16
#define SLUB_DEFAULT_ALIGN          16
#define SLUB_DEFAULT_LENGTH         PAGE_SIZE
#define SLUB_DEFAULT_SLUB_COUNT     4

#define SLUB_NONE 0x00
#define SLUB_LAZY 0x01

#define slub_is_aligned(slub, obj) \
    (((uintptr_t) (obj) & ~((slub)->object_align - 1)) == 0)

// Very basic, but enough for now, use slub_is_aligned() for extra checks
#define slub_is_in(slub, obj)             \
    ((uintptr_t) (obj) >= (slub)->start && \
     (uintptr_t) (obj) < (slub)->end)

typedef struct slub {
    struct list_head free_objects;
    struct list_head slub_list;
    struct spinlock lock;
    
    unsigned int object_align;
    unsigned int object_size;
    unsigned int objects_max;
    unsigned int objects_used;
    vaddr_t start;
    vaddr_t end;
} slub_t;

typedef struct slub_allocator {
    struct list_head free_slubs;
    struct list_head full_slubs;
    struct list_head used_slubs;
    struct spinlock lock;

    unsigned int object_per_slub;   // Only used as a hint when creating a slub
    unsigned int object_align;
    unsigned int object_size;
    unsigned int total_count;
    unsigned int min_free;
    uatomic_t free_count;
} slub_allocator_t;

_init void slub_setup(void);
_init int slub_add_memory(
    slub_allocator_t *allocator,
    const vaddr_t start,
    const vaddr_t end);
    
_export void *slub_allocate(slub_allocator_t *allocator);
_export int slub_free(slub_allocator_t *allocator, void *object);
_export slub_allocator_t *creat_slub_allocator(
    size_t obj_size,
    size_t obj_align,
    size_t min_free,
    uint_t obj_per_slub,
    uint_t slub_count,
    uint_t flags);
// Slub allocator cannot be destroyed yet.
