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
#include <lib/log.h>
#include <core/mm/slub.h>
#include <core/mm/malloc.h>

static malloc_slub_t slub[] = {
    {32, NULL, 256, 8},
    {64, NULL, 128, 4},
    {128, NULL, 64, 4},
    {256, NULL, 32, 2},
    {512, NULL, 16, 2},
    {1024, NULL, 8, 1},
    {2048, NULL, 8, 1},
    {4096, NULL, 8, 1},
    {8192, NULL, 4, 0},
    {16384, NULL, 4, 0},
    {32768, NULL, 4, 0},
    {65536, NULL, 2, 0},
    {0, NULL, 0, 0}
};

void kmalloc_setup(void)
{
    for (int i = 0; slub[i].length != 0; i++) {
        slub[i].allocator = creat_slub_allocator(
            slub[i].length,
            slub[i].length,
            0, // No minimum free object count
            slub[i].obj_per_slub,
            slub[i].initial_slub_count,
            SLUB_LAZY);
    }
}

_malloc void *kmalloc(size_t size, int flags)
{
    for (int i = 0; slub[i].length != 0; i++) {
        if (size <= slub[i].length)
            return slub_allocate(slub[i].allocator);
    }
    error("Allocation of %u bytes is too big for kmalloc", size);
    return NULL;
}

void kfree(void *obj)
{
    for (int i = 0; slub[i].length != 0; i++) {
        if (slub_free(slub[i].allocator, obj))
            return;
    }
    error("Allocation 0x%p cannot be freed: not allocated with kmalloc", obj);
}