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
#include <arch/x86/io.h>
#include <arch/x86/cpu.h>
#include <arch/x86/irq.h>
#include <arch/x86/cmos.h>

/**
 * @brief Convert a number from the BSD format (CMOS) to the binary format
 * 
 * @param value The value to convert
 * @return uint8_t The converted value to binary
 */
static inline uint8_t bsd2bin(const uint8_t value)
{
    return ((value >> 4) * 10 + (value & 0x0F));
}

/**
 * @brief Convert a number from the binary format to the BSD format (CMOS) 
 * 
 * @param value The value to convert
 * @return uint8_t The converted value to BSD
 */
static inline uint8_t bin2bsd(const uint8_t value)
{
    return (value / 10) + (value % 10);
}

/**
 * @brief Read the CMOS register. During this function, interrupts are disabled
 * are restored after the function is finished.
 * 
 * @param reg The register to read
 * @return uint8_t The value of the register
 */
uint8_t cmos_read(const uint8_t reg)
{
    irq_acquire() {
        outb(CMOS_IO_ADDRESS, reg);
        return bsd2bin(inb(CMOS_IO_DATA));
    }
    BUG();
}

/**
 * @brief Write the CMOS register. During this function, interrupts are disabled
 * are restored after the function is finished.
 * 
 * @param reg The register to write
 * @param data The data to write inside the register
 */
void cmos_write(const uint8_t reg, const uint8_t data)
{
    irq_acquire() {
        outb(CMOS_IO_ADDRESS, reg);
        const uint8_t nmi = inb(CMOS_IO_DATA) & 0x80;
        outb(CMOS_IO_DATA, (bin2bsd(data) & 0x80) | nmi);
    }
}
