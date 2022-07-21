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
#include <lib/list.h>
#include <multiboot.h>
#include <lib/spinlock.h>
#include <arch/x86/memory.h>

// Page alloc flags
#define PAGE_NONE 0x00 // No flags
#define PAGE_BIOS 0x01
#define PAGE_ISA 0x02
#define PAGE_CLEAR 0x04

#define page_index_to_address(index) ((index) << PAGE_SHIFT)
#define page_address_to_index(address) ((address) >> PAGE_SHIFT)
#define page_use_interval(start, end)    \
    for (paddr_t addr = (paddr_t) start; \
         addr < (paddr_t)(end);          \
         addr += PAGE_SIZE) {            \
        page_use(addr);                  \
    }
#define page_use_area(start, length)          \
    for (paddr_t addr = (paddr_t) start;      \
         addr < (paddr_t) (start) + (length); \
         addr += PAGE_SIZE) {                 \
        page_use(addr);                       \
    }

typedef struct page_info {
    struct list_head entry;
    struct spinlock lock;
    atomic_t count;
    uint32_t index;         // Index of the page
    union {
        uint32_t flags;
        struct {
            int reserved : 1;
            int cleared : 1;
            int bios : 1;
            int isa: 1;
            int unused : 28;
        }_packed;
    };
} page_info_t;

typedef struct page_table_info {
    struct page_info *pages;
    size_t nb_pages;
} page_table_info_t;

_init void page_map_table(void);
_init void page_use(const paddr_t addr);
_init void page_setup(struct mb_info *info);
_init void page_reserve(const paddr_t page);

/* Pages allocation interface */
_export void page_reference(const paddr_t addr);
_export int page_counter(const paddr_t addr);
_export paddr_t page_alloc(const int flags);
_export void page_free(const paddr_t addr);
_export int page_unlock(const paddr_t addr);
_export int page_lock(const paddr_t addr);
