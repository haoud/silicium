.intel_syntax noprefix

.section .text
.align 16

# ####################################################
# Parameters
#   - esp: return address
#   - esp + 4: Pointer to the saved CPU state
# ####################################################
.global switch_to
.type switch_to, @function
switch_to:
    mov ebp, [esp + 4]
    mov esp, ebp
    popd ss
    popd gs
    popd fs
    popd es
    popd ds
    popad
	add esp, 8
	iretd
