# Switch between two threads. This function will save and suspend the current
# thread's state and restore the new thread's state.
# 
# When the thread will be resumed, it will continue normally in the caller's
# address,as if it was never suspended. 
#
# This function use the System V ABI to optimize the context switch. This ABI
# specifies that the registers RBP, RBX, R12, R13, R14, R15, and RSP must be
# preserved across function calls, and that the RDI, RSI, RDX, RCX, R8, R9, and
# RAX registers are clobbered by function calls. 
# This means that since we use an ABI-compliant function call to switch threads,
# we can only save the register that need to be restored before this function
# returns to the caller since others registers will be preserved by the compiler
# automatically when calling this function.
#
# Parameters:
#   - RDI: Pointer to the current thread's register context, where the
#          current thread's state will be saved
#   - RSI: Pointer to the new thread's register context, from where the
#          new thread's state will be restored
# 
# This function does not return any value.
thread_switch:
    # Load into RAX the address where we want to execute when the current 
    # thread will be resumed
    lea rax, thread_return

    # Save RFLAGS into RAX
    pushfq
    pop rdx

    # Save registers that must be preserved into 
    # the current thread's register context
    mov [rdi], rdx          # Save RFLAGS
    mov [rdi + 8], rbp      # Save RBP
    mov [rdi + 16], rbx     # Save RBX
    mov [rdi + 24], r12     # Save R12
    mov [rdi + 32], r13     # Save R13
    mov [rdi + 40], r14     # Save R14
    mov [rdi + 48], r15     # Save R15
    mov [rdi + 56], rsp     # Save RSP
    mov [rdi + 64], rax     # Save RIP

    # Load registers that was previously saved from 
    # the new thread's register context
    mov rdx, [rsi]          # Restore RFLAGS
    mov rbp, [rsi + 8]      # Restore RBP
    mov rbx, [rsi + 16]     # Restore RBX
    mov r12, [rsi + 24]     # Restore R12
    mov r13, [rsi + 32]     # Restore R13
    mov r14, [rsi + 40]     # Restore R14
    mov r15, [rsi + 48]     # Restore R15
    mov rsp, [rsi + 56]     # Restore RSP
    mov rax, [rsi + 64]     # Restore RIP

    # Restore RFLAGS from RAX and resume the thread
    push rdx
    popfq
    
    # Remove the return address from the stack and jump to the
    # caller's address stored in RAX
    jmp rax

# Execute the ret instruction
# 
# This function does nothing from a caller's point of view, but it is very
# useful to resume a thread that was previously suspended by the thread_switch.
# The thread will land here and return to the caller's address pushed on the
# stack by the thread_switch function, and continue normally as if it was
# never suspended.
thread_return:
    ret

# Jump to a new thread. This function restore the given thread's state and
# resume it without saving the current thread's state.
#
# Parameters:
#   - RDI: Pointer to the thread's register context to restore
#
# This function never returns to the caller.
thread_jump:
    # Load registers that was previously saved from 
    # the new thread's register context
    mov rdx, [rdi]          # Restore RFLAGS
    mov rbp, [rdi + 8]      # Restore RBP
    mov rbx, [rdi + 16]     # Restore RBX
    mov r12, [rdi + 24]     # Restore R12
    mov r13, [rdi + 32]     # Restore R13
    mov r14, [rdi + 40]     # Restore R14
    mov r15, [rdi + 48]     # Restore R15
    mov rsp, [rdi + 56]     # Restore RSP
    mov rax, [rdi + 64]     # Restore RIP

    # Restore RFLAGS from RAX and resume the thread
    push rdx
    popfq
    
    jmp rax

# Enter a new thread. This function is called when a newly created thread is
# started. 
# It will clear all registers to avoid leaking sensitive data (especially if
# this is a user thread) and jump to the entry point of the new thread with
# the iretq instruction.
#
# This function never returns to the caller, and should never be called 
# manually. 
thread_enter:
    # Clear all registers to avoid leaking sensitive data. Some registers 
    # are not cleared because they should already be zeroed by the 
    # thread_jump or thread_switch function that started the thread.
    xor r8, r8
    xor r9, r9
    xor r10, r10
    xor r11, r11
    xor rax, rax
    xor rcx, rcx
    xor rdx, rdx
    xor rsi, rsi
    xor rdi, rdi

    # Restore the user GS value if we was in user mode. If we was in kernel 
    # mode, we can simply continue to use the current GS value.
    cmp QWORD ptr [rsp + 8], 0x08
    je 1f
    swapgs

1:
    # Return to user mode with iretq, loading an interrupt stack frame
    # that was previously prepared during the thread creation
    iretq
