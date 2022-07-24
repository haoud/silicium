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
#include <multiboot.h>
#include <lib/hashmap.h>

#define SYMBOLS_HASHMAP_LENGTH 128

typedef struct symbol {
    struct hash_node node;
    const char *name;
    vaddr_t value;
} symbol_t;

_init void symbol_init(struct mb_info *mb_info);

int symbol_remove(const char *name);
bool symbol_exists(const char *name);
vaddr_t symbol_get_value(const char *name);
int symbol_add(const char *name, const vaddr_t value);
