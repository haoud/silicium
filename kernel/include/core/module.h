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
#include <module.h>
#include <lib/list.h>

typedef struct module {
    const char *elf;

    const char *author;
    const char *description;
    const char *name;
    const char *version;
    
    module_init_t init;
    module_finit_t finit;
    uatomic_t usage;
    struct list_head node;
} module_t;

int module_load(char *module);
int module_unload(const char *name);
int module_exist(const char *name);
