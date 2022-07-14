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

	/* 
	 * Align is not need here because on x86, these two sections 
	 * have the same paging rights
	 */
	.rodata : AT(ADDR(.rodata) - 0xC0000000)
	{
		_rodata_start = .;
		*(.rodata*)
		_rodata_end = .;
	}

	. = ALIGN(4096);
	.data : AT(ADDR(.data) - 0xC0000000)
	{
		_data_start = .;
		*(.data*)
		_data_end = .;
	}

	/* 
	 * Align is not need here because on x86, these sections 
	 * have the same paging rights (.init are not keeped in
	 * memory after the boot so we can discard access rights)
	 */

	_init_start = .;
	.init.text : AT(ADDR(.init.text) - 0xC0000000)
	{
		*(.init.text*)
	}

	.init.rodata : AT(ADDR(.init.rodata) - 0xC0000000)
	{
		*(.init.rodata*)
	}

	.init.data : AT(ADDR(.init.data) - 0xC0000000)
	{
		*(.init.data*)
	}

	. = ALIGN(4096);
	.init.bss (NOLOAD) :
	{
		*(.init.bss*)
	}
	_init_end = .;

	. = ALIGN(4096);
	.bss (NOLOAD) :
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