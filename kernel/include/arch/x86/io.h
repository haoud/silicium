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

 // TODO: More accurate way to wait for I/O to finish ?
#define iowait(void) outb(0x80, 0)
#define outb(port, data) asm volatile("out dx, al" ::"d"(port), "a"(data));
#define outw(port, data) asm volatile("out dx, ax" ::"d"(port), "a"(data));
#define outd(port, data) asm volatile("out dx, eax" ::"d"(port), "a"(data));

#define inb(port) ({           \
	uint8_t data = 0;          \
	asm volatile("in al, dx"   \
				 : "=a"(data)  \
				 : "d"(port)); \
	data;                      \
})
#define inw(port) ({           \
	uint16_t data = 0;         \
	asm volatile("in ax, dx"   \
				 : "=a"(data)  \
				 : "d"(port)); \
	data;                      \
})
#define ind(port) ({           \
	uint32_t data = 0;         \
	asm volatile("in eax, dx"  \
				 : "=a"(data)  \
				 : "d"(port)); \
	data;                      \
})
#define inpb(port) ({          \
	uint8_t data = 0;          \
	asm volatile("in al, dx"   \
				 : "=a"(data)  \
				 : "d"(port)); \
	iowait();                  \
	data;                      \
})
#define inpw(port) ({          \
	uint16_t data = 0;         \
	asm volatile("in ax, dx"   \
				 : "=a"(data)  \
				 : "d"(port)); \
	iowait();                  \
	data;                      \
})
#define inpd(port) ({          \
	uint32_t data = 0;         \
	asm volatile("in eax, dx"  \
				 : "=a"(data)  \
				 : "d"(port)); \
	iowait();                  \
	data;                      \
})
