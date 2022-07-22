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
#include <core/elf.h>

#define MB_HEADER_MAGIC 0x1BADB002
#define MB_BOOTLOADER_MAGIC 0x2BADB002
#define MB_STACK_SIZE 0x4000

#define MB_INFO_MEMORY 0x00000001
#define MB_INFO_BOOTDEV 0x00000002
#define MB_INFO_CMDLINE 0x00000004
#define MB_INFO_MODS 0x00000008
#define MB_INFO_AOUT_SYMS 0x00000010
#define MB_INFO_ELF_SHDR 0x00000020
#define MB_INFO_MEM_MAP 0x00000040
#define MB_INFO_DRIVE_INFO 0x00000080
#define MBT_INFO_CONFIG_TABLE 0x00000100
#define MB_INFO_BOOT_LOADER_NAME 0x00000200
#define MB_INFO_APM_TABLE 0x00000400
#define MB_INFO_VBE_INFO 0x00000800
#define MB_INFO_FRAMEBUFFER_INFO 0x00001000

#define MB_FRAMEBUFFER_TYPE_INDEXED 0
#define MB_FRAMEBUFFER_TYPE_RGB 1
#define MB_FRAMEBUFFER_TYPE_EGA_TEXT 2

#define MB_MEMORY_AVAILABLE 1
#define MB_MEMORY_RESERVED 2
#define MB_MEMORY_ACPI_RECLAIMABLE 3
#define MB_MEMORY_NVS 4
#define MB_MEMORY_BADRAM 5

struct mb_header
{
    uint32_t magic;
    uint32_t flags;
    uint32_t checksum;
    uint32_t header_addr;
    uint32_t load_addr;
    uint32_t load_end_addr;
    uint32_t bss_end_addr;
    uint32_t entry_addr;
} _packed;

struct mb_elf_table
{
    uint32_t num;
    uint32_t size;
    uint32_t addr;
    uint32_t shndx;
} _packed;

struct mb_module
{
    uint32_t mod_start;
    uint32_t mod_end;
    uint32_t string;
    uint32_t reserved;
} _packed;

struct mb_mmap
{
    uint32_t size;
    uint64_t addr;
    uint64_t len;
    uint32_t type;
} _packed;

struct mb_info
{
    uint32_t flags;
    uint32_t mem_lower;
    uint32_t mem_upper;
    uint32_t boot_device;
    uint32_t cmdline;
    uint32_t mods_count;
    uint32_t mods_addr;

    struct mb_elf_table elf_sec;

    uint32_t mmap_length;
    struct mb_mmap *mmap_addr;

    uint32_t drives_length;
    uint32_t drives_addr;

    uint32_t config_table;

    uint32_t boot_loader_name;

    uint32_t apm_table;

    uint32_t vbe_control_info;
    uint32_t vbe_mode_info;
    uint16_t vbe_mode;
    uint16_t vbe_interface_seg;
    uint16_t vbe_interface_off;
    uint16_t vbe_interface_len;

    uint64_t fb_addr;
    uint32_t fb_pitch;
    uint32_t fb_width;
    uint32_t fb_height;
    uint8_t fb_bpp;
    uint8_t fb_type;

    union {
        struct {
            uint32_t fb_palette_addr;
            uint16_t fb_palette_num_colors;
        } palette;
        struct {
            uint8_t fb_red_field_position;
            uint8_t fb_red_mask_size;
            uint8_t fb_green_field_position;
            uint8_t fb_green_mask_size;
            uint8_t fb_blue_field_position;
            uint8_t fb_blue_mask_size;
        } color_mask;
    } fb_data;
} _packed;

_init struct mb_module *mb_get_module(struct mb_info *mbi, char *name);
_init elf_shdr_t *mb_get_section(struct mb_info *mbi, char *name);
