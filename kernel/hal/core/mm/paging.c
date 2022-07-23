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
#include <core/mm/page.h>
#include <core/mm/paging.h>

_export void paging_set_directory(const paddr_t directory)
{
    set_cr3(directory);
}

/**
 * @brief Change the rights of a range of virtual pages.
 * @param start The start of the range.
 * @param end The end of the range.
 * @param access The access rights to set.
 * @return 0 on success, -1 on error.
 */
_export int paging_change_rights_interval(
    const vaddr_t start,
    const vaddr_t end,
    const int access)
{
    for (vaddr_t vaddr = start; vaddr < end; vaddr += PAGE_SIZE)
        if (paging_set_rights(vaddr, access) != 0)
            return -1;
    return 0;
}

/**
 * @brief Map an interval of virtual addresses
 * @param start The start of the interval.
 * @param end The end of the interval.
 * @param access The access rights of the mapped pages.
 * @return 0 on success, -1 on error.
 */
_export int paging_map_interval(
    const vaddr_t start,
    const vaddr_t end,
    const int access)
{
    for (vaddr_t vaddr = start; vaddr < end; vaddr += PAGE_SIZE) {
        const paddr_t page = page_alloc(PAGE_CLEAR);
        if (page == 0)
            return -1;
        if (paging_map_page(vaddr, page, access, PAGING_PRESENT) != 0)
            return -1;
    }
    return 0;
}

/**
 * @brief Unmap an interval of virtual addresses and free the associated
 * physical pages
 * 
 * @param start Start address of the interval to unmap.
 * @param end End address of the interval to unmap.
 * @return No return: all errors are discarded.
 */
_export void paging_unmap_interval(
    const vaddr_t start,
    const vaddr_t end)
{
    for (vaddr_t vaddr = start; vaddr < end; vaddr += PAGE_SIZE)
        page_free(paging_unmap_page(vaddr));
}
