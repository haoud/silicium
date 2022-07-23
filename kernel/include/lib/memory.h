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

#define memzero(dst, len) __memset((void *)dst, 0, len)
#define memset(dst, c, len) __memset((void *)dst, c, len)
#define memcmp(dst, src, len) _memcmp((void *)dst, (void *)src, len)
#define memcpy(dst, src, len) __memcpy((void *)dst, (void *)src, len)
#define memmove(dst, src, len) _memmove((void *)dst, (void *)src, len)

void *memscan(const void *src,
			  size_t size,
			  const void *pattern,
			  const size_t len);

int _memcmp(const void *p1, const void *p2, size_t count);
void *_memcpy(void *restrict dst, const void *restrict src, size_t len);
void *_memset(void *dst, uint8_t fill, size_t len);
void *_memmove(void *dst, const void *src, size_t len);

void *_aligned_memcpy(void *restrict dst, const void *restrict src, size_t len);
void *_aligned_memset(void *dst, uint32_t fill, size_t len);

static inline void *__memcpy(void *restrict dst, const void *restrict src, size_t len)
{
	if (!((uint32_t)dst & 3) && !((uint32_t)src & 3))
		return _aligned_memcpy(dst, src, len);

	return _memcpy(dst, src, len);
}

static inline void *__memset(void *dst, uint8_t fill, size_t len)
{
	uint32_t fill32 = 0;
	if (unlikely(fill))
		fill32 = fill | fill << 8 | fill << 16 | fill << 24;

	if (!((uint32_t)dst & 3))
		return _aligned_memset(dst, fill32, len);

	uint32_t offset = ((uint32_t)dst & 3);
	_memset(dst, fill, 4 - offset);
	return _aligned_memset((char *)dst + offset, fill32, len - offset);
}
