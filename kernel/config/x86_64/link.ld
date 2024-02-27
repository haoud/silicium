ENTRY(_start)
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
        . += 32;
        // gs:0 -> self pointer
        // gs:8 -> cpu identifier
        // gs:16 -> kernel stack pointer
        // gs:24 -> user stack pointer temporary save location
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
