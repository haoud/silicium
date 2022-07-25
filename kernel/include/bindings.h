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

// Includes all the .h files
#include <lib/hashmap.h>
#include <lib/log.h>
#include <lib/list.h>
#include <lib/string.h>
#include <lib/spinlock.h>

#include <arch/x86/cpu.h>
#include <arch/x86/exception.h>
#include <arch/x86/gdt.h>
#include <arch/x86/idt.h>
#include <arch/x86/io.h>
#include <arch/x86/memory.h>
#include <arch/x86/paging.h>
#include <arch/x86/pic.h>
#include <arch/x86/pit.h>

#include <core/elf.h>
#include <core/module.h>
#include <core/symbol.h>
#include <core/ustar.h>

#include <mm/context.h>
#include <mm/malloc.h>
#include <mm/page.h>
#include <mm/paging.h>
#include <mm/slub.h>
#include <mm/vmalloc.h>
