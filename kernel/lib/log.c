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

static const unsigned int log_level = LOG_LEVEL;
static DECLARE_SPINLOCK(lock);

static inline void print(const char *s)
{
	while (*s != '\0')
		outb(0xe9, *s++);
}

_export void log(const unsigned int gravity, const char *const fmt, ...)
{
#ifdef CONFIG_LOG
	if (gravity < log_level)
		return;
	char str[LOG_MAX_LEN];
	spin_lock(&lock);

	va_list arg;
	va_start(arg, fmt);
	vsnprintf(str, LOG_MAX_LEN, fmt, arg);
	va_end(arg);

	print(level_icon_colored[gravity]);
	print(" ");
	print(str);
	print("\n");
	spin_unlock(&lock);
#endif
}
