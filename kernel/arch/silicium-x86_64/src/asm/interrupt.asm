# This is the common interrupt handler, which is called by all interrupt
# handlers. It switchs to the kernel GS segment if we was in user mode,
# and saves all registers before calling the irq_handler function that will
# handle and dispatch the interrupt. After the irq_handler returns, it restores
# the registers, switches back to the user GS segment if we was in user mode,
# and returns to the interrupted code.
interrupt_common:
    # Swap the kernel and user GS if we was in user mode
    cmp QWORD ptr [rsp + 8 * 3], 0x08
    je 1f
    swapgs
1:
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
   
    mov rdi, rsp
    call irq_handler

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

    # Restore user GS if we was in user mode
    cmp QWORD ptr [rsp + 8 * 3], 0x08
    je 1f
    swapgs
1:
    # Skip error code and the interrupt number and return
    add rsp, 16
    iretq


.altmacro
.macro generate_handler irq
  .if irq == 8 || irq == 10 || irq == 11 || irq == 12 || irq == 13 || irq == 14 || irq == 17 || irq == 21 || irq == 29 || irq == 30
    .align 16
    .global irq_\irq
    irq_\irq :
      push irq
      jmp interrupt_common
  .else
    .align 16
    .global irq_\irq
    irq_\irq :
      push 0
      push irq
      jmp interrupt_common
  .endif
.endm

.global interrupt_handlers
.align 16
interrupt_handlers:
.set irq, 0
.rept 256
  generate_handler %irq
  .set irq, irq + 1
.endr

