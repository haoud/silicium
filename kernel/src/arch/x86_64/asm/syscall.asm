syscall_enter:
    swapgs            # Switch to the kernel GS
    mov gs:24, rsp    # Save the user stack pointer in the per-cpu area
    mov rsp, gs:16    # Set the kernel stack pointer

    # Enable interrupts, it's safe now that we're on the kernel stack
    sti

    # Create a fake interrupt frame
    push 0x23           # SS
    push gs:24          # RSP 
    push r11            # RFLAGS
    push 0x2B           # CS
    push rcx            # RIP

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
    call kernel_enter
    call syscall_handler
    call kernel_leave

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

    # Disable interrupts since we're about to return to user mode. If interrupts are
    # enabled before we use swapgs, it could cause a race condition if an interrupt
    # occurs after the swapgs but before the iretq. The interrupt handler would then
    # assume that we was in user mode and use swapgs again, loading the user GS into
    # the active GS ! This would cause the kernel to crash and is a security issue.
    cli

    # Restore user GS and return
    swapgs
    iretq
