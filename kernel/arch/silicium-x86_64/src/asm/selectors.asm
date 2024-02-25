.global reload_selectors
.section .init
reload_selectors:
  mov rax, 0x10
  mov ds, ax    # Load the kernel data segment
  mov es, ax    # Load the kernel extra segment
  mov ss, ax    # Load the kernel extra segment
  
  # A little trick to load the kernel code segment. We must use the retqf
  # instruction that work almost like a ret instruction, but it also pops
  # the CS register from the stack.
  # So we push the desired value of CS on the stack and the address of the
  # call instruction, and then we use the retfq instruction to pop the
  # desired value of CS into the CS register and return to the caller
  push 0x08
  push [rsp + 8]
  retfq

.section .text
