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

#define cli() asm volatile("cli")
#define sti() asm volatile("sti")
#define hlt() asm volatile("hlt")
#define clts() asm volatile("clts")

#define cpu_stop() ({             \
	asm volatile("1:hlt;jmp 1b"); \
	_unreachable();               \
})

#define cpu_relax() asm volatile("pause" ::: "memory")

#define clear_task_switched() clts()
#define enable_interruption() sti()
#define disable_interruption() cli()

#define CPUID_GET_FEATURE 1
#define CPUID_GET_CAPABILITIES 0x80000007

#define CPUID_EDX_FEATURE_FPU 0x00000001
#define CPUID_EDX_FEATURE_VME 0x00000002
#define CPUID_EDX_FEATURE_DE 0x00000004
#define CPUID_EDX_FEATURE_PSE 0x00000008
#define CPUID_EDX_FEATURE_TSC 0x00000010
#define CPUID_EDX_FEATURE_MSR 0x00000020
#define CPUID_EDX_FEATURE_PAE 0x00000040
#define CPUID_EDX_FEATURE_MCE 0x00000080
#define CPUID_EDX_FEATURE_CX8 0x00000100
#define CPUID_EDX_FEATURE_APIC 0x00000200
#define CPUID_EDX_FEATURE_SEP 0x00000800
#define CPUID_EDX_FEATURE_MTRR 0x00001000
#define CPUID_EDX_FEATURE_PGE 0x00002000
#define CPUID_EDX_FEATURE_MCA 0x00008000
#define CPUID_EDX_FEATURE_CMOV 0x00010000
#define CPUID_EDX_FEATURE_PAT 0x00020000
#define CPUID_EDX_FEATURE_PSE36 0x00040000
#define CPUID_EDX_FEATURE_PSN 0x00080000
#define CPUID_EDX_FEATURE_CLF 0x00100000
#define CPUID_EDX_FEATURE_DTES 0x00200000
#define CPUID_EDX_FEATURE_ACPI 0x00400000
#define CPUID_EDX_FEATURE_MMX 0x00800000
#define CPUID_EDX_FEATURE_FXSR 0x01000000
#define CPUID_EDX_FEATURE_SSE 0x02000000
#define CPUID_EDX_FEATURE_SSE2 0x04000000
#define CPUID_EDX_FEATURE_SS 0x08000000
#define CPUID_EDX_FEATURE_HTT 0x10000000
#define CPUID_EDX_FEATURE_TM1 0x20000000
#define CPUID_EDX_FEATURE_IA64 0x40000000
#define CPUID_EDX_FEATURE_PBE 0x80000000

#define CPUID_EDX_CAPABILITIES_ITSC 0x80000100

#define EFLAGS_CF 0x00000001
#define EFLAGS_PF 0x00000004
#define EFLAGS_AF 0x00000010
#define EFLAGS_ZF 0x00000040
#define EFLAGS_SF 0x00000080
#define EFLAGS_TF 0x00000100
#define EFLAGS_IF 0x00000200
#define EFLAGS_DF 0x00000400
#define EFLAGS_OF 0x00000800
#define EFLAGS_IOPL 0x00003000
#define EFLAGS_IOPL_KERNEL 0x00000000
#define EFLAGS_IOPL_USER 0x00003000
#define EFLAGS_NT 0x00004000
#define EFLAGS_RF 0x00010000
#define EFLAGS_VM 0x00020000
#define EFLAGS_AC 0x00040000
#define EFLAGS_VIF 0x00080000
#define EFLAGS_VIP 0x00100000
#define EFLAGS_ID 0x00200000

#define CR0_PROTECTED_MODE 0x00000001
#define CR0_COPROCESSOR_MON 0x00000002
#define CR0_COPROCESSOR_EMU 0x00000004
#define CR0_TASK_SWITCHED 0x00000008
#define CR0_EXTENSION_TYPE 0x00000010
#define CR0_NUMERIC_ERROR 0x00000020
#define CR0_WRITE_PROTECT 0x00010000
#define CR0_ALIGN_MASK 0x00040000
#define CR0_NOT_WRITE_THROUGH 0x20000000
#define CR0_CACHE_DISABLE 0x40000000
#define CR0_PAGING 0x80000000

#define CR4_VME 0x00000001
#define CR4_PVI 0x00000002
#define CR4_TSD 0x00000004
#define CR4_DE 0x00000008
#define CR4_PSE 0x00000010
#define CR4_PAE 0x00000020
#define CR4_MCE 0x00000040
#define CR4_PGE 0x00000080
#define CR4_PCE 0x00000100
#define CR4_OSFXRS 0x00000200
#define CR4_OSMXMME 0x00000400
#define CR4_UMIP 0x00000800
#define CR4_WMXE 0x00002000
#define CR4_SMXE 0x00004000
#define CR4_PCIDE 0x00020000
#define CR4_OSXSAVE 0x00040000
#define CR4_SMEP 0x00100000
#define CR4_SMAP 0x00200000

typedef struct cpu_state {
	uint32_t ss;
	uint32_t gs;
	uint32_t fs;
	uint32_t es;
	uint32_t ds;
	uint32_t edi;
	uint32_t esi;
	uint32_t ebp;
	uint32_t pushad_esp;
	uint32_t ebx;
	uint32_t edx;
	uint32_t ecx;
	uint32_t eax;
	uint32_t data;
	uint32_t error_code;
	uint32_t eip;
	uint32_t cs;
	uint32_t eflags;
	uint32_t esp3;
	uint16_t ss3;
} _packed cpu_state_t;

static inline void set_task_switched(void)
{
	asm volatile("	mov eax, cr0 		\n\
					or eax, 0x08 		\n\
					mov cr0, eax" ::
					 : "eax");
}

static inline void cpuid_count(const uint32_t code, const uint32_t count,
							   uint32_t *const eax, uint32_t *const ebx,
							   uint32_t *const ecx, uint32_t *const edx)
{
	asm volatile("cpuid"
				 : "=a"(*eax), "=b"(*ebx), "=c"(*ecx), "=d"(*edx)
				 : "0"(code), "2"(count)
				 : "memory");
}

static inline void cpuid(const uint32_t code,
						 uint32_t *const eax, uint32_t *const ebx,
						 uint32_t *const ecx, uint32_t *const edx)
{
	cpuid_count(code, 0, eax, ebx, ecx, edx);
}

static inline uint32_t cpuid_eax(const uint32_t code)
{
	uint32_t eax, ebx, ecx, edx;
	cpuid_count(code, 0, &eax, &ebx, &ecx, &edx);
	return eax;
}

static inline uint32_t cpuid_ebx(const uint32_t code)
{
	uint32_t eax, ebx, ecx, edx;
	cpuid_count(code, 0, &eax, &ebx, &ecx, &edx);
	return ebx;
}

static inline uint32_t cpuid_ecx(const uint32_t code)
{
	uint32_t eax, ebx, ecx, edx;
	cpuid_count(code, 0, &eax, &ebx, &ecx, &edx);
	return ecx;
}

static inline uint32_t cpuid_edx(const uint32_t code)
{
	uint32_t eax, ebx, ecx, edx;
	cpuid_count(code, 0, &eax, &ebx, &ecx, &edx);
	return edx;
}

static inline uint32_t get_eflags(void)
{
	uint32_t e;
	asm volatile("pushf; pop %0"
				 : "=r"(e));
	return e;
}

static inline uint64_t rdtsc(void)
{
	uint64_t ret;
	asm volatile("rdtsc"
				 : "=A"(ret));
	return ret;
}