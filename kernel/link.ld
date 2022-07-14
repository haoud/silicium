ENTRY(_start)
OUTPUT_ARCH("i386")
OUTPUT_FORMAT("elf32-i386")

SECTIONS
{
	. = 0xC0100000;
	.text : AT(ADDR(.text) - 0xC0000000)
	{
		_text_start = .;
			*(.multiboot*)
			*(.text*)
		_text_end = .;
	}

	. = ALIGN(4096);
	.data : AT(ADDR(.data) - 0xC0000000)
	{
		_data_start = .;
			*(.data*)
		_data_end = .;
	}

	. = ALIGN(4096);
	.rodata : AT(ADDR(.rodata) - 0xC0000000)
	{
		_rodata_start = .;
			*(.rodata*)
		_rodata_end = .;
	}

	. = ALIGN(4096);
	.bss : AT(ADDR(.bss) - 0xC0000000)
	{
		_bss_start = .;
		*(.bss)
		*(COMMON)
		_bss_end = .;
	}

	. = ALIGN(4096);
	_end = .;
	
	/DISCARD/ :
	{
		*(.fini_array*)
		*(.eh_frame*)
		*(.comment)
		*(note*)
	}
}