# Reloads the default GDT selectors in the current CPU core. This function
# set the ds`, `es` and `ss` selectors to 0x10, and the `cs` selector to 0x08.
#
# This function is put in the .init section because it should only be called
# once per core at the beginning of the kernel.
.global reload_selectors
.section .init
reload_selectors:
  mov rax, 0x10
  mov ds, ax    # Load the kernel data segment
  mov es, ax    # Load the kernel extra segment
  mov ss, ax    # Load the kernel stack segment
  
  # A little trick to load the kernel code segment. We must use the retqf
  # instruction that work almost like a ret instruction, but it also pops
  # the CS register from the stack.
  # So we pop the return address from the stack, and then we push the desired 
  # value of CS on the stack and we push the return address back on the stack
  # before calling retfq. It will return to the address we pushed on the stack
  # and load the CS register with the desired value.
  pop rax
  push 0x08
  push rax
  retfq

.section .text
