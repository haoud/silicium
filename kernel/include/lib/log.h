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

#ifndef LOG_LEVEL
#define LOG_LEVEL LOG_LEVEL_TRACE
#endif

#define LOG_LEVEL_UNDEFINED 0
#define LOG_LEVEL_TRACE 1
#define LOG_LEVEL_DEBUG 2
#define LOG_LEVEL_INFO 3
#define LOG_LEVEL_WARN 4
#define LOG_LEVEL_ERROR 5
#define LOG_LEVEL_CRIT 6
#define LOG_LEVEL_FATAL 7

#define LOG_MAX_LEN 256

#define info(fmt...) log(LOG_LEVEL_INFO, fmt)
#define warn(fmt...) log(LOG_LEVEL_WARN, fmt)
#define trace(fmt...) log(LOG_LEVEL_TRACE, fmt)
#define debug(fmt...) log(LOG_LEVEL_DEBUG, fmt)
#define error(fmt...) log(LOG_LEVEL_ERROR, fmt)
#define fatal(fmt...) log(LOG_LEVEL_FATAL, fmt)
#define critical(fmt...) log(LOG_LEVEL_CRIT, fmt)

void log(const unsigned int gravity, const char *const fmt, ...);
