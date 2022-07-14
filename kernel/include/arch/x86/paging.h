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
#include <arch/x86/memory.h>

#define KERNEL_BASE_PAGE (KERNEL_BASE >> PAGE_SHIFT)
#define KERNEL_BASE_PAGE_INDEX (KERNEL_BASE_PAGE >> 10)

// Mirroring
#define PAGING_MIRRORING_INDEX 1023
#define PAGING_MIRRORING_BASE 0xFFC00000
#define PAGING_MIRRORING_PD_MASK 0xFFC00000
#define PAGING_MIRRORING_PT_MASK 0X003FF000

#define mirroring(addr) (((uintptr_t)(addr) > PAGING_MIRRORING_BASE))

#define PAGING_NONE 0x00

// Maping access flags
#define PAGING_READ 0x01
#define PAGING_WRITE 0x02
#define PAGING_EXECUTE 0x04
#define PAGING_USER 0x08

// Mapping flags
#define PAGING_PRESENT 0x01
#define PAGING_GLOBAL 0x02

#define pd_offset(vaddr) (((vaddr) & 0xFFC00000) >> 22)
#define pt_offset(vaddr) (((vaddr) & 0x003FF000) >> 12)
#define pg_offset(vaddr) ((vaddr) & 0x00000FFF)
#define pde_index(vaddr) (((vaddr) >> 22) & 0x3FF)

// Page directory macros
#define pde_set_address(pde, paddr) ((pde)->address = ((paddr) >> 12))
#define pde_get_address(pte) ((pde)->address << 12)
#define pde_set(pde, value) ((pde)->value = value)
#define pde_clear(pde) ((pde)->value = 0)

// Page table macros
#define pte_set_address(pte, addr) ((pte)->address = ((addr) >> 12))
#define pte_get_address(pte) ((pte)->address << 12)
#define pte_set(pte, value) ((pde)->value = value)
#define pte_clear(pte) ((pte)->value = 0)

typedef uint32_t vaddr_t;
typedef uint32_t paddr_t;

typedef struct pde {
    union {
        struct {
            int present : 1;
            int write : 1;
            int user : 1;
            int write_through : 1;
            int cache_disable : 1;
            int accessed : 1;
            int reserved : 1;
            int large : 1;
            int available : 4;
            int address : 20;
        } _packed;
        uint32_t value;
    };
} _packed pde_t;

typedef struct pte {
    union {
        struct {
            int present : 1;
            int write : 1;
            int user : 1;
            int write_through : 1;
            int cache_disable : 1;
            int accessed : 1;
            int dirty : 1;
            int pat : 1;
            int global : 1;
            int available : 3;
            int address : 20;
        } _packed;
        uint32_t value;
    };
} _packed pte_t;

#define set_cr3(cr3) asm volatile("mov cr3, %0" :: "r"(cr3))
#define invlpg(vaddr) asm volatile("invlpg [%0]" :: "r"(vaddr) \
                                   : "memory")
#define flush_tlb(void)               \
    asm volatile("mov eax, cr3 \n"    \
                 "mov cr3, eax \n" :: \
                     : "eax")
#define get_cr2() ({           \
    vaddr_t x;                 \
    asm volatile("mov %0, cr2" \
                 : "=r"(x));   \
    x                          \
})

_init void paging_remap_kernel(void);

void paging_creat_pd(const vaddr_t pd);
paddr_t paging_get_paddr(const vaddr_t vaddr);

/* Paging interface */
_export int paging_set_rights(const vaddr_t vaddr, const int access);
_export int paging_set_flags(const vaddr_t vaddr, const int flags);
_export int paging_unmap_page(const vaddr_t vaddr);
_export int paging_rights(const vaddr_t vaddr);
_export int paging_flags(const vaddr_t vaddr);
_export int paging_map_page(
    const vaddr_t vaddr,
    const paddr_t paddr,
    const int access,
    const int flags);
