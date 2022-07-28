.intel_syntax noprefix

// Exceptions that does not pushes an error code
.macro DECLARE_EXCEPTION num
    .global exception_\num
    exception_\num:
        push 0
        push \num
        jmp exception_common
.endm

// Exceptions that pushes an error code
.macro DECLARE_EXCEPTION_ERR num
    .global exception_\num
    exception_\num:
        push \num
        jmp exception_common
.endm

.section .text
.align 4

// Boring...
DECLARE_EXCEPTION 0
DECLARE_EXCEPTION 1
DECLARE_EXCEPTION 2
DECLARE_EXCEPTION 3
DECLARE_EXCEPTION 4
DECLARE_EXCEPTION 5
DECLARE_EXCEPTION 6
DECLARE_EXCEPTION 7
DECLARE_EXCEPTION_ERR 8
DECLARE_EXCEPTION 9
DECLARE_EXCEPTION_ERR 10
DECLARE_EXCEPTION_ERR 11
DECLARE_EXCEPTION_ERR 12
DECLARE_EXCEPTION_ERR 13
DECLARE_EXCEPTION_ERR 14
DECLARE_EXCEPTION 15
DECLARE_EXCEPTION 16
DECLARE_EXCEPTION 17
DECLARE_EXCEPTION 18
DECLARE_EXCEPTION 19
DECLARE_EXCEPTION 20
DECLARE_EXCEPTION 21
DECLARE_EXCEPTION 22
DECLARE_EXCEPTION 23
DECLARE_EXCEPTION 24
DECLARE_EXCEPTION 25
DECLARE_EXCEPTION 26
DECLARE_EXCEPTION 27
DECLARE_EXCEPTION 28
DECLARE_EXCEPTION 29
DECLARE_EXCEPTION 30
DECLARE_EXCEPTION 31

exception_common:
	pushad
	pushd ds
	pushd es
	pushd gs
	pushd gs
	pushd ss
	mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
	push esp
	call exception_handler
	jmp ret_from_interrupt
	
