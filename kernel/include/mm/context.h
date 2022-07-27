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

typedef struct mm_context {
    atomic_t usage;
    vaddr_t pd;
} mm_context_t;

struct mm_context *mm_context_clone(struct mm_context *context);
struct mm_context *mm_context_create(void);

void mm_context_use(struct mm_context *context);
void mm_context_set(struct mm_context *context);
void mm_context_drop(struct mm_context *context);
