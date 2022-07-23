/**
 * Copyright (C) 2022 Romain CADILHAC
 *
 * This file is part of SiliciumOS.
 *
 * SiliciumOS is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * SiliciumOS is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with HaoudOS.  If not, see <http://www.gnu.org/licenses/>.
 */
#pragma once
#define isdigit(c) (((c) >= '0' && (c) <= '9') ? 1 : 0)

#define abs(x) ({        \
	typeof(x) _x = (x);  \
	(_x < 0) ? -_x : _x; \
})

#define min(a, b) ({   \
	typeof(a) x = (a); \
	typeof(b) y = (b); \
	(x < y) ? x : y;   \
})

#define max(a, b) ({   \
	typeof(a) x = (a); \
	typeof(b) y = (b); \
	(x > y) ? x : y;   \
})

#define align(a, b) ({                    \
	typeof(a) x = (a);                    \
	typeof(b) al = (b);                   \
	(x % al) ? (x) + (al - (x % al)) : x; \
})

#define clamp(v, min, max) ({                       \
	typeof(v) _v = (v);                             \
	typeof(min) _min = (min);                       \
	typeof(max) _max = (max);                       \
	(_v < _min) ? _min : ((_v > _max) ? _max : _v); \
})

#define is_power_of_two(v) ({            \
	typeof(v) _v = (v);                  \
	(_v != 0) && ((_v & (_v - 1)) == 0); \
})
