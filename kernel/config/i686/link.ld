ENTRY(_start)
OUTPUT_FORMAT("elf32-i386")

KERNEL_VIRT_BASE = 0xC0000000;

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
