# Restore the saved user context registers passed in RSI and jump to the saved
# user context RIP.
#
# Parameters:
#   - RSI: pointer to the saved user context registers
# 
# This function never returns to the caller.
jump_to:
  mov rsp, rsi
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
  add rsp, 16
  swapgs
  iretq
