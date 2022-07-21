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
#include <config.h>
#include <stdarg.h>
#include <lib/log.h>
#include <lib/maths.h>
#include <lib/string.h>
#include <arch/x86/io.h>
#include <lib/spinlock.h>

static const char *level_icon[] = {
	"[?]",	 // Undefined
	"[T]",	 // Trace
	"[D]",	 // Debug
	"[*]",	 // Info
	"[-]",	 // Warning
	"[!]",	 // Error
	"[!!]",	 // Critical
	"[!!!]", // Fatal
};

static const char *level_icon_colored[] = {
	"[?]",
	"\033[1m[T]\033[0m",
	"\033[1m\033[34m[D]\033[0m",
	"\033[1m\033[32m[*]\033[0m",
	"\033[1m\033[33m[-]\033[0m",
	"\033[1m\033[31m[!]\033[0m",
	"\033[1m\033[31m[!!]\033[0m",
	"\033[1m\033[31m[!!!]\033[0m",
};

static const int log_level = LOG_LEVEL;
static DECLARE_SPINLOCK(lock);

static void putb(const char c)
{
	outb(0xe9, c); // Fixme: Only on bochs
}

static void print(const char *s)
{
	while (*s != '\0')
		putb(*s++);
}

static void printf(const char *const fmt, ...)
{
	char str[LOG_MAX_LEN];

	va_list arg;
	va_start(arg, fmt);
	vsnprintf(str, LOG_MAX_LEN, fmt, arg);
	va_end(arg);
	print(str);
}

void log(const int gravity, const char *const fmt, ...)
{
	char str[LOG_MAX_LEN];
	if (gravity < log_level)
		return;
	spin_lock(&lock);

	va_list arg;
	va_start(arg, fmt);
	vsnprintf(str, LOG_MAX_LEN, fmt, arg);
	va_end(arg);

#ifndef CONFIG_DISABLE_CHECKS
	const int g = clamp(gravity, LOG_LEVEL_UNDEFINED, LOG_LEVEL_FATAL);
	printf("%s %s\n", level_icon_colored[g], str);
#else
	printf("%s %s\n", level_icon_colored[gravity], str);
#endif

	spin_unlock(&lock);
}
