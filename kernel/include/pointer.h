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

typedef uint32_t ptroff_t;
typedef uint32_t ptr_t;

// Macro to handle pointer arithmetic more easily and safely.
#define ptr_offset(ptr, i) ((void *) ((ptr_t) (ptr) + (ptrdiff_t) (i)))
#define ptr_add_cast(ptr, u, type) ptr_cast(((ptr_t) (ptr) + (ptr_t) (u)), type)
#define ptr_sub_cast(ptr, u, type) ptr_cast(((ptr_t) (ptr) - (ptr_t) (u)), type)
#define ptr_add(ptr, u) ptr_add_cast(ptr, u, void *)
#define ptr_sub(ptr, u) ptr_sub_cast(ptr, u, void *)
#define ptr_cast(ptr, type) ((type) (ptr))
