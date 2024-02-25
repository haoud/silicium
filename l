
iso/boot/silicium.elf:     format de fichier elf64-x86-64


Déassemblage de la section .text :

ffffffff80001000 <.text>:
ffffffff80001000:	50                   	push   rax
ffffffff80001001:	fa                   	cli
ffffffff80001002:	f4                   	hlt
ffffffff80001003:	eb fc                	jmp    0xffffffff80001001
