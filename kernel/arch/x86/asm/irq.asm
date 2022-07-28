.intel_syntax noprefix

.macro DECLARE_IRQ num
    .global irq_\num
    .type irq_\num, @function
    irq_\num:
        push 0
        push \num
        jmp irq_common
.endm

.section .text
.align 4

DECLARE_IRQ 0
DECLARE_IRQ 1
DECLARE_IRQ 2
DECLARE_IRQ 3
DECLARE_IRQ 4
DECLARE_IRQ 5
DECLARE_IRQ 6
DECLARE_IRQ 7
DECLARE_IRQ 8
DECLARE_IRQ 9
DECLARE_IRQ 10
DECLARE_IRQ 11
DECLARE_IRQ 12
DECLARE_IRQ 13
DECLARE_IRQ 14
DECLARE_IRQ 15

irq_common:
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
	call irq_handler
	jmp ret_from_interrupt
	
