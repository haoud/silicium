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
#include <core/elf.h>

elf_shdr_t *elf_get_section(const elf_ehdr_t *ehdr, const unsigned int idx)
{
    if (idx >= ehdr->shnum)
        return NULL;

    elf_shdr_t *shdr = (elf_shdr_t *) ((char *) ehdr + ehdr->shoff);
    return &shdr[idx];
}
