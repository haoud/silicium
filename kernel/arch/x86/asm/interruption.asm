.intel_syntax noprefix

.section .text

.align 4
.global default_int
default_int:
    push 0
    push 0
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
	# call exception_handler
	add esp, 4

.global ret_from_interrupt
.type ret_from_interrupt, @function
ret_from_interrupt:
    popd ss
	popd gs
	popd fs
	popd es
	popd ds
	popad
	add esp, 8
	iretd
