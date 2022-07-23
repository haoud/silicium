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
#include <lib/maths.h>
#include <lib/memory.h>
#include <mm/page.h>
#include <arch/x86/paging.h>

static pde_t kernel_pd[1024] _align(PAGE_SIZE);
extern const char _rodata_start, _rodata_end;
extern const char _data_start, _data_end;
extern const char _text_start, _text_end;
extern const char _init_start, _init_end;
extern const char _bss_start, _bss_end;

static _init void paging_map_page_helper(
    const vaddr_t vaddr,
    const paddr_t paddr,
    const int access,
    const int flags)
{
    pde_t *const pde = &kernel_pd[pd_offset(vaddr)];   
    if (!pde->present) {
        const paddr_t pt = page_alloc(PAGE_NONE);
        if (null(pt))
            panic("Failed to allocate a page");
        pde_set_address(pde, pt);
        pde->present = 1;
        pde->write = 1;
        pde->user = 0;
        memzero(pt, PAGE_SIZE);
    }

    pte_t *const pte = (pte_t *const) (
        pde_get_address(pde) +
        pt_offset(vaddr) *
        sizeof(pte_t));   

    if(pte->present)
        panic("Mapping page at 0x%08x: already mapped", vaddr);
    pte_set_address(pte, paddr);
    pte->write = !!(access & PAGING_WRITE);
    pte->present = 1;
}

static _init void paging_map_interval_helper(
    const vaddr_t vaddr,
    const paddr_t paddr,
    const int length,
    const int access,
    const int flags)
{
    for (int i = 0; i < length; i += PAGE_SIZE) {
        paging_map_page_helper(
            vaddr + i,
            paddr + i,
            access,
            flags);
    }
}

_init void paging_remap_kernel(void)
{
    memzero(kernel_pd, PAGE_SIZE);

    const vaddr_t bss_start = align((vaddr_t) &_bss_start, PAGE_SIZE);
    const vaddr_t data_start = align((vaddr_t) &_data_start, PAGE_SIZE);
    const vaddr_t init_start = align((vaddr_t) &_init_start, PAGE_SIZE);
    const vaddr_t text_start = align((vaddr_t) &_text_start, PAGE_SIZE);
    const vaddr_t rodata_start = align((vaddr_t) &_rodata_start, PAGE_SIZE);

    const int bss_length = (vaddr_t) &_bss_end - bss_start;
    const int data_length = (vaddr_t) &_data_end - data_start;
    const int init_length = (vaddr_t) &_init_end - init_start;
    const int text_length = (vaddr_t) &_text_end - text_start;
    const int rodata_length = (vaddr_t) &_rodata_end - rodata_start;

    // Identity map the first 3 GO
    for (unsigned int i = 0; i < pd_offset(KERNEL_BASE); i++) {
        pde_set_address(&kernel_pd[i], i << 22);
        kernel_pd[i].present = 1;
        kernel_pd[i].write = 1;
        kernel_pd[i].large = 1;
    }

    // Map the .text segment
    paging_map_interval_helper(
        text_start,
        text_start - KERNEL_BASE,
        text_length,
        PAGING_EXECUTE,
        PAGING_PRESENT);
       
    // Map the .rodata segment
    paging_map_interval_helper(
        rodata_start,
        rodata_start - KERNEL_BASE,
        rodata_length,
        PAGING_READ,
        PAGING_PRESENT);

    // Map the .data segment
    paging_map_interval_helper(
        data_start,
        data_start - KERNEL_BASE,
        data_length,
        PAGING_READ | PAGING_WRITE,
        PAGING_PRESENT);

     // Map the .init segment
    paging_map_interval_helper(
        init_start,
        init_start - KERNEL_BASE,
        init_length,
        PAGING_EXECUTE | PAGING_READ | PAGING_WRITE,
        PAGING_PRESENT);

    // Map the .bss segment
   paging_map_interval_helper(
        bss_start,
        bss_start - KERNEL_BASE,
        bss_length,
        PAGING_READ | PAGING_WRITE,
        PAGING_PRESENT);

    // Mirroring
    const paddr_t kernel_pd_paddr = (paddr_t) kernel_pd - KERNEL_BASE;
    pde_set_address(&kernel_pd[PAGING_MIRRORING_INDEX], kernel_pd_paddr);
    kernel_pd[PAGING_MIRRORING_INDEX].present = 1;
    kernel_pd[PAGING_MIRRORING_INDEX].write = 1;

    // Set CR3 to the new page directory
    set_cr3(kernel_pd_paddr);

    // Preallocate all kernel page directory entries
    for (uint_t i = pd_offset(KERNEL_BASE); i < 1023; i++) {
        if(kernel_pd[i].present)
            continue;

        const paddr_t page = page_alloc(PAGE_CLEAR);
        pde_set_address(&kernel_pd[i], page);
        kernel_pd[i].present = 1;
        kernel_pd[i].write = 1;
    }
    flush_tlb();
}

_init void paging_clear_userspace(void)
{
    for (uint_t i = 0; i < pd_offset(KERNEL_BASE); i++)
        pde_clear(&kernel_pd[i]);
}

/**
 * @brief Get the page direction entry for a given address in the current 
 * address space
 * 
 * @param addr Address to get the page directory entry for
 * @return pde_t* The page directory entry for the given address
 */
pde_t *paging_get_pde(const vaddr_t addr) 
{
    // Magic
    return (pde_t *) (PAGING_MIRRORING_BASE 
        + (pd_offset(PAGING_MIRRORING_BASE) << PAGE_SHIFT)
        + (pd_offset(addr) << 2));
}

/**
 * @brief Get the page table entry for a given address in the current
 * address space
 * 
 * @param addr Address to get the page table entry for
 * @return pte_t* The page table entry for the given address, or NULL
 * if the entry is not present.
 */
pte_t *paging_get_pte(const vaddr_t addr)
{
    // More magic
    if (!paging_get_pde(addr)->present)
        return NULL;
    return (pte_t *) (PAGING_MIRRORING_BASE 
        + (pd_offset(addr) << PAGE_SHIFT)
        + (pt_offset(addr) << 2));
}

/**
 * @brief Get the physique address of a virtual address in the current
 * address space
 * 
 * @param vaddr Address to get the physical address for, should be aligned
 * on a page boundary
 * @return paddr_t Physical address mapped to the given virtual address,
 * or 0 if address is not mapped
 */
paddr_t paging_get_paddr(const vaddr_t vaddr)
{
	const pte_t *pte = paging_get_pte(vaddr);
	if (pte == NULL || !pte->present)
		return 0;
	return pte_get_address(pte) + pg_offset(vaddr);
}

/**
 * @brief Create a copy of a page directory. The page directory entries are
 * copied, but the page tables are not: There are marked as not present and
 * as not writable: this allow to do Copy-on-Write
 * 
 * @param src The virtual address of the source page directory. Must be aligned
 * on a page boundary
 * @param dst The virtual address of the destination page directory. Must be
 * aligned on a page boundary
 */
void paging_clone_pd(const vaddr_t src, const vaddr_t dst)
{
    paging_creat_pd(dst);
    pde_t *const s = (pde_t *) src;
    pde_t *const d = (pde_t *) dst;
    for (uint_t i = 0; i < pd_offset(KERNEL_BASE); i++) {
        page_reference(pde_get_address(&s[i]));
        s[i].present = s[i].write = 0;
        pde_copy(&d[i], &s[i]);
    }
}

void paging_destroy_userspace(void)
{
    for (uint_t i = 0; i < pd_offset(KERNEL_BASE); i++) {
        const pde_t *pde = paging_get_pde(i << 22);
        const paddr_t pt_paddr = pde_get_address(pde);
        if (!pde->present)
            continue;

        // If the page table is referenced several times, we do
        // not release the pages it contains because it is still 
        // used by other processes
        page_lock(pt_paddr);
        if (page_counter(pt_paddr) == 1) {
            for (int j = 0; j < 1024; j++) {
                const pte_t *pte = paging_get_pte(i << 22 | j << 12);
                if (pte->present)
                    continue;
                page_free(pte_get_address(pte));
            }
        }
        page_unlock(pt_paddr);
        page_free(pt_paddr);
    }
}

void paging_creat_pd(const vaddr_t dst)
{
    pde_t *pd = (pde_t *) dst;
    memzero(pd, PAGE_SIZE);
    pd_set_mirroring(pd);
}

void paging_set_pd(const vaddr_t pd)
{
    set_cr3(paging_get_paddr(pd));
}


/**
 * @brief Load the kernel page directory into the current address space
 * This function is useful when a thread is destroyed and its page directory
 * must be freed: This function can be called to load the kernel page 
 * directory and free the thread's page directory without problems.
 */
void paging_use_kernel_pd(void)
{
    paging_set_pd((vaddr_t) kernel_pd);
}

/**
 * @brief Map a physical address to a virtual address in the current address
 * space
 * 
 * TODO: Reduce invlpg count (invlpg is expensive and called multiple times
 * when calling paging_set_rights() and paging_set_acess()) and verify if we
 * use invlpg correctly
 * 
 * @param vaddr Where to map the physical address
 * @param paddr Physical address to map
 * @param access Access rights of the mapping
 * @param flags Flags for the mapping
 * @return 0 on success, or -1 if there are not enought memory 
 */
_export int paging_map_page(
    const vaddr_t vaddr,
    const paddr_t paddr,
    const int access,
    const int flags)
{
    assert(!mirroring(vaddr));
    assert(!null(vaddr));
    assert(!null(paddr));
    pde_t *const pde = paging_get_pde(vaddr);   
    const bool user = (vaddr < KERNEL_BASE);
    if (!pde->present) {
        const paddr_t pt = page_alloc(PAGE_CLEAR);
        if (null(pt))
            return -1;
        pde_set_address(pde, pt);
        pde->present = 1;
        pde->user = user;
        pde->write = 1;
        invlpg((vaddr_t) paging_get_pte(vaddr));
    }

    pte_t *const pte = paging_get_pte(vaddr);   
    if(pte->present)
        panic("Mapping page at 0x%08x: already mapped", vaddr);
    paging_set_rights(vaddr, access);
    paging_set_flags(vaddr, flags);
    pte_set_address(pte, paddr);
    invlpg(vaddr);
    return 0;
}

/**
 * @brief Set access rights of a virtual address in the current address space
 * 
 * @param vaddr Address to set the access rights for
 * @param access Access rights to set
 * @return 0 on success, or -1 if the address is not mapped
 */
_export int paging_set_rights(const vaddr_t vaddr, const int access)
{
    pte_t *const pte = paging_get_pte(vaddr);
    if (pte == NULL)
        return -1;
    pte->write = pte->user = 0;
    if (access & PAGING_WRITE)
        pte->write = 1;
    if (access & PAGING_USER)
        pte->user = 1;
    invlpg(vaddr);
    return 0;
}

/**
 * @brief Set the flags of a virtual address in the current address space
 * 
 * @param vaddr Address to set the flags for
 * @param flags Flags to set
 * @return 0 on success, or -1 if the address is not mapped
 */
_export int paging_set_flags(const vaddr_t vaddr, const int flags)
{
    pte_t *const pte = paging_get_pte(vaddr);
    if (pte == NULL)
        return -1;
    pte->present = pte->global = 0;
    if (flags & PAGING_PRESENT)
        pte->present = 1;
    if (flags & PAGING_GLOBAL)
        pte->global = 1;
    invlpg(vaddr);
    return 0;
}

/**
 * @brief Get access rights of a virtual address in the current address space
 * 
 * @param vaddr Address to get the access rights for
 * @return Access rights of the address, or PAGING_NONE if the address is not
 * mapped or does not have access rights set
 */
_export int paging_rights(const vaddr_t vaddr)
{
    // Get the access rights of the page
    const pte_t *const pte = paging_get_pte(vaddr);
    if (pte == NULL)
        return PAGING_NONE;
    int flags = PAGING_EXECUTE | PAGING_WRITE;
    if (pte->write)
        flags |= PAGING_WRITE;
    if (pte->user)
        flags |= PAGING_USER;
    return flags;
}

/**
 * @brief Get the flags of a virtual address in the current address space
 * 
 * @param vaddr Address to get the flags for
 * @return Flags of the address, or PAGING_NONE if the address has no flags set
 */
_export int paging_flags(const vaddr_t vaddr)
{
    // Get the flags of the page at the given address
    const pte_t *const pte = paging_get_pte(vaddr);
    if (pte == NULL)
        return PAGING_NONE;
    int flags = PAGING_NONE;
    if (pte->present)
        flags |= PAGING_PRESENT;
    if (pte->global)
        flags |= PAGING_GLOBAL;
    return flags;
}

/**
 * @brief Unmap a virtual address in the current address space
 * 
 * @param vaddr Address to unmap
 * @return Physical address of the page that was unmapped, or 0 if the address
 * is not mapped
 */
_export int paging_unmap_page(const vaddr_t vaddr)
{
    assert(!mirroring(vaddr));
    assert(!null(vaddr));

    // Unmap the page at the given address
    pte_t *const pte = paging_get_pte(vaddr);
    if (pte == NULL || !pte->present)
        return 0;

    // Allocated page tables are never freed by simplicity
    // Is it useful to do so ?
    const paddr_t page_addr = pte_get_address(pte);
    pte_clear(pte);
    invlpg(vaddr);
    return page_addr;
}
