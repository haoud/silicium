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
#include <lib/string.h>
#include <core/ustar.h>
#include <core/mm/malloc.h>

/**
 * @brief This is a simple implementation of the Ustar format. It
 * is currently very incomplete and only used to load an initrd.
 */

/**
 * @brief Convert a string containing an octal number to a decimal unsigned
 * int. If the string is not an octal number or if the string contains
 * non-numeric characters, the behavior is undefined.
 * 
 * @param str The string to convert
 * @param size The size of the string
 * @return unsigned int The converted number
 */
static unsigned int oct2bin(const char *str, int size) 
{
    unsigned int n = 0;
    while (size-- > 0)
        n = n * 8 + *str++ - '0';
    return n;
}

/**
 * @brief Search a file in a tar archive.
 * 
 * @param archive The archive to read: must be entirely in memory.
 * @param name The name of the file to find in the archive.
 * @return struct ustar_header* The header of the file 
 * @return NULL if the file is not found or if the memory allocation has failed 
 */
struct ustar_entry *ustar_lookup(char *archive, const char *name)
{
    const size_t name_length = strlen(name);
    while (memcmp(archive + 257, "ustar", 5) == 0) {
        unsigned int length = oct2bin(archive + 0x7C, 11);
        if (memcmp(archive, name, name_length + 1) == 0) {
            struct ustar_entry *entry = malloc(sizeof(struct ustar_entry));
            if (entry == NULL)
                return NULL;
            entry->data = archive + 512;
            entry->length = length;
            return entry;
        }
        archive = archive + 512 + align(length, 512);
    }
    return NULL;
}
