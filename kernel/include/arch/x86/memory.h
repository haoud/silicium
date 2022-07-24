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
#include <config.h>
#include <kernel.h>

#define PAGE_SIZE   4096
#define PAGE_SHIFT  12
#define PAGE_MASK   ~(PAGE_SIZE - 1)

#define PAGE_ALIGNED(x) (((x) & ~PAGE_MASK) == 0)
#define PAGE_ALIGN(x) ((x) & PAGE_MASK)

#define KERNEL_BASE 0xC0000000

#ifdef CONFIG_EXTRA_CHECKS
#define null(addr) ((uintptr_t) (addr) < PAGE_SIZE)
#else
#define null(addr) (!(uintptr_t) (addr))
#endif

#define kernel_space(addr) ((uintptr_t) (addr) > KERNEL_BASE)
