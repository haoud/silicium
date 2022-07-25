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
#include <module.h>
#include <lib/log.h>

MODULE_NAME("test")
MODULE_VERSION("1.0")
MODULE_LICENSE("GPLv3")
MODULE_AUTHOR("Romain Cadilhac")
MODULE_DESCRIPTION("A module to test the kernel module system")

static void startup(void)
{
    info("Hello from module !");
}

static void cleanup(void)
{
    info("Goodbye...");
}

MODULE_INIT(startup)
MODULE_EXIT(cleanup)
