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

#define ELF_NIDENT          16
#define ELF_INVALID_SYMBOL  0xFFFFFFFF

#define ELF_IDENT_MAGIC0        0
#define ELF_IDENT_MAGIC1        1
#define ELF_IDENT_MAGIC2        2
#define ELF_IDENT_MAGIC3        3
#define ELF_IDENT_CLASS         4
#define ELF_IDENT_DATA          5
#define ELF_IDENT_VERSION       6
#define ELF_IDENT_OS_ABI        7
#define ELF_IDENT_ABI_VERSION   8
#define ELF_IDENT_PAD           9

#define ELF_MAGIC0  0x7F
#define ELF_MAGIC1  'E'
#define ELF_MAGIC2  'L'
#define ELF_MAGIC3  'F'

#define ELF_DATA_LSB    1  // Little endian
#define ELF_CLASS32     1   // 32-bit architecture

#define ELF_TYPE_NONE   0
#define ELF_TYPE_REL    1
#define ELF_TYPE_EXEC   2
#define ELF_TYPE_DYN    3
#define ELF_TYPE_CORE   4

#define EM_386          3   // x86 machine
#define EV_CURRENT      1   // ELF Current Version

#define ELF_SHN_UNDEF   0

// Program table entry types
#define ELF_PT_NULL     0
#define ELF_PT_LOAD     1
#define ELF_PT_DYNAMIC  2
#define ELF_PT_INTERP   3
#define ELF_PT_NOTE     4
#define ELF_PT_SHLIB    5
#define ELF_PT_PHDR     6

// Elf special section header indices
#define ELF_SHN_LORESERVE   0xFF00
#define ELF_SHN_BEFORE      0xFF00
#define ELF_SHN_LOPROC      0xFF00
#define ELF_SHN_AFTER       0xFF01
#define ELF_SHN_HIPROC      0xFFF1
#define ELF_SHN_ABS         0xFFF1
#define ELF_SHN_COMMON      0xFFF2
#define ELF_SHN_RESERVED    0xFFFF

#define ELF_SHT_TYPE_NULL       0
#define ELF_SHT_TYPE_PROGBITS   1
#define ELF_SHT_TYPE_SYMTAB     2
#define ELF_SHT_TYPE_STRTAB     3
#define ELF_SHT_TYPE_RELA       4
#define ELF_SHT_TYPE_HASH       5
#define ELF_SHT_TYPE_DYNAMIC    6
#define ELF_SHT_TYPE_NOTE       7
#define ELF_SHT_TYPE_NOBITS     8
#define ELF_SHT_TYPE_REL        9
#define ELF_SHT_TYPE_SHLIB      10
 
#define ELF_SHT_ATTRIB_WRITE    0x01
#define ELF_SHT_ATTRIB_ALLOC    0x02
#define ELF_SHT_ATTRIB_EXECUTE  0x04

#define ELF_ST_BIND(info)   ((info) >> 4)
#define ELF_ST_TYPE(info)   ((info) & 0x0F)

#define ELF32_R_SYM(info)   ((info) >> 8)
#define ELF32_R_TYPE(info)  ((info) & 0xFF)

#define ELF_STB_LOCAL   0
#define ELF_STB_GLOBAL  1
#define ELF_STB_WEAK    2

#define ELF_STT_NOTYPE  0
#define ELF_STT_OBJECT  1
#define ELF_STT_FUNC    2
#define ELF_STT_SECTION 3
#define ELF_STT_FILE    4

#define ELF_STV_DEFAULT     0
#define ELF_STV_INTERNAL    1
#define ELF_STV_HIDDEN      2
#define ELF_STV_PROTECTED   3

#define ELF_RTT_NONE    0
#define ELF_RTT_32      1
#define ELF_RTT_PC32    2

#define elf_section_entry_count(section)  \
    ((section)->size / (section)->entsize)

typedef uint16_t elf_half_t;
typedef uint32_t elf_addr_t;
typedef uint32_t elf_word_t;
typedef int32_t elf_sword_t;
typedef uint32_t elf_off_t;

typedef struct elf_ehdr {
    uint8_t ident[ELF_NIDENT];
    elf_half_t type;
    elf_half_t machine;
    elf_word_t version;
    elf_addr_t entry;
    elf_off_t phoff;
    elf_off_t shoff;
    elf_word_t flags;
    elf_half_t ehsize;
    elf_half_t phentsize;
    elf_half_t phnum;
    elf_half_t shentsize;
    elf_half_t shnum;
    elf_half_t shstrndx;
}_packed elf_ehdr_t;

typedef struct elf_phdr {
    elf_word_t type;
    elf_off_t offset;
    elf_addr_t vaddr;
    elf_addr_t paddr;
    elf_word_t filesz;
    elf_word_t memsz;
    elf_word_t flags;
    elf_word_t align;
}_packed elf_phdr_t;

typedef struct elf_shdr {
    elf_word_t name;
    elf_word_t type;
    elf_word_t flags;
    elf_addr_t addr;
    elf_off_t offset;
    elf_word_t size;
    elf_word_t link;
    elf_word_t info;
    elf_word_t addralign;
    elf_word_t entsize;
}_packed elf_shdr_t;

typedef struct elf_sym {
    elf_word_t name;
    elf_addr_t value;
    elf_word_t size;
    uint8_t info;
    uint8_t other;
    elf_half_t shndx;
}_packed elf_sym_t;

typedef struct elf_rel {
    elf_addr_t offset;
    elf_word_t info;
}_packed elf_rel_t;

typedef struct elf_rela {
    elf_addr_t offset;
    elf_word_t info;
    elf_sword_t addend;
}_packed elf_rela_t;

elf_shdr_t *elf_get_section(const elf_ehdr_t *ehdr, const unsigned int idx);
