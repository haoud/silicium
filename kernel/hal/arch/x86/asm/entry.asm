.intel_syntax noprefix
.set MULTIBOOT_ALIGN_FLAG,      0x01
.set MULTIBOOT_MEMINFO_FLAG,    0x02
.set MULTIBOOT_VIDEO_INFO,      0x04

.set MULTIBOOT_MAGIC,           0x1BADB002

.set MULTIBOOT_FLAGS,           MULTIBOOT_ALIGN_FLAG | MULTIBOOT_MEMINFO_FLAG | MULTIBOOT_VIDEO_INFO
.set MULTIBOOT_CHECKSUM,        -(MULTIBOOT_MAGIC + MULTIBOOT_FLAGS)

.section .multiboot
.align 8

.long MULTIBOOT_MAGIC
.long MULTIBOOT_FLAGS
.long MULTIBOOT_CHECKSUM
.long 0
.long 0
.long 0
.long 0
.long 0
.long 0
.long 0
.long 0
.long 0

.section .init.text
.global _start
.extern start

.align 4
identity_map:
    mov [edx], eax
    add edx, 4                          # Next directory entry
    add eax, 0x400000                   # Next large page
    loop identity_map                   # Loop until we mapped all entries
    ret

.align 4
_start:
    lea esp, stack_top - 0xC0000000     # Setup stack

    # Define constants for this function
    lea edi, identity_map - 0xC0000000  # Physical address of this function
    lea esi, boot_pd - 0xC0000000       # Physical address of the page directory

    # Setup identity mapping for the first 3GB of memory.
    mov eax, 0x83                       # Present/Write/Large page
    mov ecx, 768                        # Number of entries to map
    mov edx, esi                        # Physical address of boot_pd
    call edi

    # Map the first Gb of physical memory at 0xC0000000
    mov eax, 0x83                        # Present/Write/Large page
    mov ecx, 255                         # Number of entries to map
    call edi

    # Set up the mirroring
    mov eax, esi                        # Physical address of boot_pd
    or al, 0x03                         # Read/Write
    mov [edx], eax

    # Load page directory
    mov cr3, esi

    # Enable 4MB pages
    mov eax, cr4
	or al, 0x10
	mov cr4, eax
	
    # Enable paging and write protect for kernel
	mov eax, cr0
	or eax, 0x80010000
	mov cr0, eax

    lea esp, stack_top                  # Setup the stack with the virtual address
    lea eax, start                      # Absolute address of the function
    xor ebp, ebp                        # First stack frame
    push ebx                            # Multiboot structure (physical address !)
    call eax                            # Let's go !

.section .init.bss, "aw", @nobits
.align 4096
boot_pd: .skip 4096
stack_bottom: .skip 4096
stack_top:
