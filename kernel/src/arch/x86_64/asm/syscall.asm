# Called when a syscall is made with the SYSCALL instruction.
# Interrupts was masked by the SYSCALL instruction as well as some others
# flags needed by the system V ABI. The original RFLAGS is saved in the
# R10 register.
# 
# TODO: Better description
.global syscall_enter
syscall_enter:
    # TODO: -ENOSYS fast path

    swapgs            # Switch to the kernel GS
    mov gs:16, rsp    # Save the user stack pointer in the per-cpu area
    mov rsp, gs:32    # Set the kernel stack pointer to the user registers area
    
    # Save registers
    push 0x23         # SS
    push gs:16        # RSP 
    push r11          # RFLAGS
    push 0x2B         # CS
    push rcx          # RIP

    # Push some data to caracterize the trap type
    push 0    # Error code
    push 2    # Trap type (syscall)
    push rax  # Custom data (syscall number)
    push 0    # Padding

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

    # Resume to the kernel task responsible for the current 
    # user thread
    jmp resume_kernel

