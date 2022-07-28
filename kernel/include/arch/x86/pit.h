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

#define PIT_IRQ 0
#define PIT_TICK_NS 838
#define PIT_KERN_FREQ 100
#define PIT_INTERN_FREQ 1193180

#define PIT_KERN_LATCH (PIT_INTERN_FREQ / PIT_KERN_FREQ)

#define PIT_IO_CMD 0x43
#define PIT_IO_TIMER0 0x40
#define PIT_IO_TIMER1 0x41
#define PIT_IO_TIMER2 0x42

#define PIT_ACCESS_LOW 0x10
#define PIT_ACCESS_HIGH 0x20
#define PIT_ACCESS_LATCH 0x00
#define PIT_ACCESS_LOW_HIGH 0x30

#define PIT_FORMAT_BIN 0x00
#define PIT_FORMAT_BCD 0x01

#define PIT_CHANNEL0 0x00
#define PIT_CHANNEL1 0x40
#define PIT_CHANNEL2 0x80
#define PIT_CHANNEL_READ_BACK 0xC0

#define PIT_MODE_ONE_SHOT 0x02
#define PIT_MODE_SQUARE_WAVE 0x06
#define PIT_MODE_RATE_GENERATOR 0x04
#define PIT_MODE_SFW_TRIGGERED_STROBE 0x08
#define PIT_MODE_HDW_TRIGGERED_STROBE 0x08

_init void pit_configure(void);
uint32_t pit_nano_offset(void);
