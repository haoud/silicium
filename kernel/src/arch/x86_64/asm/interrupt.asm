# Common code for interrupt handling. This code is called by every interrupt handler located in the
# `interrupt_handlers` section. It will save the registers and depending if the interrupted code was
# running in the kernel or in user space, will call `kernel_interrupt` if the kernel was interrupted or
# `resume_kernel` if the user space was interrupted by the interrupt.
.global interrupt_common
interrupt_common:
    # Save scratch registers
    push r11
    push r10
    push r9
    push r8
    push rdi
    push rsi
    push rdx
    push rcx
    push rax

    # Save preserved registers
    push r15
    push r14
    push r13
    push r12
    push rbx
    push rbp

    # Depending on the privilege level of the interrupted 
    # code, call the appropriate handler
    cmp QWORD ptr [rsp + 160], 0x08
    je kernel_interrupt

    swapgs
    jmp resume_kernel

# Called when an interrupt occurs during the execution of the kernel. It will enable interrupts (because
# it is safe to do so here) and call the interrupt handler with the stack frame as argument. After the
# completion of the interrupt handler, it will call `interrupt_exit` to restore the interrupted code.
.global kernel_interrupt
kernel_interrupt:
    # Enable interrupts and call the interrupt handler with the
    # stack frame as argument.
    cmp QWORD ptr [rsp + 136], 0
    je .exception

    cmp QWORD ptr [rsp + 136], 1
    je .interrupt

    # Should never be reached
    jmp kernel_interrupt

.exception:
    mov rdi, rsp
    call exception_handler
    jmp interrupt_exit

.interrupt:
    mov rdi, QWORD ptr [rsp + 128]
    call irq_handler
    jmp interrupt_exit 


# Called by the interrupt handler when an interrupt was handled by the kernel. It will disable
# interrupts temporarily and restore the interrupted code.
.global interrupt_exit
interrupt_exit:
    # Disable interrupts
    cli

    # Restore preserved registers
    pop rbp
    pop rbx
    pop r12
    pop r13
    pop r14
    pop r15

    # Restore scratch registers
    pop rax
    pop rcx
    pop rdx
    pop rsi
    pop rdi
    pop r8
    pop r9
    pop r10
    pop r11

    # Skip useless data and return to the interrupted code
    add rsp, 32
    iretq

# A macro to generate interrupt handlers for all interrupts.
.altmacro
.macro generate_handler irq
  .if irq == 8 || irq == 10 || irq == 11 || irq == 12 || irq == 13 || irq == 14 || irq == 17 || irq == 21 || irq == 29 || irq == 30
    .align 16
    .global irq_\irq
    irq_\irq :
      push 0
      push irq
      push 0
      jmp interrupt_common
  .elseif irq < 32
    .align 16
    .global irq_\irq
    irq_\irq :
      push 0
      push 0
      push irq
      push 0
      jmp interrupt_common
  .else
    .align 16
    .global irq_\irq
    irq_\irq :
      push 0
      push 1
      push irq
      push 0
      jmp interrupt_common
  .endif
.endm

# The interrupt handlers code
.global interrupt_handlers
.align 16
interrupt_handlers:
.set irq, 0
.rept 256
  generate_handler %irq
  .set irq, irq + 1
.endr

