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
#include <lib/memory.h>

void *_memcpy(void *restrict dst, const void *restrict src, size_t len)
{
	int d0, d1, d2;
	asm volatile("cld; rep movsb"
				 : "=&D"(d0), "=&S"(d1), "=&c"(d2)
				 : "0"(dst), "1"(src), "2"(len)
				 : "memory", "cc");
	return dst;
}

void *_memset(void *dst, uint8_t fill, size_t len)
{
	int d0, d1;
	asm volatile("cld; rep stosb"
				 : "=&D"(d0), "=&c"(d1)
				 : "a"(fill), "0"(dst), "1"(len)
				 : "memory", "cc");
	return dst;
}

void *_memmove(void *dst, const void *src, size_t len)
{
	const char *src8 = (const char *) src;
	char *dst8 = (char *) dst;

	if (dst > src) {
		asm volatile("std" ::
						 : "cc");
		dst8 += len;
		src8 += len;
	} else {
		asm volatile("cld" ::
						 : "cc");
	}

	int d0, d1, d2;
	asm volatile("rep movsb"
				 : "=&D"(d0), "=&S"(d1), "=&c"(d2)
				 : "0"(dst8), "1"(src8), "2"(len)
				 : "memory", "cc");
	return dst;
}

void *_aligned_memcpy(void *restrict dst, const void *restrict src, size_t len)
{
	int d0, d1, d2;
	asm volatile("	cld; rep movsd	\n\
					mov ecx, %6		\n\
					rep movsb"
				 : "=&D"(d0), "=&S"(d1), "=&c"(d2)
				 : "0"(dst), "1"(src), "2"(len >> 2), "g"(len & 3)
				 : "memory", "cc");
	return dst;
}

void *_aligned_memset(void *dst, uint32_t fill, size_t len)
{
	int d0, d1;
	asm volatile("	cld; rep stosd	\n\
					mov ecx, %5 	\n\
					rep stosb"
				 : "=&D"(d0), "=&c"(d1)
				 : "a"(fill), "0"(dst), "1"(len >> 2), "g"(len & 3)
				 : "memory", "cc");
	return dst;
}

int _memcmp(const void *p1, const void *p2, size_t len)
{
	const char *mem1_8 = (const char *) p1;
	const char *mem2_8 = (const char *) p2;
	while (len--) {
		if (*mem1_8++ != *mem2_8++)
			return mem1_8[-1] - mem2_8[-1];
	}
	return 0;
}

void *memscan(const void *src,
			  size_t size,
			  const void *pattern,
			  const size_t len)
{
	const char *pattern8 = (const char *) pattern;
	const char *mem8 = (const char *) src;
	if (len > size)
		return NULL;

	while ((size - len - 1) > 0) {
		if (!memcmp(mem8, pattern8, len))
			return (void *) mem8;
		mem8++;
		size--;
	}
	return NULL;
}

#undef memmove
#undef memset
#undef memcmp
#undef memcpy

void *memset(void *dst, uint8_t fill, size_t len)
__attribute__((weak, alias("_memset")));
void *memcpy(void *dst, const void *src, size_t len)
__attribute__((weak, alias("_memcpy")));
void *memmove(void *dst, const void *src, size_t len)
__attribute__((weak, alias("_memmove")));
int memcmp(const void *p1, const void *p2, size_t len)
__attribute__((weak, alias("_memcmp")));
