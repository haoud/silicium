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
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <stdatomic.h>

#include <assert.h>
#include <lib/log.h>

typedef atomic_uint uatomic_t;
typedef atomic_int atomic_t;
typedef uint32_t vaddr_t;
typedef uint32_t paddr_t;

typedef unsigned int uint_t;

#define _always_inline inline __attribute__((__always_inline__))
#define _no_optimizations __attribute__((optimize("-O0")))
#define _deprecated __attribute__((deprecated))
#define _no_inline __attribute__((noinline))
#define _noreturn __attribute__((noreturn))
#define _malloc __attribute__((malloc))
#define _packed __attribute__((packed))
#define _unused __attribute__((unused))
#define _naked __attribute__((naked))
#define _pure __attribute__((pure))
#define _used __attribute__((used))
#define _cdecl __attribute__((cdecl))
#define _weak __attribute__((weak, visibility("default")))
#define _asmlinkage __attribute__((regparm(0)))

#define _align(al) __attribute__((aligned(al)))
#define _section(name) __attribute__((section(name)))
#define _unreachable() __builtin_unreachable()

#define _export _used _cdecl _asmlinkage

#define _init __attribute__((section(".init.text")))
#define _initdata __attribute__((section(".init.data")))
#define _initrodata __attribute__((section(".init.rotdata")))

#define _interrupt _cdecl _asmlinkage
#define _syscall _cdecl _asmlinkage
#define _irq _interrupt

#define likely(expr) __builtin_expect(!!(expr), 1)
#define unlikely(expr) __builtin_expect(!!(expr), 0)
#define assume_aligned(ptr, al) __builtin_assume_aligned(ptr, al)

#define container_of(ptr, type, member) \
    ((type *)((char *)ptr - offsetof(type, member)))

#define BUG(x) _unreachable()

// Some useful function definitions
_noreturn void panic(const char *fmt, ...);