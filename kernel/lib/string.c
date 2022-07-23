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
#include <lib/maths.h>
#include <lib/string.h>
#include <lib/memory.h>
#include <mm/malloc.h>

#ifdef CONFIG_VSNPRINTF_64BITS
#define NUMBER_INT_MAX INT64_MAX
#define BUFFER_LEN 128
typedef uint64_t number_t;
#else 
#define NUMBER_INT_MAX INT32_MAX
#define BUFFER_LEN 64
typedef uint32_t number_t;

#endif

char *strdup(const char *str)
{
	const size_t len = strlen(str);
	char *copy = malloc(len + 1);
	if (copy == NULL)
		return NULL;
	memcpy(copy, str, len + 1);
	return copy;
}

size_t strlen(const char *str)
{
	size_t len = 0;
	while (str[len] != '\0')
		len++;
	return len;
}

char *strchr(const char *str, char c)
{
	do {
		if (*str == c)
			return (char *) str;
	} while (*str++);
	return NULL;
}

char *strncpy(char *dst, const char *src, size_t len)
{
	size_t size = strlen(src);
	if (size < len)
		memzero(dst + size, len - size);
	return memcpy(dst, src, min(len, size));
}

int strcmp(const char *s1, const char *s2)
{
	uint32_t i = 0;
	while (1) {
		if (s1[i] < s2[i])
			return -1;
		else if (s1[i] > s2[i])
			return 1;
		else if (s1[i++] == '\0') 
			return 0;
	}
}

int strncmp(const char *s1, const char *s2, const size_t len)
{
	size_t offset = 0;
	char c1, c2;

	do {
		c1 = *s1++;
		c2 = *s2++;
		if (c1 == '\0')
			break;
	} while ((c1 == c2) && (++offset < len));
	return c1 - c2;
}

int snprintf(char *str, size_t n, const char *fmt, ...)
{
	va_list arg;
	va_start(arg, fmt);
	int len = vsnprintf(str, n, fmt, arg);
	va_end(arg);
	return len;
}

#define vsnprintf_putbuf(buf, n, c) ({	\
	if (n) {							\
		(*(buf++)) = (c);				\
		(n)--;							\
	}									\
})

#define VSNPRINTF_HASHTAG 0x01
#define VSNPRINTF_ZERO 0x02
#define VSNPRINTF_MINUS 0x04
#define VSNPRINTF_SPACE 0x08
#define VSNPRINTF_PLUS 0x10
#define VSNPRINTF_SIGN 0x20
#define VSNPRINTF_LARGE 0x40

char *number(
	char *buf,
 	size_t *const n,
	number_t number,
	int pad,
	int base,
	const int flags)
{
	const char *digits = "0123456789abcdef";
	char buffer[BUFFER_LEN];
	char padding = ' ';
	char sign = 0;
	int i = 0;

#ifndef CONFIG_DISABLE_CHECKS
	base = clamp(base, 2, 16);
	pad = clamp(pad, 0, 32);
#endif

	if (flags & VSNPRINTF_SIGN && number > NUMBER_INT_MAX) {
		number = -number;
		sign = '-';
	}
	if (flags & VSNPRINTF_SPACE) {
		sign = ' ';
	}
	if (flags & VSNPRINTF_PLUS) {
		sign = '+';
	}
	if (flags & VSNPRINTF_ZERO) {
		padding = '0';
	}
	if (flags & VSNPRINTF_LARGE) {
		digits = "0123456789ABCDEF";
	}

	do {
		buffer[i++] = digits[number % base];
		number /= base;
	} while (number != 0);

	if (sign && padding == '0') {
		vsnprintf_putbuf(buf, *n, sign);
		pad--;
	}

	int len = pad - i;
	while (len-- > 0) 
		vsnprintf_putbuf(buf, *n, padding);
	if (sign && padding == ' ') 
		vsnprintf_putbuf(buf, *n, sign);
	while (i) 
		vsnprintf_putbuf(buf, *n, buffer[--i]);

	return buf;
}

int vsnprintf(char *str, size_t len, const char *fmt, va_list arg)
{
	number_t value = 0;
	size_t n = len - 1;
	char *buf = NULL;
	int flags = 0;
	int pad = 0;
	int end = 0;

	for (buf = str; *fmt != '\0' && n; fmt++) {
		if (*fmt != '%') {
			vsnprintf_putbuf(buf, n, *fmt);
			continue;
		}

		flags = end = pad = 0;
		while (!end) {
			switch (*++fmt) {
				default:
					end = 1;
					break;
				case '+':
					flags |= VSNPRINTF_PLUS;
					break;
				case '0':
					flags |= VSNPRINTF_ZERO;
					break;
				case '-':
					flags |= VSNPRINTF_MINUS;
					break;
				case ' ':
					flags |= VSNPRINTF_SPACE;
					break;
				case '#':
					flags |= VSNPRINTF_HASHTAG;
					break;
			}
		}

		/* On récupère le chiffre de padding s'il existe */
		while (isdigit(*fmt))
			pad = pad * 10 + (*fmt++ - '0');

		switch (*fmt) {
#ifdef CONFIG_VSNPRINTF_64BITS
		// Long...
			case 'l':
				switch (*++fmt) {
				// Long...
					case 'l':
						switch (*++fmt) {
							case 'b':
								value = va_arg(arg, uint64_t);
								buf = number(buf, &n, value, pad, 2, flags);
								break;
							case 'd':
								flags |= VSNPRINTF_SIGN;
								value = va_arg(arg, uint64_t);
								buf = number(buf, &n, value, pad, 10, flags);
								break;
							case 'o':
								value = va_arg(arg, uint64_t);
								buf = number(buf, &n, value, pad, 8, flags);
								break;
							case 'u':
								value = va_arg(arg, uint64_t);
								buf = number(buf, &n, value, pad, 10, flags);
								break;
							case 'x':
								value = va_arg(arg, uint64_t);
								buf = number(buf, &n, value, pad, 16, flags);
								break;
							default:
								break;
						}
						break;
					default:
						break;
					}
				break;
#endif
			case 'b':
				value = va_arg(arg, uint32_t);
				buf = number(buf, &n, value & 0xFFFFFFFF, pad, 2, flags);
				break;
			case 'c':
				value = va_arg(arg, uint32_t);
				vsnprintf_putbuf(buf, n, (char)value & 0xFFFFFFFF);
				break;
			case 'd':
				flags |= VSNPRINTF_SIGN;
				value = va_arg(arg, int32_t);
				buf = number(buf, &n, value & 0xFFFFFFFF, pad, 10, flags);
				break;
			case 'i':
				flags |= VSNPRINTF_SIGN;
				value = va_arg(arg, int32_t);
				buf = number(buf, &n, value & 0xFFFFFFFF, pad, 10, flags);
				break;
			case 'o':
				value = va_arg(arg, int32_t);
				buf = number(buf, &n, value & 0xFFFFFFFF, pad, 8, flags);
				break;
			case 's':
				for (const char *s = va_arg(arg, char *); *s != '\0' && n;)
					vsnprintf_putbuf(buf, n, *s++);
				break;
			case 'x':
				value = va_arg(arg, int32_t);
				buf = number(buf, &n, value & 0xFFFFFFFF, pad, 16, flags);
				break;
			case 'X':
				flags |= VSNPRINTF_LARGE;
				value = va_arg(arg, int32_t);
				buf = number(buf, &n, value & 0xFFFFFFFF, pad, 16, flags);
				break;
			case 'u':
				value = va_arg(arg, int32_t);
				buf = number(buf, &n, value & 0xFFFFFFFF, pad, 10, flags);
				break;
			case 'p':
				flags |= VSNPRINTF_ZERO;
				value = va_arg(arg, int32_t);
				buf = number(buf, &n, value & 0xFFFFFFFF, 8, 16, flags);
				break;
			case '%':
				vsnprintf_putbuf(buf, n, '%');
				break;
			default:
				break;
		}
	}

	*buf++ = '\0';
	return (int) (len - n - 1);
}
