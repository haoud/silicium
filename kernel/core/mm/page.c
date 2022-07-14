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
#include <pointer.h>
#include <multiboot.h>
#include <lib/maths.h>
#include <lib/memory.h>
#include <lib/spinlock.h>
#include <core/mm/page.h>
#include <arch/x86/paging.h>

/**
 * @file The algorythme system used here is based on linked-list.
 * One of advantage of this system is that it is very easy to implement.
 * The other advantage is that it one of the fastest algorythme (O(1) in
 * alloc/free).
 * 
 * The main drawback is that it is not trivial to allocate severals
 * contiguous pages (complexity is O(n) in worst case) and leads to high
 * fragmentation. 
 * But for my kernel, it is not a problem because I don't need to allocate 
 * contiguous pages :)
 * 
 * TODO: Fix potential concurrency issues by using a lock or an atomic counter
 */
static struct page_table_info table;
static DECLARE_LIST(bios_free_list);
static DECLARE_LIST(isa_free_list);
static DECLARE_LIST(free_list);
static DECLARE_SPINLOCK(lock);

extern const char _end;
static const vaddr_t end = (vaddr_t) &_end;

/**
 * @brief Execute a function for each memory area passed by GRUB. This function
 * discard any entry that is above 4GB.
 * 
 * @param start First memory area
 * @param length  Length of the memory areas structures
 * @param function Function to execute for each memory area
 */
static _init void for_each_mmap(
    const struct mb_mmap *start, 
    const size_t length, 
    void (*function) (const struct mb_mmap *const))
{
    const struct mb_mmap *last = ptr_add(start, length);
    for (const struct mb_mmap *mmap = start;
        mmap < last;
        mmap = ptr_add(ptr_add(mmap, mmap->size), sizeof(mmap->size))) {
        // Discard invalid entries
        if (mmap->addr > 0xFFFFFFFF ||
            mmap->len > 0xFFFFFFFF)
            continue;
        if (mmap->addr + mmap->len - 1 > 0xFFFFFFFF)
            continue;
       function(mmap);
    }
}

/**
 * @brief Calculate the number of entries needed in the table array
 * @param entry Memory area
 */
static _init void page_nb_page(const struct mb_mmap *const entry) 
{
    static paddr_t last_page = 0;
    if (entry->addr + entry->len > last_page) {
        last_page = entry->addr + entry->len - 1;
        table.nb_pages = last_page / PAGE_SIZE;
    }
}

/**
 * @brief Calculate the location of the array in order to no collide with kernel
 * @param entry Memory area to check
 */
static _init void page_array_location(const struct mb_mmap *const entry) 
{
    // The array are already allocated 
    if (table.pages)
        return;

    // TODO: Find a better location
    table.pages = (void *) (end - KERNEL_BASE);
}

/**
 * @brief Mark avaible memory areas as free for the page allocator.
 * @param entry Memory area to check
 */
static _init void page_mark_free_area(const struct mb_mmap *const entry)
{
    if (entry->type != MB_MEMORY_AVAILABLE)
        return;

    for (paddr_t addr = entry->addr;
        addr < entry->addr + entry->len;
        addr += PAGE_SIZE) {
        table.pages[page_address_to_index(addr)].reserved = 0;
    }
}

static _init void page_construct_lists(void)
{
    for (size_t i = 0; i < table.nb_pages; i++) {
        list_entry_init(&table.pages[i].entry);
        if (table.pages[i].reserved || table.pages[i].count)
            continue;
        else if (table.pages[i].bios) 
            list_add_tail(&bios_free_list, &table.pages[i].entry);
        else if (table.pages[i].isa)
            list_add_tail(&isa_free_list, &table.pages[i].entry);
        else 
            list_add_tail(&free_list, &table.pages[i].entry);
    }
}

_init void page_map_table(void)
{
    const vaddr_t length = table.nb_pages * sizeof(page_info_t);
    const paddr_t array = (const paddr_t) table.pages;
    const vaddr_t start = end;
    for (vaddr_t i = 0; i < length; i += PAGE_SIZE) {
        paging_map_page(
            start + i,
            array + i,
            PAGING_READ | PAGING_WRITE,
            PAGING_PRESENT);
    }

    table.pages = (page_info_t *) start;
    // Rebuild linked lists
    list_init(&bios_free_list);
    list_init(&isa_free_list);
    list_init(&free_list);
    page_construct_lists();
}

/**
 * @brief Initialise the page allocator.
 * 
 * @param info Multiboot info structure
 * @param reserved_memory A list of memory areas reserved by the kernel 
 * (or by the hardware)
 * @return The number of physical pages of the memory 
 */
_init void page_setup(struct mb_info *info)
{
    for_each_mmap(info->mmap_addr, info->mmap_length, page_nb_page);
    for_each_mmap(info->mmap_addr, info->mmap_length, page_array_location);
    
    // Hum...
    if (!table.pages)
        panic("Not enough memory to allocate the page array");  

    // Initialize page info array
    for (size_t i = 0; i < table.nb_pages; i++) {
        table.pages[i].reserved = 1;
        table.pages[i].count = 0;
        table.pages[i].flags = 0;
        table.pages[i].index = i;
        if (i < page_address_to_index(0x100000))
            table.pages[i].bios = 1;
        if (i < page_address_to_index(0x1000000))
            table.pages[i].isa = 1;
    }

    for_each_mmap(info->mmap_addr, info->mmap_length, page_mark_free_area);
    page_construct_lists();

    // Yeeep ! We can allocate pages now
    // TODO: reserve memory used by modules
    page_reserve_interval(0, PAGE_SIZE);
    page_reserve_interval(0x100000, end - KERNEL_BASE);
    page_reserve_area(table.pages, table.nb_pages * sizeof(page_info_t));
}

static struct page_info *page_get(paddr_t paddr)
{
    if (paddr >= table.nb_pages * PAGE_SIZE)
        return NULL;
    return &table.pages[page_address_to_index(paddr)];
}

/**
 * @brief Clear a physical page with zeros
 * 
 * @param paddr Address of the page to clear, must be aligned on PAGE_SIZE
 */
static void page_clear(paddr_t paddr)
{
    static char buffer[PAGE_SIZE] _align(PAGE_SIZE);

    paging_unmap_page((vaddr_t) buffer);
    paging_map_page((vaddr_t) buffer, paddr, PAGING_WRITE, PAGING_PRESENT);
    memset(buffer, 0, PAGE_SIZE);
}

/**
 * @brief Mark a page as reserved (cannot be allocated)
 * @param page Address of the page
 */
_export paddr_t page_reserve(const paddr_t page)
{
    const size_t index = page_address_to_index(page);
    if (index >= table.nb_pages)
        panic("Page %p is out of range and cannot be reserved", page);
    if (table.pages[index].count)
        panic("Page %p is used and cannot be reserved", page);

    list_remove(&table.pages[index].entry);
    table.pages[index].reserved = 1;
    return page;
}

/**
 * @brief Get the value of the reference counter of a page
 * @param addr Address of the page
 * @return The value of the reference counter, or -1 if the page doesn't have 
 * a reference counter (i.e. doesn't exist or are reserved)
 */
_export int page_get_counter(const paddr_t addr)
{
    const page_info_t *const page = page_get(PAGE_ALIGN(addr));
    if (page == NULL || page->reserved)
        return -1;
    return page->count;
}

/**
 * @brief Allows to free a page marked as reserved.I hope you know what
 * you are doing or the whole system will explode horribly !
 * However, this function cannot free a page that is higher than the last
 * page of free real RAM, because it is not included in the page information
 * table.
 * @param addr Address of the page to unreserve
 * @return int 0 on success, -1 on failure
 */
_export int page_unreserve(const paddr_t addr)
{
    page_info_t *const page = page_get(PAGE_ALIGN(addr));
    if (page == NULL || !page->reserved)
        return -1;

    spin_lock(&lock);
    page->reserved = 0;
    list_entry_init(&page->entry);
    if (page->bios)
        list_add_head(&page->entry, &bios_free_list);
    else if (page->isa)
        list_add_head(&page->entry, &isa_free_list);
    else
        list_add_head(&page->entry, &free_list);
    spin_unlock(&lock);
    return 0;
}

/**
 * Incremente the reference counter of a page.
 * @param page The physical address of the page.
 * @return The physical address of the page, aligned to PAGE_SIZE.
 */
_export paddr_t page_reference(const paddr_t addr)
{
    const paddr_t paddr = PAGE_ALIGN(addr);
    page_info_t *const page = page_get(paddr);
    if (page->count == 0)
        panic("Trying to reference a free page");
    page->count++;
    return paddr;
}

/**
 * Allocation a page and return the address of the allocated page.
 * @param flags Flags 
 * @return The physical address of the allocated page.
 */
_export paddr_t page_alloc(const int flags)
{
    spin_lock(&lock);
    struct list_head *list = &free_list;
    if (flags & PAGE_ISA || list_empty(list))
        list = &isa_free_list;
    if (flags & PAGE_BIOS || list_empty(list))
        list = &bios_free_list;
    if (list_empty(list)) {
        error("No free pages");
        spin_unlock(&lock);
        return 0;
    }

    page_info_t *const page = container_of(list->next, page_info_t, entry);
    const paddr_t paddr = page_index_to_address(page->index);
    list_remove(&page->entry);
    spin_unlock(&lock);

    if (flags & PAGE_CLEAR && !page->cleared)
        page_clear(paddr);
    page->cleared = 0;
    page->count = 1;
    return paddr;
}

/**
 * Decremente the reference counter of a page and free it if the reference
 * counter is 0.
 * @param addr The physical address of the page to free
 */
_export void page_free(const paddr_t addr)
{
    page_info_t *const page = page_get(PAGE_ALIGN(addr));
    if (page->count == 0) 
        panic("Trying to free a page that is already free");
    if (page->reserved)
        panic("Trying to free a reserved page");

    if (--page->count == 0) {
        list_remove(&page->entry);
        if (page->bios)
            list_add_head(&page->entry, &bios_free_list);
        else if (page->isa)
            list_add_head(&page->entry, &isa_free_list);
        else
            list_add_head(&page->entry, &free_list);
    }
}
