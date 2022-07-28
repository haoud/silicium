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

#define CMOS_IO_ADDRESS     0x70
#define CMOS_IO_DATA        0x71

#define CMOS_REG_SECONDS            0x00
#define CMOS_REG_SECONDS_ALARM      0x01
#define CMOS_REG_MINUTES            0x02
#define CMOS_REG_MINUTES_ALARM      0x03
#define CMOS_REG_HOURS              0x04
#define CMOS_REG_HOURS_ALARM        0x05
#define CMOS_REG_WEEK_DAY           0x06
#define CMOS_REG_DATE_DAY           0x07
#define CMOS_REG_DATE_MONTH         0x08
#define CMOS_REG_DATE_YEAR          0x09
#define CMOS_REG_STAT_A             0x0A
#define CMOS_REG_STAT_A_UIP         0x40    // UIP = Update in progress
#define CMOS_REG_STAT_B             0x0B
#define CMOS_REG_STAT_C             0x0C
#define CMOS_REG_STAT_D             0x0D
#define CMOS_REG_DIAGNOSTIC         0x0E
#define CMOS_REG_CENTURY            0x32

uint8_t cmos_read(const uint8_t reg);
void cmos_write(const uint8_t reg, const uint8_t data);
