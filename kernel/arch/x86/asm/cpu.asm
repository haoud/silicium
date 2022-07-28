.intel_syntax noprefix

.section .text
.align 16

# ####################################################
# Parameters
#   - esp: return address
#   - esp + 4: Pointer to location to store esp
#   - esp + 8: Pointer to the saved CPU state
# ####################################################
.global save_switch_to
.type save_switch_to, @function
save_switch_to:
    pushfd              # EFLAGS
    pushd 0x08          # CS
    pushd [esp + 8]     # EIP
    sub esp, 8          # Skip error code and custom data
    pushad
	pushd ds
	pushd es
	pushd gs
	pushd fs
	pushd ss

    # 72 bytes have been pushed on the stack.

    mov eax, [esp + 72 + 4]     # Store address of cpu_state in eax
    mov [eax], esp              # Update cpu_state of the thread
    mov esp, [esp + 72 + 8]     # Load the saved CPU state
    popd ss
    popd gs
    popd fs
    popd es
    popd ds
    popad
	add esp, 8
	iretd

.align 16
# ####################################################
# Parameters
#   - esp: return address
#   - esp + 4: Pointer to the saved CPU state
# ####################################################
.global switch_to
.type switch_to, @function
switch_to:
    mov ebp, [esp + 4]      # Load the saved CPU state
    mov esp, ebp
    popd ss
    popd gs
    popd fs
    popd es
    popd ds
    popad
	add esp, 8
	iretd
