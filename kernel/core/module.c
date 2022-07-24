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
#include <core/elf.h>
#include <lib/list.h>
#include <core/module.h>
#include <core/symbol.h>
#include <lib/spinlock.h>

static DECLARE_LIST(module_list);
static DECLARE_SPINLOCK(lock);

int module_load(char *module, const char *name)
{
    spin_lock(&lock);
    spin_unlock(&lock);
}

int module_unload(const char *name)
{
    spin_lock(&lock);
    spin_unlock(&lock);
}
