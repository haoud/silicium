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

typedef void (*module_init_t)(void);
typedef void (*module_finit_t)(void);

#define MODULE_NAME(name) \
    static const _used char *__module_name__ = (name);

#define MODULE_AUTHOR(author) \
    static const _used char *__module_author__ = (author);

#define MODULE_LICENSE(license) \
    static const _used char *__module_license__ = (license);

#define MODULE_VERSION(version) \
    static const _used char *__module_version__ = (version);

#define MODULE_DESCRIPTION(description) \
    static const _used char *__module_description__ = (description);

#define MODULE_INIT(init) \
    static const _used module_init_t __module_init__ = (init);

#define MODULE_EXIT(exit) \
    static const _used module_finit_t __module_exit__ = (exit);
