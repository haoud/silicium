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

// List of x86 processor exceptions
#define EXCEPTION_COUNT 32
#define EXCEPTION_DIVIDE_ERROR 0
#define EXCEPTION_DEBUG 1
#define EXCEPTION_NMI 2
#define EXCEPTION_BREAKPOINT 3
#define EXCEPTION_OVERFLOW 4
#define EXCEPTION_BOUND 5
#define EXCEPTION_INVALID_OPCODE 6
#define EXCEPTION_DEVICE_NOT_AVAILABLE 7
#define EXCEPTION_DOUBLE_FAULT 8
#define EXCEPTION_COPROCESSOR_SEGMENT_OVERRUN 9
#define EXCEPTION_INVALID_TSS 10
#define EXCEPTION_SEGMENT_NOT_PRESENT 11
#define EXCEPTION_STACK_SEGMENT_FAULT 12
#define EXCEPTION_GENERAL_PROTECTION 13
#define EXCEPTION_PAGE_FAULT 14
#define EXCEPTION_RESERVED 15
#define EXCEPTION_FPU_ERROR 16
#define EXCEPTION_ALIGNMENT_CHECK 17
#define EXCEPTION_MACHINE_CHECK 18
#define EXCEPTION_SIMD_ERROR 19

#define install_exception(i) ({                 \
    extern void exception_##i(void);            \
    set_interrupt_gate(i, &exception_##i);      \
})

void exception_install(void);