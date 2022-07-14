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
#include <assert.h>
#include <arch/x86/cpu.h>
#include <arch/x86/idt.h>
#include <arch/x86/exception.h>

_init void exception_install(void)
{
    install_exception(0);
    install_exception(1);
    install_exception(2);
    install_exception(3);
    install_exception(4);
    install_exception(5);
    install_exception(6);
    install_exception(7);
    install_exception(8);
    install_exception(9);
    install_exception(10);
    install_exception(11);
    install_exception(12);
    install_exception(13);
    install_exception(14);
    install_exception(15);
    install_exception(16);
    install_exception(17);
    install_exception(18);
    install_exception(19);
    install_exception(20);
    install_exception(21);
    install_exception(22);
    install_exception(23);
    install_exception(24);
    install_exception(25);
    install_exception(26);
    install_exception(27);
    install_exception(28);
    install_exception(29);
    install_exception(30);
    install_exception(31);
}

void divide_error_exception(struct cpu_state *cpu)
{
    panic("Divide error exception");
}

void debug_exception(struct cpu_state *cpu)
{
    panic("Debug exception");
}

void nmi_exception(struct cpu_state *cpu)
{
    panic("NMI exception");
}

void breakpoint_exception(struct cpu_state *cpu)
{
    panic("Breakpoint exception");
}

void overflow_exception(struct cpu_state *cpu)
{
    panic("Overflow exception");
}

void bound_exception(struct cpu_state *cpu)
{
    panic("Bound exception");
}

void invalid_opcode_exception(struct cpu_state *cpu)
{
    panic("Invalid opcode exception");
}

void device_not_available_exception(struct cpu_state *cpu)
{
    panic("Device not available exception");
}

void double_fault_exception(struct cpu_state *cpu)
{
    panic("Double fault exception");
}

void coprocessor_segment_overrun_exception(struct cpu_state *cpu)
{
    panic("Coprocessor segment overrun exception");
}

void invalid_tss_exception(struct cpu_state *cpu)
{
    panic("Invalid TSS exception");
}

void segment_not_present_exception(struct cpu_state *cpu)
{
    panic("Segment not present exception");
}

void stack_segment_fault_exception(struct cpu_state *cpu)
{
    panic("Stack segment fault exception");
}

void general_protection_exception(struct cpu_state *cpu)
{
    panic("General protection exception");
}

void page_fault_exception(struct cpu_state *cpu)
{
    panic("Page fault exception");
}

void reserved_exception(struct cpu_state *cpu)
{
    panic("Reserved exception");
}

void floating_point_exception(struct cpu_state *cpu)
{
    panic("Floating point exception");
}

void alignment_check_exception(struct cpu_state *cpu)
{
    panic("Alignment check exception");
}

void machine_check_exception(struct cpu_state *cpu)
{
    panic("Machine check exception");
}

void simd_exception(struct cpu_state *cpu)
{
    panic("SIMD exception");
}

void exception_handler(struct cpu_state *cpu)
{
    assert(cpu->data < EXCEPTION_COUNT);
    switch (cpu->data) {
        case EXCEPTION_DIVIDE_ERROR:
            divide_error_exception(cpu);
            break;
        case EXCEPTION_DEBUG:
            debug_exception(cpu);
            break;
        case EXCEPTION_NMI:
            nmi_exception(cpu);
            break;
        case EXCEPTION_BREAKPOINT:
            breakpoint_exception(cpu);
            break;
        case EXCEPTION_OVERFLOW:
            overflow_exception(cpu);
            break;
        case EXCEPTION_BOUND:
            bound_exception(cpu);
            break;
        case EXCEPTION_INVALID_OPCODE:
            invalid_opcode_exception(cpu);
            break;
        case EXCEPTION_DEVICE_NOT_AVAILABLE:
            device_not_available_exception(cpu);
            break;
        case EXCEPTION_DOUBLE_FAULT:
            double_fault_exception(cpu);
            break;
        case EXCEPTION_COPROCESSOR_SEGMENT_OVERRUN:
            coprocessor_segment_overrun_exception(cpu);
            break;
        case EXCEPTION_INVALID_TSS:
            invalid_tss_exception(cpu);
            break;
        case EXCEPTION_SEGMENT_NOT_PRESENT:
            segment_not_present_exception(cpu);
            break;
        case EXCEPTION_STACK_SEGMENT_FAULT:
            stack_segment_fault_exception(cpu);
            break;
        case EXCEPTION_GENERAL_PROTECTION:
            general_protection_exception(cpu);
            break;
        case EXCEPTION_PAGE_FAULT:
            page_fault_exception(cpu);
            break;
        case EXCEPTION_RESERVED:
            reserved_exception(cpu);
            break;
        case EXCEPTION_FPU_ERROR:
            floating_point_exception(cpu);
            break;
        case EXCEPTION_ALIGNMENT_CHECK:
            alignment_check_exception(cpu);
            break;
        case EXCEPTION_MACHINE_CHECK:
            machine_check_exception(cpu);
            break;
        case EXCEPTION_SIMD_ERROR:
            simd_exception(cpu);
            break;
        default:
            panic("Unknown exception %u", cpu->data);
            break;
    }
}