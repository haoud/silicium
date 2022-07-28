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
#include <arch/x86/irq.h>
#include <arch/x86/pic.h>
#include <arch/x86/pit.h>
#include <process/schedule.h>

static uint32_t startup_tick = 0;

void pit_tick(struct cpu_state *state)
{
	schedule_tick();
	startup_tick++;
}

/**
 * @brief Configure the channel of the PIT to generate a periodic interrupt at
 * 100 Hz.
 */
_init void pit_configure(void)
{
    outb(PIT_IO_CMD, PIT_CHANNEL0 | PIT_MODE_RATE_GENERATOR);
	outb(PIT_IO_TIMER0, PIT_KERN_LATCH & 0xFF);
	outb(PIT_IO_TIMER0, (PIT_KERN_LATCH >> 8) & 0xFF);
	irq_request(PIT_IRQ, pit_tick, "PIT", 0);
}

/**
 * @brief Calculate the time elapsed in nanoseconds since the beginning of
 * the tick
 * 
 * @return The offset in nanosecond in the current tick
 */
uint32_t pit_nano_offset(void)
{
    outb(PIT_IO_CMD, PIT_CHANNEL0);
	const uint32_t count_low = inb(PIT_IO_TIMER0);
	const uint32_t count_high = inb(PIT_IO_TIMER0);
	const uint32_t count = count_low | (count_high) << 8;
    return ((PIT_KERN_LATCH - (PIT_KERN_LATCH - count)) * PIT_TICK_NS);
}
