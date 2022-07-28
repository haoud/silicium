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
#include <core/date.h>
#include <arch/x86/pit.h>
#include <arch/x86/time.h>

static_assert(PIT_KERN_FREQ <= 1000, "PIT_KERN_FREQ must be <= 1000");
static_assert(sizeof(time_t) == 4, "time_t must be 4 bytes");

/**
 * @brief Get the current time in seconds since the epoch (1970-01-01 
 * 00:00:00 UTC).
 * 
 * @return time_t The current time in seconds since the epoch.
 */
time_t time_unix(void)
{
    return date_startup_unix_time() + time_startup();
}

/**
 * @brief Get the time in second since the kernel was started.
 * 
 * @return time_t The time in seconds since the kernel was started.
 */
time_t time_startup(void)
{
    return pit_startup_tick() / PIT_KERN_FREQ;
}

/**
 * @brief Get the time in millisecond since the kernel was started.
 * FIXME: This function will overflow 49 days after the start of the kernel.
 * For now it is absolutely not a problem, but it should be fixed.
 * 
 * @return time_t The unix time when the kernel was started, in ms
 */
time_t time_startup_ms(void)
{
    time_t time = pit_startup_tick() % PIT_KERN_FREQ;
    time *= 1000 / PIT_KERN_FREQ;
    time += time_startup() * 1000;
    return time;
}

/**
 * @brief Return the current time in a timespec_t structure. The nanoseconds
 * field is calculated from the PIT ticks and the PIT internal frequency and
 * counter. This is not the fastest and accurate way to get the current time
 * (because PIT I/O is slow), but it is fine for now.
 * 
 * @param ts The timespec structure to fill: must be a valid pointer.
 */
void time_current(timespec_t *ts)
{
    assert(ts != NULL);
    ts->tv_nsec = (pit_startup_tick() % PIT_KERN_FREQ);
    ts->tv_nsec *= (1000000 / PIT_KERN_FREQ);
    ts->tv_nsec += pit_nano_offset();
    ts->tv_sec = time_unix();
}
