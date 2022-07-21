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

// TODO: use a wider and more precise range of addresses
#define VMALLOC_START   0xD0000000
#define VMALLOC_END     0xF0000000

#define VMALLOC_NONE    0x00
#define VMALLOC_MAP     0x01

#define VMALLOC_VMAREA_MIN_FREE 0
#define VMALLOC_VMAREA_PER_SLUB 64
#define VMALLOC_VMAREA_ALIGN    16

#define vmallocp(size, flags)   ((void *) vmalloc(size, flags))
#define vmfreep(ptr)            vmfree((void *) ptr)

typedef struct vmarea {
    vaddr_t base;
    vaddr_t length;
    struct list_head node;
    int mapped;
} vmarea_t;

_init void vmalloc_setup(void);

_export vaddr_t vmalloc(size_t size, int flags);
_export void vmfree(vaddr_t addr);