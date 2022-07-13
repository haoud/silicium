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

#define sw_barrier()    asm volatile("" ::: "memory")
#define hw_barrier()    __sync_synchronize()

#define compiler_barrier()  sw_barrier()
#define memory_barrier()    hw_barrier()

#define read_once(x) ({                         \
    (*(volatile typeof(x) *)&(x))               \
})

#define write_once(x, v) ({                     \
    *(volatile typeof(x) *)&(x) = (v);	        \
})
