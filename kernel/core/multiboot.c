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
#include <multiboot.h>
#include <lib/string.h>

_init elf_shdr_t *mb_get_section(struct mb_info *mbi, char *name)
{
    elf_shdr_t *shdr_table = (elf_shdr_t *) mbi->elf_sec.addr;
    paddr_t shdr_name_table = (paddr_t) shdr_table[mbi->elf_sec.shndx].addr;

    for (uint32_t i = 0; i < mbi->elf_sec.size; i++) {
        const char *section_name = 
            (const char *) (shdr_name_table + shdr_table[i].name);
        if (strcmp(section_name, name) == 0)
            return shdr_table + i;
    }
    return NULL;
}
