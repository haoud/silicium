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

#define GFP_NONE 0x00
#define GFP_ATOMIC 0x01
#define GFP_KERNEL 0x02

#define malloc(size) kmalloc(size, GFP_KERNEL)
#define free(obj) kfree(obj)

_init void kmalloc_setup(void);
_malloc void *kmalloc(const size_t size, const int flags);
void kfree(void *obj);
