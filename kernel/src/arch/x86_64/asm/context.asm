# Execute the user context passed in RSI. This function is quite similar to the
# jump_to function, but it will save the kernel stack pointer into gs:24
#
# This function will return to the caller when an exception, an interrupt or a 
# syscall will occur. The register state of the user context will be saved in
# its (small) stack frame before restoring the stack frame previously stored
# into gs:24 and returning to the caller.
execute_thread:
    # Save RFLAGS and callee-saved registers
    pushfq
    push r15
    push r14
    push r13
    push r12
    push rbx
    push rbp

    # Disable interrupts
    cli

    # Save the stack pointer into the per-cpu data structure
    mov gs:24, rsp

    # Load the user context
    mov rsp, rdi
    pop rbp
    pop rbx
    pop r12
    pop r13
    pop r14
    pop r15
    pop rax
    pop rcx
    pop rdx
    pop rsi
    pop rdi
    pop r8
    pop r9
    pop r10
    pop r11
    add rsp, 32
    swapgs 
    iretq

# Resume the kernel. This function is called when an exception, an interrupt or a syscall
# occurs. It will restore the kernel stack pointer and the callee-saved registers before
# returning to the caller, allowing the kernel to resume its execution.
resume_kernel:
  mov rsp, gs:24
  pop rbp
  pop rbx
  pop r12
  pop r13
  pop r14
  pop r15
  popfq
  ret
