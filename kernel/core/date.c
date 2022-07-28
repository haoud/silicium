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
#include <arch/x86/cmos.h>

static const unsigned int month[12] = {
    0,
	31,
	31 + 28,
	31 + 28 + 31,
	31 + 28 + 31 + 30,
	31 + 28 + 31 + 30 + 31,
	31 + 28 + 31 + 30 + 31 + 30,
	31 + 28 + 31 + 30 + 31 + 30 + 31,
	31 + 28 + 31 + 30 + 31 + 30 + 31 + 31,
	31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30,
	31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31,
	31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30
};

static struct tm startup_date;

/**
 * @brief Read the current date from the CMOS. Only used at startup.
 * 
 * @param tm Where to store the date, must be allocated.
 */
_init void date_read(struct tm *tm)
{
    tm->isdst = 0;    
    tm->sec = cmos_read(CMOS_REG_SECONDS);
    tm->min = cmos_read(CMOS_REG_MINUTES);
    tm->hour = cmos_read(CMOS_REG_HOURS);
    tm->mday = cmos_read(CMOS_REG_DATE_DAY);
    tm->mon = cmos_read(CMOS_REG_DATE_MONTH);
    tm->wday = cmos_read(CMOS_REG_DATE_DAY);
    tm->yday = month[tm->mon] + tm->mday - 1;
	tm->year = cmos_read(CMOS_REG_CENTURY) * 100;
    tm->year += cmos_read(CMOS_REG_DATE_YEAR);    
}

/**
 * @brief Setup the date system and fetch the startup date
 */
_init void date_setup(void)
{
    date_read(&startup_date);
    info("startup date: %02d/%02d/%d %02d:%02d:%02d\n", 
        startup_date.mday, startup_date.mon, startup_date.year,
        startup_date.hour, startup_date.min, startup_date.sec);
}

/**
 * @brief Convert the startup date to a POSIX time
 * 
 * @return time_t The startup POSIX time
 */
time_t date_startup_unix_time(void)
{
    time_t time = startup_date.sec;
    time += startup_date.min * 60;
    time += startup_date.hour * 3600;
    time += startup_date.mday * 86400;
    time += month[startup_date.mon - 1] * 86400;
    time += (startup_date.year - 1970) * 31536000;
    time += ((startup_date.year - 1970 - 4) / 4) * 86400;

    // If the year is a leap year, add 1 day to the time
    // Not very sure about this, but it seems to work
    if ((((startup_date.year % 4) == 0) && startup_date.mon > 2) || 
        (startup_date.mon == 2 && startup_date.mday > 28))
        time += 86400;

    return time;
}
