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

#include <errno.h>
#include <config.h>
#include <assert.h>
#include <barrier.h>

typedef atomic_uint uatomic_t;
typedef atomic_int atomic_t;
typedef uint32_t vaddr_t;
typedef uint32_t paddr_t;
typedef uint32_t addr_t;

typedef unsigned int uint_t;

#define _assume_aligned(al) __attribute__((__assume_aligned__(al)))
#define _no_optimizations   __attribute__((optimize("-O0")))
#define _asmlinkage         __attribute__((regparm(0)))
#define _deprecated __attribute__((__deprecated__))
#define _inline     __attribute__((__always_inline__))
#define _no_inline  __attribute__((__noinline__))
#define _noreturn   __attribute__((__noreturn__))
#define _malloc     __attribute__((__malloc__))
#define _packed     __attribute__((__packed__))
#define _unused     __attribute__((__unused__))
#define _naked      __attribute__((__naked__))
#define _pure       __attribute__((__pure__))
#define _used       __attribute__((__used__))
#define _cdecl      __attribute__((__cdecl__))
#define _weak       __attribute__((__weak__, visibility("default")))

#define _cold   __attribute__((__cold__))
#define _hot    __attribute__((__hot__))

#define _align(al)      __attribute__((aligned(al)))
#define _section(name)  __attribute__((section(name)))
#define _unreachable()  __builtin_unreachable()

#define _export _used _cdecl _asmlinkage

#define _init       __attribute__((section(".init.text")))
#define _initdata   __attribute__((section(".init.data")))
#define _initrodata __attribute__((section(".init.rotdata")))

#define _interrupt  _cdecl _asmlinkage
#define _syscall    _cdecl _asmlinkage
#define _irq        _interrupt

#define assume_aligned(ptr, al) __builtin_assume_aligned(ptr, al)
#define unlikely(expr)          __builtin_expect(!!(expr), 0)
#define likely(expr)            __builtin_expect(!!(expr), 1)

#define container_of(ptr, type, member) \
    ((type *) ((char *) ptr - offsetof(type, member)))

#define BUG(x) _unreachable()

#include <lib/log.h>

// Some useful function definitions
_noreturn _cold void panic(const char *fmt, ...);
