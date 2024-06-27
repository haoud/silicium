ENTRY(_entry)
OUTPUT_FORMAT("elf64-x86-64")

KERNEL_VIRT_BASE = 0xFFFFFFFF80000000;

SECTIONS
{
    . = KERNEL_VIRT_BASE;

    __init_start = .;
    .init :
    {
        *(.init*)

        . = ALIGN(16);
        __percpu_start = .;
        . += 64;
        // gs:0 -> Self pointer
        // gs:8 -> CPU LAPIC Identifier
        // gs:16 -> User stack pointer (syscall only)
        // gs:24 -> Per-cpu kernel stack pointer
        // gs:32 -> Per-thread kernel stack pointer
        *(.percpu*)
        __percpu_end = .;
    }

    __init_end = .;

    . = ALIGN(4096);
    .text :
    {        
        *(.text*)
    }

    . = ALIGN(4096);
    .rodata :
    {
        *(.rodata*)
    }

    . = ALIGN(4096);
    .data :
    {
        *(.data*)
    }

    . = ALIGN(4096);
    .bss (NOLOAD) :
    {
        *(.bss*)
    }

    . = ALIGN(4096);
    __end = .;
    
    /DISCARD/ :
    {
        *(.fini_array*)
        *(.eh_frame*)
        *(.comment)
        *(note*)
    }
}
