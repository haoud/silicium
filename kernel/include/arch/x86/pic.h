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

#define IRQ_BASE 32
#define IRQ_PER_PIC 8
#define PIC_TOTAL_IRQ 16

#define PIC_MASTER_CMD 0x20
#define PIC_MASTER_DATA 0x21

#define PIC_SLAVE_CMD 0xA0
#define PIC_SLAVE_DATA 0xA1

#define PIC_EOI 0x20

#define PIC_ICW1_NEED_ICW4 0x01
#define PIC_ICW1_SINGLE_MODE 0x02
#define PIC_ICW1_INTERVAL_4 0x04
#define PIC_ICW1_LEVEL 0x08
#define PIC_ICW1_INIT_REQUIRED 0x10

#define PIC_ICW4_8086 0x01
#define PIC_ICW4_EOI_AUTO 0x02
#define PIC_ICW4_BUFFER_SLAVE 0x08
#define PIC_ICW4_BUFFER_MASTER 0x0C
#define PIC_ICW4_SPECIAL_FULLY 0x10

void pic_remap(void);
void pic_enable_all(void);
void pic_disable_all(void);
void pic_enable(const uint32_t irq);
void pic_disable(const uint32_t irq);
void pic_send_eoi(const uint32_t irq);
