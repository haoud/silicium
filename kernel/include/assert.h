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

#define static_assert(cond, msg)    _Static_assert(cond, msg)
#define assume(cond)                assert(cond)

#ifdef CONFIG_DEBUG
#define assert_warning(expr, msg)                                       \
    if (!(expr)) {                                                      \
        warn("Warning (%s:%i) %s: %s", __FILE__, __LINE__, #expr, msg); \
    }

#define assert_msg(expr)                          \
    if (!(expr)) {                                \
        panic("Assertion failed (%s:%i) %s : %s", \
              __FILE__, __LINE__, #expr, msg);    \
    }

#define assert(expr)                                                        \
    if (!(expr)) {                                                          \
        panic("Assertion failed (%s:%i): %s", __FILE__, __LINE__, #expr);   \
    }

#define unimplemented()                                     \
    panic("Unimplemented (%s:%i)", __FILE__, __LINE__);

#define todo()                                              \
    panic("TODO at %s:%i:", __FILE__, __LINE__);

#else 
#define assert_warning(expr, msg)
#define assert_msg(expr, msg)
#define assert(expr)
#define unimplemented()
#define todo()
#endif