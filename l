
iso/boot/silicium.elf:     format de fichier elf64-x86-64


Déassemblage de la section .init :

ffffffff80000000 <__init_start>:
ffffffff80000000:	48 c7 c0 10 00 00 00 	mov    $0x10,%rax
ffffffff80000007:	66 8e d8             	mov    %ax,%ds
ffffffff8000000a:	66 8e c0             	mov    %ax,%es
ffffffff8000000d:	66 8e d0             	mov    %ax,%ss
ffffffff80000010:	6a 08                	push   $0x8
ffffffff80000012:	ff 74 24 08          	push   0x8(%rsp)
ffffffff80000016:	48 cb                	lretq

ffffffff80000018 <_start>:
ffffffff80000018:	55                   	push   %rbp
ffffffff80000019:	41 57                	push   %r15
ffffffff8000001b:	41 56                	push   %r14
ffffffff8000001d:	41 55                	push   %r13
ffffffff8000001f:	41 54                	push   %r12
ffffffff80000021:	53                   	push   %rbx
ffffffff80000022:	48 83 ec 18          	sub    $0x18,%rsp
ffffffff80000026:	b1 01                	mov    $0x1,%cl
ffffffff80000028:	31 c0                	xor    %eax,%eax
ffffffff8000002a:	f0 0f b0 0d ce 3f 00 	lock cmpxchg %cl,0x3fce(%rip)        # ffffffff80004000 <silicium_x86_64::boot::MMAP_REQUEST>
ffffffff80000031:	00 
ffffffff80000032:	74 0e                	je     ffffffff80000042 <_start+0x2a>
ffffffff80000034:	8a 05 c6 3f 00 00    	mov    0x3fc6(%rip),%al        # ffffffff80004000 <silicium_x86_64::boot::MMAP_REQUEST>
ffffffff8000003a:	84 c0                	test   %al,%al
ffffffff8000003c:	74 ea                	je     ffffffff80000028 <_start+0x10>
ffffffff8000003e:	f3 90                	pause
ffffffff80000040:	eb f2                	jmp    ffffffff80000034 <_start+0x1c>
ffffffff80000042:	48 83 3d e6 3f 00 00 	cmpq   $0x0,0x3fe6(%rip)        # ffffffff80004030 <silicium_x86_64::boot::MMAP_REQUEST+0x30>
ffffffff80000049:	00 
ffffffff8000004a:	c6 05 af 3f 00 00 00 	movb   $0x0,0x3faf(%rip)        # ffffffff80004000 <silicium_x86_64::boot::MMAP_REQUEST>
ffffffff80000051:	0f 84 22 05 00 00    	je     ffffffff80000579 <_start+0x561>
ffffffff80000057:	49 be 00 00 00 00 00 	movabs $0xffff800000000000,%r14
ffffffff8000005e:	80 ff ff 
ffffffff80000061:	45 31 ff             	xor    %r15d,%r15d
ffffffff80000064:	31 ff                	xor    %edi,%edi
ffffffff80000066:	e8 f2 06 00 00       	call   ffffffff8000075d <silicium_x86_64::percpu::setup>
ffffffff8000006b:	0f 20 d8             	mov    %cr3,%rax
ffffffff8000006e:	4a 8d 14 30          	lea    (%rax,%r14,1),%rdx
ffffffff80000072:	48 81 c2 00 08 00 00 	add    $0x800,%rdx
ffffffff80000079:	be 00 01 00 00       	mov    $0x100,%esi
ffffffff8000007e:	b9 00 01 00 00       	mov    $0x100,%ecx
ffffffff80000083:	48 c7 c7 00 78 00 80 	mov    $0xffffffff80007800,%rdi
ffffffff8000008a:	45 31 c0             	xor    %r8d,%r8d
ffffffff8000008d:	e8 b8 05 00 00       	call   ffffffff8000064a <silicium_x86_64::paging::pml4::recursive_copy>
ffffffff80000092:	49 bc fc 0e 00 00 00 	movabs $0xfff0000000000efc,%r12
ffffffff80000099:	00 f0 ff 
ffffffff8000009c:	49 bd 00 f0 ff ff ff 	movabs $0xffffffffff000,%r13
ffffffff800000a3:	ff 0f 00 
ffffffff800000a6:	4a 8b 2c fd 00 78 00 	mov    -0x7fff8800(,%r15,8),%rbp
ffffffff800000ad:	80 
ffffffff800000ae:	40 f6 c5 01          	test   $0x1,%bpl
ffffffff800000b2:	75 3a                	jne    ffffffff800000ee <_start+0xd6>
ffffffff800000b4:	bf 00 10 00 00       	mov    $0x1000,%edi
ffffffff800000b9:	be 00 10 00 00       	mov    $0x1000,%esi
ffffffff800000be:	e8 cc 04 00 00       	call   ffffffff8000058f <silicium_x86_64::boot::allocate_align_physical>
ffffffff800000c3:	48 89 c3             	mov    %rax,%rbx
ffffffff800000c6:	4a 8d 3c 30          	lea    (%rax,%r14,1),%rdi
ffffffff800000ca:	ba 00 10 00 00       	mov    $0x1000,%edx
ffffffff800000cf:	31 f6                	xor    %esi,%esi
ffffffff800000d1:	e8 66 21 00 00       	call   ffffffff8000223c <memset>
ffffffff800000d6:	4c 21 e5             	and    %r12,%rbp
ffffffff800000d9:	4c 21 eb             	and    %r13,%rbx
ffffffff800000dc:	48 8d 04 2b          	lea    (%rbx,%rbp,1),%rax
ffffffff800000e0:	48 05 03 01 00 00    	add    $0x103,%rax
ffffffff800000e6:	4a 89 04 fd 00 78 00 	mov    %rax,-0x7fff8800(,%r15,8)
ffffffff800000ed:	80 
ffffffff800000ee:	49 ff c7             	inc    %r15
ffffffff800000f1:	49 81 ff 00 01 00 00 	cmp    $0x100,%r15
ffffffff800000f8:	75 ac                	jne    ffffffff800000a6 <_start+0x8e>
ffffffff800000fa:	65 48 8b 04 25 00 00 	mov    %gs:0x0,%rax
ffffffff80000101:	00 00 
ffffffff80000103:	48 c7 c1 10 09 00 80 	mov    $0xffffffff80000910,%rcx
ffffffff8000010a:	48 f7 d9             	neg    %rcx
ffffffff8000010d:	48 01 c8             	add    %rcx,%rax
ffffffff80000110:	48 05 30 09 00 80    	add    $0xffffffff80000930,%rax
ffffffff80000116:	66 c7 44 24 08 3f 00 	movw   $0x3f,0x8(%rsp)
ffffffff8000011d:	48 89 44 24 0a       	mov    %rax,0xa(%rsp)
ffffffff80000122:	4c 8d 7c 24 08       	lea    0x8(%rsp),%r15
ffffffff80000127:	41 0f 01 17          	lgdt   (%r15)
ffffffff8000012b:	e8 d0 fe ff ff       	call   ffffffff80000000 <__init_start>
ffffffff80000130:	48 c7 c0 60 10 00 80 	mov    $0xffffffff80001060,%rax
ffffffff80000137:	31 c9                	xor    %ecx,%ecx
ffffffff80000139:	89 c2                	mov    %eax,%edx
ffffffff8000013b:	c1 ea 10             	shr    $0x10,%edx
ffffffff8000013e:	48 89 c6             	mov    %rax,%rsi
ffffffff80000141:	48 c1 ee 20          	shr    $0x20,%rsi
ffffffff80000145:	66 89 81 38 40 00 80 	mov    %ax,-0x7fffbfc8(%rcx)
ffffffff8000014c:	c7 81 3a 40 00 80 08 	movl   $0x8e000008,-0x7fffbfc6(%rcx)
ffffffff80000153:	00 00 8e 
ffffffff80000156:	66 89 91 3e 40 00 80 	mov    %dx,-0x7fffbfc2(%rcx)
ffffffff8000015d:	89 b1 40 40 00 80    	mov    %esi,-0x7fffbfc0(%rcx)
ffffffff80000163:	c7 81 44 40 00 80 00 	movl   $0x0,-0x7fffbfbc(%rcx)
ffffffff8000016a:	00 00 00 
ffffffff8000016d:	48 8d 51 10          	lea    0x10(%rcx),%rdx
ffffffff80000171:	48 83 c0 10          	add    $0x10,%rax
ffffffff80000175:	48 89 d1             	mov    %rdx,%rcx
ffffffff80000178:	48 81 fa 00 10 00 00 	cmp    $0x1000,%rdx
ffffffff8000017f:	75 b8                	jne    ffffffff80000139 <_start+0x121>
ffffffff80000181:	4c 8d 74 24 0f       	lea    0xf(%rsp),%r14
ffffffff80000186:	66 41 c7 46 f9 ff 0f 	movw   $0xfff,-0x7(%r14)
ffffffff8000018d:	49 c7 46 fb 38 40 00 	movq   $0xffffffff80004038,-0x5(%r14)
ffffffff80000194:	80 
ffffffff80000195:	41 0f 01 1f          	lidt   (%r15)
ffffffff80000199:	65 48 8b 04 25 00 00 	mov    %gs:0x0,%rax
ffffffff800001a0:	00 00 
ffffffff800001a2:	48 c7 c1 10 09 00 80 	mov    $0xffffffff80000910,%rcx
ffffffff800001a9:	48 29 c8             	sub    %rcx,%rax
ffffffff800001ac:	48 8d 80 70 09 00 80 	lea    -0x7ffff690(%rax),%rax
ffffffff800001b3:	49 89 46 f9          	mov    %rax,-0x7(%r14)
ffffffff800001b7:	44 89 f8             	mov    %r15d,%eax
ffffffff800001ba:	25 00 00 00 ff       	and    $0xff000000,%eax
ffffffff800001bf:	48 c1 e0 20          	shl    $0x20,%rax
ffffffff800001c3:	44 89 fa             	mov    %r15d,%edx
ffffffff800001c6:	81 e2 f8 ff ff 00    	and    $0xfffff8,%edx
ffffffff800001cc:	48 c1 e2 10          	shl    $0x10,%rdx
ffffffff800001d0:	48 09 c2             	or     %rax,%rdx
ffffffff800001d3:	48 b8 67 00 00 00 00 	movabs $0x890000000067,%rax
ffffffff800001da:	89 00 00 
ffffffff800001dd:	48 09 d0             	or     %rdx,%rax
ffffffff800001e0:	65 48 8b 14 25 00 00 	mov    %gs:0x0,%rdx
ffffffff800001e7:	00 00 
ffffffff800001e9:	48 c7 c6 30 09 00 80 	mov    $0xffffffff80000930,%rsi
ffffffff800001f0:	48 29 ce             	sub    %rcx,%rsi
ffffffff800001f3:	48 89 44 32 30       	mov    %rax,0x30(%rdx,%rsi,1)
ffffffff800001f8:	4c 89 f8             	mov    %r15,%rax
ffffffff800001fb:	48 c1 e8 20          	shr    $0x20,%rax
ffffffff800001ff:	65 48 8b 0c 25 00 00 	mov    %gs:0x0,%rcx
ffffffff80000206:	00 00 
ffffffff80000208:	48 89 44 31 38       	mov    %rax,0x38(%rcx,%rsi,1)
ffffffff8000020d:	66 b8 30 00          	mov    $0x30,%ax
ffffffff80000211:	0f 00 d8             	ltr    %ax
ffffffff80000214:	e8 c9 05 00 00       	call   ffffffff800007e2 <silicium_x86_64::simd::setup>
ffffffff80000219:	31 c0                	xor    %eax,%eax
ffffffff8000021b:	31 c9                	xor    %ecx,%ecx
ffffffff8000021d:	48 89 db             	mov    %rbx,%rbx
ffffffff80000220:	0f a2                	cpuid
ffffffff80000222:	48 87 db             	xchg   %rbx,%rbx
ffffffff80000225:	89 d8                	mov    %ebx,%eax
ffffffff80000227:	41 88 5e f9          	mov    %bl,-0x7(%r14)
ffffffff8000022b:	88 7c 24 09          	mov    %bh,0x9(%rsp)
ffffffff8000022f:	c1 eb 10             	shr    $0x10,%ebx
ffffffff80000232:	c1 e8 18             	shr    $0x18,%eax
ffffffff80000235:	89 d6                	mov    %edx,%esi
ffffffff80000237:	41 88 5e fb          	mov    %bl,-0x5(%r14)
ffffffff8000023b:	41 88 46 fc          	mov    %al,-0x4(%r14)
ffffffff8000023f:	41 88 56 fd          	mov    %dl,-0x3(%r14)
ffffffff80000243:	88 74 24 0d          	mov    %dh,0xd(%rsp)
ffffffff80000247:	c1 ea 10             	shr    $0x10,%edx
ffffffff8000024a:	c1 ee 18             	shr    $0x18,%esi
ffffffff8000024d:	89 c8                	mov    %ecx,%eax
ffffffff8000024f:	41 88 56 ff          	mov    %dl,-0x1(%r14)
ffffffff80000253:	41 88 36             	mov    %sil,(%r14)
ffffffff80000256:	41 88 4e 01          	mov    %cl,0x1(%r14)
ffffffff8000025a:	88 6c 24 11          	mov    %ch,0x11(%rsp)
ffffffff8000025e:	c1 e9 10             	shr    $0x10,%ecx
ffffffff80000261:	c1 e8 18             	shr    $0x18,%eax
ffffffff80000264:	41 88 4e 03          	mov    %cl,0x3(%r14)
ffffffff80000268:	41 88 46 04          	mov    %al,0x4(%r14)
ffffffff8000026c:	31 c9                	xor    %ecx,%ecx
ffffffff8000026e:	31 c0                	xor    %eax,%eax
ffffffff80000270:	48 89 de             	mov    %rbx,%rsi
ffffffff80000273:	0f a2                	cpuid
ffffffff80000275:	48 87 f3             	xchg   %rsi,%rbx
ffffffff80000278:	31 c0                	xor    %eax,%eax
ffffffff8000027a:	49 83 e6 f8          	and    $0xfffffffffffffff8,%r14
ffffffff8000027e:	4d 29 fe             	sub    %r15,%r14
ffffffff80000281:	0f b6 4c 04 08       	movzbl 0x8(%rsp,%rax,1),%ecx
ffffffff80000286:	84 c9                	test   %cl,%cl
ffffffff80000288:	78 29                	js     ffffffff800002b3 <_start+0x29b>
ffffffff8000028a:	44 89 f1             	mov    %r14d,%ecx
ffffffff8000028d:	29 c1                	sub    %eax,%ecx
ffffffff8000028f:	f6 c1 07             	test   $0x7,%cl
ffffffff80000292:	0f 85 0f 01 00 00    	jne    ffffffff800003a7 <_start+0x38f>
ffffffff80000298:	eb 0d                	jmp    ffffffff800002a7 <_start+0x28f>
ffffffff8000029a:	48 ff c0             	inc    %rax
ffffffff8000029d:	48 83 f8 0c          	cmp    $0xc,%rax
ffffffff800002a1:	0f 84 0d 01 00 00    	je     ffffffff800003b4 <_start+0x39c>
ffffffff800002a7:	80 7c 04 08 00       	cmpb   $0x0,0x8(%rsp,%rax,1)
ffffffff800002ac:	79 ec                	jns    ffffffff8000029a <_start+0x282>
ffffffff800002ae:	e9 f7 00 00 00       	jmp    ffffffff800003aa <_start+0x392>
ffffffff800002b3:	0f b6 91 40 30 00 80 	movzbl -0x7fffcfc0(%rcx),%edx
ffffffff800002ba:	83 fa 02             	cmp    $0x2,%edx
ffffffff800002bd:	74 69                	je     ffffffff80000328 <_start+0x310>
ffffffff800002bf:	83 fa 03             	cmp    $0x3,%edx
ffffffff800002c2:	74 3b                	je     ffffffff800002ff <_start+0x2e7>
ffffffff800002c4:	83 fa 04             	cmp    $0x4,%edx
ffffffff800002c7:	0f 85 e7 00 00 00    	jne    ffffffff800003b4 <_start+0x39c>
ffffffff800002cd:	48 83 f8 0b          	cmp    $0xb,%rax
ffffffff800002d1:	0f 83 dd 00 00 00    	jae    ffffffff800003b4 <_start+0x39c>
ffffffff800002d7:	42 8a 54 38 01       	mov    0x1(%rax,%r15,1),%dl
ffffffff800002dc:	48 81 f9 f0 00 00 00 	cmp    $0xf0,%rcx
ffffffff800002e3:	74 63                	je     ffffffff80000348 <_start+0x330>
ffffffff800002e5:	81 f9 f4 00 00 00    	cmp    $0xf4,%ecx
ffffffff800002eb:	0f 85 8a 00 00 00    	jne    ffffffff8000037b <_start+0x363>
ffffffff800002f1:	80 fa 8f             	cmp    $0x8f,%dl
ffffffff800002f4:	0f 8e 8e 00 00 00    	jle    ffffffff80000388 <_start+0x370>
ffffffff800002fa:	e9 b5 00 00 00       	jmp    ffffffff800003b4 <_start+0x39c>
ffffffff800002ff:	48 83 f8 0b          	cmp    $0xb,%rax
ffffffff80000303:	0f 83 ab 00 00 00    	jae    ffffffff800003b4 <_start+0x39c>
ffffffff80000309:	42 8a 54 38 01       	mov    0x1(%rax,%r15,1),%dl
ffffffff8000030e:	81 f9 e0 00 00 00    	cmp    $0xe0,%ecx
ffffffff80000314:	74 28                	je     ffffffff8000033e <_start+0x326>
ffffffff80000316:	81 f9 ed 00 00 00    	cmp    $0xed,%ecx
ffffffff8000031c:	75 34                	jne    ffffffff80000352 <_start+0x33a>
ffffffff8000031e:	80 fa 9f             	cmp    $0x9f,%dl
ffffffff80000321:	7e 45                	jle    ffffffff80000368 <_start+0x350>
ffffffff80000323:	e9 8c 00 00 00       	jmp    ffffffff800003b4 <_start+0x39c>
ffffffff80000328:	48 83 f8 0b          	cmp    $0xb,%rax
ffffffff8000032c:	0f 83 82 00 00 00    	jae    ffffffff800003b4 <_start+0x39c>
ffffffff80000332:	80 7c 04 09 bf       	cmpb   $0xbf,0x9(%rsp,%rax,1)
ffffffff80000337:	7f 7b                	jg     ffffffff800003b4 <_start+0x39c>
ffffffff80000339:	48 ff c0             	inc    %rax
ffffffff8000033c:	eb 69                	jmp    ffffffff800003a7 <_start+0x38f>
ffffffff8000033e:	80 e2 e0             	and    $0xe0,%dl
ffffffff80000341:	80 fa a0             	cmp    $0xa0,%dl
ffffffff80000344:	74 22                	je     ffffffff80000368 <_start+0x350>
ffffffff80000346:	eb 6c                	jmp    ffffffff800003b4 <_start+0x39c>
ffffffff80000348:	80 c2 40             	add    $0x40,%dl
ffffffff8000034b:	80 fa d0             	cmp    $0xd0,%dl
ffffffff8000034e:	73 38                	jae    ffffffff80000388 <_start+0x370>
ffffffff80000350:	eb 62                	jmp    ffffffff800003b4 <_start+0x39c>
ffffffff80000352:	8d 71 1f             	lea    0x1f(%rcx),%esi
ffffffff80000355:	40 80 fe 0c          	cmp    $0xc,%sil
ffffffff80000359:	72 08                	jb     ffffffff80000363 <_start+0x34b>
ffffffff8000035b:	80 e1 fe             	and    $0xfe,%cl
ffffffff8000035e:	80 f9 ee             	cmp    $0xee,%cl
ffffffff80000361:	75 51                	jne    ffffffff800003b4 <_start+0x39c>
ffffffff80000363:	80 fa bf             	cmp    $0xbf,%dl
ffffffff80000366:	7f 4c                	jg     ffffffff800003b4 <_start+0x39c>
ffffffff80000368:	48 83 f8 0a          	cmp    $0xa,%rax
ffffffff8000036c:	74 46                	je     ffffffff800003b4 <_start+0x39c>
ffffffff8000036e:	80 7c 04 0a bf       	cmpb   $0xbf,0xa(%rsp,%rax,1)
ffffffff80000373:	7f 3f                	jg     ffffffff800003b4 <_start+0x39c>
ffffffff80000375:	48 83 c0 02          	add    $0x2,%rax
ffffffff80000379:	eb 2c                	jmp    ffffffff800003a7 <_start+0x38f>
ffffffff8000037b:	83 c1 0c             	add    $0xc,%ecx
ffffffff8000037e:	80 f9 fd             	cmp    $0xfd,%cl
ffffffff80000381:	72 31                	jb     ffffffff800003b4 <_start+0x39c>
ffffffff80000383:	80 fa bf             	cmp    $0xbf,%dl
ffffffff80000386:	7f 2c                	jg     ffffffff800003b4 <_start+0x39c>
ffffffff80000388:	48 83 f8 0a          	cmp    $0xa,%rax
ffffffff8000038c:	74 26                	je     ffffffff800003b4 <_start+0x39c>
ffffffff8000038e:	42 80 7c 38 02 bf    	cmpb   $0xbf,0x2(%rax,%r15,1)
ffffffff80000394:	7f 1e                	jg     ffffffff800003b4 <_start+0x39c>
ffffffff80000396:	48 83 f8 08          	cmp    $0x8,%rax
ffffffff8000039a:	77 18                	ja     ffffffff800003b4 <_start+0x39c>
ffffffff8000039c:	80 7c 04 0b c0       	cmpb   $0xc0,0xb(%rsp,%rax,1)
ffffffff800003a1:	7d 11                	jge    ffffffff800003b4 <_start+0x39c>
ffffffff800003a3:	48 83 c0 03          	add    $0x3,%rax
ffffffff800003a7:	48 ff c0             	inc    %rax
ffffffff800003aa:	48 83 f8 0c          	cmp    $0xc,%rax
ffffffff800003ae:	0f 82 cd fe ff ff    	jb     ffffffff80000281 <_start+0x269>
ffffffff800003b4:	48 b9 00 00 c0 fe 00 	movabs $0xffff8000fec00000,%rcx
ffffffff800003bb:	80 ff ff 
ffffffff800003be:	66 ba 20 00          	mov    $0x20,%dx
ffffffff800003c2:	b0 11                	mov    $0x11,%al
ffffffff800003c4:	ee                   	out    %al,(%dx)
ffffffff800003c5:	31 f6                	xor    %esi,%esi
ffffffff800003c7:	66 ba 80 00          	mov    $0x80,%dx
ffffffff800003cb:	31 c0                	xor    %eax,%eax
ffffffff800003cd:	ee                   	out    %al,(%dx)
ffffffff800003ce:	66 ba a0 00          	mov    $0xa0,%dx
ffffffff800003d2:	b0 11                	mov    $0x11,%al
ffffffff800003d4:	ee                   	out    %al,(%dx)
ffffffff800003d5:	66 ba 80 00          	mov    $0x80,%dx
ffffffff800003d9:	31 c0                	xor    %eax,%eax
ffffffff800003db:	ee                   	out    %al,(%dx)
ffffffff800003dc:	66 ba 21 00          	mov    $0x21,%dx
ffffffff800003e0:	b0 20                	mov    $0x20,%al
ffffffff800003e2:	ee                   	out    %al,(%dx)
ffffffff800003e3:	66 ba 80 00          	mov    $0x80,%dx
ffffffff800003e7:	31 c0                	xor    %eax,%eax
ffffffff800003e9:	ee                   	out    %al,(%dx)
ffffffff800003ea:	66 ba a1 00          	mov    $0xa1,%dx
ffffffff800003ee:	b0 28                	mov    $0x28,%al
ffffffff800003f0:	ee                   	out    %al,(%dx)
ffffffff800003f1:	66 ba 80 00          	mov    $0x80,%dx
ffffffff800003f5:	31 c0                	xor    %eax,%eax
ffffffff800003f7:	ee                   	out    %al,(%dx)
ffffffff800003f8:	66 ba 21 00          	mov    $0x21,%dx
ffffffff800003fc:	b0 04                	mov    $0x4,%al
ffffffff800003fe:	ee                   	out    %al,(%dx)
ffffffff800003ff:	66 ba 80 00          	mov    $0x80,%dx
ffffffff80000403:	31 c0                	xor    %eax,%eax
ffffffff80000405:	ee                   	out    %al,(%dx)
ffffffff80000406:	66 ba a1 00          	mov    $0xa1,%dx
ffffffff8000040a:	b0 02                	mov    $0x2,%al
ffffffff8000040c:	ee                   	out    %al,(%dx)
ffffffff8000040d:	66 ba 80 00          	mov    $0x80,%dx
ffffffff80000411:	31 c0                	xor    %eax,%eax
ffffffff80000413:	ee                   	out    %al,(%dx)
ffffffff80000414:	66 ba 21 00          	mov    $0x21,%dx
ffffffff80000418:	b0 01                	mov    $0x1,%al
ffffffff8000041a:	ee                   	out    %al,(%dx)
ffffffff8000041b:	66 ba 80 00          	mov    $0x80,%dx
ffffffff8000041f:	31 c0                	xor    %eax,%eax
ffffffff80000421:	ee                   	out    %al,(%dx)
ffffffff80000422:	66 ba a1 00          	mov    $0xa1,%dx
ffffffff80000426:	b0 01                	mov    $0x1,%al
ffffffff80000428:	ee                   	out    %al,(%dx)
ffffffff80000429:	66 ba 80 00          	mov    $0x80,%dx
ffffffff8000042d:	31 c0                	xor    %eax,%eax
ffffffff8000042f:	ee                   	out    %al,(%dx)
ffffffff80000430:	66 ba 21 00          	mov    $0x21,%dx
ffffffff80000434:	b0 ff                	mov    $0xff,%al
ffffffff80000436:	ee                   	out    %al,(%dx)
ffffffff80000437:	66 ba 80 00          	mov    $0x80,%dx
ffffffff8000043b:	31 c0                	xor    %eax,%eax
ffffffff8000043d:	ee                   	out    %al,(%dx)
ffffffff8000043e:	66 ba a1 00          	mov    $0xa1,%dx
ffffffff80000442:	b0 ff                	mov    $0xff,%al
ffffffff80000444:	ee                   	out    %al,(%dx)
ffffffff80000445:	66 ba 80 00          	mov    $0x80,%dx
ffffffff80000449:	31 c0                	xor    %eax,%eax
ffffffff8000044b:	ee                   	out    %al,(%dx)
ffffffff8000044c:	c7 81 f0 00 20 00 ff 	movl   $0x1ff,0x2000f0(%rcx)
ffffffff80000453:	01 00 00 
ffffffff80000456:	c7 01 01 00 00 00    	movl   $0x1,(%rcx)
ffffffff8000045c:	8b 41 10             	mov    0x10(%rcx),%eax
ffffffff8000045f:	c1 e8 10             	shr    $0x10,%eax
ffffffff80000462:	fe c0                	inc    %al
ffffffff80000464:	88 05 96 5b 00 00    	mov    %al,0x5b96(%rip)        # ffffffff80006000 <silicium_x86_64::apic::io::IRQ_COUNT>
ffffffff8000046a:	75 0c                	jne    ffffffff80000478 <_start+0x460>
ffffffff8000046c:	c7 81 b0 00 20 00 00 	movl   $0x0,0x2000b0(%rcx)
ffffffff80000473:	00 00 00 
ffffffff80000476:	eb 42                	jmp    ffffffff800004ba <_start+0x4a2>
ffffffff80000478:	0f b6 c0             	movzbl %al,%eax
ffffffff8000047b:	01 c0                	add    %eax,%eax
ffffffff8000047d:	8d 56 11             	lea    0x11(%rsi),%edx
ffffffff80000480:	89 11                	mov    %edx,(%rcx)
ffffffff80000482:	c7 41 10 00 00 00 00 	movl   $0x0,0x10(%rcx)
ffffffff80000489:	8d 56 10             	lea    0x10(%rsi),%edx
ffffffff8000048c:	89 11                	mov    %edx,(%rcx)
ffffffff8000048e:	c7 41 10 00 00 01 00 	movl   $0x10000,0x10(%rcx)
ffffffff80000495:	83 c6 02             	add    $0x2,%esi
ffffffff80000498:	39 f0                	cmp    %esi,%eax
ffffffff8000049a:	75 e1                	jne    ffffffff8000047d <_start+0x465>
ffffffff8000049c:	31 c0                	xor    %eax,%eax
ffffffff8000049e:	89 81 b0 00 20 00    	mov    %eax,0x2000b0(%rcx)
ffffffff800004a4:	c7 01 11 00 00 00    	movl   $0x11,(%rcx)
ffffffff800004aa:	89 41 10             	mov    %eax,0x10(%rcx)
ffffffff800004ad:	c7 01 10 00 00 00    	movl   $0x10,(%rcx)
ffffffff800004b3:	c7 41 10 20 00 00 00 	movl   $0x20,0x10(%rcx)
ffffffff800004ba:	c7 81 20 03 20 00 20 	movl   $0x20020,0x200320(%rcx)
ffffffff800004c1:	00 02 00 
ffffffff800004c4:	c7 81 e0 03 20 00 03 	movl   $0x3,0x2003e0(%rcx)
ffffffff800004cb:	00 00 00 
ffffffff800004ce:	c7 81 80 03 20 00 ff 	movl   $0xffffffff,0x200380(%rcx)
ffffffff800004d5:	ff ff ff 
ffffffff800004d8:	66 ba 61 00          	mov    $0x61,%dx
ffffffff800004dc:	ec                   	in     (%dx),%al
ffffffff800004dd:	24 01                	and    $0x1,%al
ffffffff800004df:	ee                   	out    %al,(%dx)
ffffffff800004e0:	66 ba 43 00          	mov    $0x43,%dx
ffffffff800004e4:	b0 b2                	mov    $0xb2,%al
ffffffff800004e6:	ee                   	out    %al,(%dx)
ffffffff800004e7:	66 ba 42 00          	mov    $0x42,%dx
ffffffff800004eb:	b0 9d                	mov    $0x9d,%al
ffffffff800004ed:	ee                   	out    %al,(%dx)
ffffffff800004ee:	b0 2e                	mov    $0x2e,%al
ffffffff800004f0:	ee                   	out    %al,(%dx)
ffffffff800004f1:	66 ba 61 00          	mov    $0x61,%dx
ffffffff800004f5:	ec                   	in     (%dx),%al
ffffffff800004f6:	0c 01                	or     $0x1,%al
ffffffff800004f8:	ee                   	out    %al,(%dx)
ffffffff800004f9:	ec                   	in     (%dx),%al
ffffffff800004fa:	a8 20                	test   $0x20,%al
ffffffff800004fc:	74 05                	je     ffffffff80000503 <_start+0x4eb>
ffffffff800004fe:	f3 90                	pause
ffffffff80000500:	ec                   	in     (%dx),%al
ffffffff80000501:	eb f7                	jmp    ffffffff800004fa <_start+0x4e2>
ffffffff80000503:	8b 81 90 03 20 00    	mov    0x200390(%rcx),%eax
ffffffff80000509:	f7 d0                	not    %eax
ffffffff8000050b:	6b c0 64             	imul   $0x64,%eax,%eax
ffffffff8000050e:	48 69 c0 d3 4d 62 10 	imul   $0x10624dd3,%rax,%rax
ffffffff80000515:	48 c1 e8 26          	shr    $0x26,%rax
ffffffff80000519:	89 81 80 03 20 00    	mov    %eax,0x200380(%rcx)
ffffffff8000051f:	48 8b 05 3a 4b 00 00 	mov    0x4b3a(%rip),%rax        # ffffffff80005060 <silicium_x86_64::smp::SMP_REQUEST+0x28>
ffffffff80000526:	48 85 c0             	test   %rax,%rax
ffffffff80000529:	74 53                	je     ffffffff8000057e <_start+0x566>
ffffffff8000052b:	48 8b 48 10          	mov    0x10(%rax),%rcx
ffffffff8000052f:	48 85 c9             	test   %rcx,%rcx
ffffffff80000532:	74 28                	je     ffffffff8000055c <_start+0x544>
ffffffff80000534:	48 8b 50 18          	mov    0x18(%rax),%rdx
ffffffff80000538:	31 f6                	xor    %esi,%esi
ffffffff8000053a:	48 c7 c7 2b 08 00 80 	mov    $0xffffffff8000082b,%rdi
ffffffff80000541:	4c 8b 04 f2          	mov    (%rdx,%rsi,8),%r8
ffffffff80000545:	41 83 78 04 00       	cmpl   $0x0,0x4(%r8)
ffffffff8000054a:	74 04                	je     ffffffff80000550 <_start+0x538>
ffffffff8000054c:	49 89 78 10          	mov    %rdi,0x10(%r8)
ffffffff80000550:	48 ff c6             	inc    %rsi
ffffffff80000553:	48 39 f1             	cmp    %rsi,%rcx
ffffffff80000556:	75 e9                	jne    ffffffff80000541 <_start+0x529>
ffffffff80000558:	eb 02                	jmp    ffffffff8000055c <_start+0x544>
ffffffff8000055a:	f3 90                	pause
ffffffff8000055c:	48 8b 0d 0d 4b 00 00 	mov    0x4b0d(%rip),%rcx        # ffffffff80005070 <silicium_x86_64::smp::CPU_COUNT>
ffffffff80000563:	48 3b 48 10          	cmp    0x10(%rax),%rcx
ffffffff80000567:	75 f1                	jne    ffffffff8000055a <_start+0x542>
ffffffff80000569:	c6 05 90 8a 00 00 01 	movb   $0x1,0x8a90(%rip)        # ffffffff80009000 <silicium_x86_64::smp::AP_BOOTED>
ffffffff80000570:	e8 6b 1b 00 00       	call   ffffffff800020e0 <silicium_x86_64::paging::pml4::Pml4::set_current>
ffffffff80000575:	fb                   	sti
ffffffff80000576:	f4                   	hlt
ffffffff80000577:	eb fd                	jmp    ffffffff80000576 <_start+0x55e>
ffffffff80000579:	e8 de 1a 00 00       	call   ffffffff8000205c <core::panicking::panic_fmt>
ffffffff8000057e:	be 1b 00 00 00       	mov    $0x1b,%esi
ffffffff80000583:	48 c7 c7 6c 31 00 80 	mov    $0xffffffff8000316c,%rdi
ffffffff8000058a:	e8 d3 1a 00 00       	call   ffffffff80002062 <core::option::expect_failed>

ffffffff8000058f <silicium_x86_64::boot::allocate_align_physical>:
ffffffff8000058f:	50                   	push   %rax
ffffffff80000590:	b1 01                	mov    $0x1,%cl
ffffffff80000592:	31 c0                	xor    %eax,%eax
ffffffff80000594:	f0 0f b0 0d 64 3a 00 	lock cmpxchg %cl,0x3a64(%rip)        # ffffffff80004000 <silicium_x86_64::boot::MMAP_REQUEST>
ffffffff8000059b:	00 
ffffffff8000059c:	75 04                	jne    ffffffff800005a2 <silicium_x86_64::boot::allocate_align_physical+0x13>
ffffffff8000059e:	eb 0e                	jmp    ffffffff800005ae <silicium_x86_64::boot::allocate_align_physical+0x1f>
ffffffff800005a0:	f3 90                	pause
ffffffff800005a2:	8a 05 58 3a 00 00    	mov    0x3a58(%rip),%al        # ffffffff80004000 <silicium_x86_64::boot::MMAP_REQUEST>
ffffffff800005a8:	84 c0                	test   %al,%al
ffffffff800005aa:	75 f4                	jne    ffffffff800005a0 <silicium_x86_64::boot::allocate_align_physical+0x11>
ffffffff800005ac:	eb e4                	jmp    ffffffff80000592 <silicium_x86_64::boot::allocate_align_physical+0x3>
ffffffff800005ae:	48 8b 0d 7b 3a 00 00 	mov    0x3a7b(%rip),%rcx        # ffffffff80004030 <silicium_x86_64::boot::MMAP_REQUEST+0x30>
ffffffff800005b5:	48 85 c9             	test   %rcx,%rcx
ffffffff800005b8:	74 7a                	je     ffffffff80000634 <silicium_x86_64::boot::allocate_align_physical+0xa5>
ffffffff800005ba:	48 8d 46 ff          	lea    -0x1(%rsi),%rax
ffffffff800005be:	48 89 f2             	mov    %rsi,%rdx
ffffffff800005c1:	48 31 c2             	xor    %rax,%rdx
ffffffff800005c4:	48 39 c2             	cmp    %rax,%rdx
ffffffff800005c7:	76 7c                	jbe    ffffffff80000645 <silicium_x86_64::boot::allocate_align_physical+0xb6>
ffffffff800005c9:	48 8b 51 08          	mov    0x8(%rcx),%rdx
ffffffff800005cd:	48 85 d2             	test   %rdx,%rdx
ffffffff800005d0:	74 29                	je     ffffffff800005fb <silicium_x86_64::boot::allocate_align_physical+0x6c>
ffffffff800005d2:	48 8b 49 10          	mov    0x10(%rcx),%rcx
ffffffff800005d6:	4c 8d 04 3e          	lea    (%rsi,%rdi,1),%r8
ffffffff800005da:	48 c1 e2 03          	shl    $0x3,%rdx
ffffffff800005de:	4c 8b 09             	mov    (%rcx),%r9
ffffffff800005e1:	49 83 79 10 00       	cmpq   $0x0,0x10(%r9)
ffffffff800005e6:	75 09                	jne    ffffffff800005f1 <silicium_x86_64::boot::allocate_align_physical+0x62>
ffffffff800005e8:	4d 8b 51 08          	mov    0x8(%r9),%r10
ffffffff800005ec:	4d 39 c2             	cmp    %r8,%r10
ffffffff800005ef:	73 1b                	jae    ffffffff8000060c <silicium_x86_64::boot::allocate_align_physical+0x7d>
ffffffff800005f1:	48 83 c1 08          	add    $0x8,%rcx
ffffffff800005f5:	48 83 c2 f8          	add    $0xfffffffffffffff8,%rdx
ffffffff800005f9:	75 e3                	jne    ffffffff800005de <silicium_x86_64::boot::allocate_align_physical+0x4f>
ffffffff800005fb:	be 2e 00 00 00       	mov    $0x2e,%esi
ffffffff80000600:	48 c7 c7 ab 31 00 80 	mov    $0xffffffff800031ab,%rdi
ffffffff80000607:	e8 56 1a 00 00       	call   ffffffff80002062 <core::option::expect_failed>
ffffffff8000060c:	49 8b 11             	mov    (%r9),%rdx
ffffffff8000060f:	48 21 d0             	and    %rdx,%rax
ffffffff80000612:	48 29 c6             	sub    %rax,%rsi
ffffffff80000615:	48 01 f7             	add    %rsi,%rdi
ffffffff80000618:	49 29 fa             	sub    %rdi,%r10
ffffffff8000061b:	4d 89 51 08          	mov    %r10,0x8(%r9)
ffffffff8000061f:	48 8b 01             	mov    (%rcx),%rax
ffffffff80000622:	48 01 38             	add    %rdi,(%rax)
ffffffff80000625:	48 01 d6             	add    %rdx,%rsi
ffffffff80000628:	c6 05 d1 39 00 00 00 	movb   $0x0,0x39d1(%rip)        # ffffffff80004000 <silicium_x86_64::boot::MMAP_REQUEST>
ffffffff8000062f:	48 89 f0             	mov    %rsi,%rax
ffffffff80000632:	59                   	pop    %rcx
ffffffff80000633:	c3                   	ret
ffffffff80000634:	be 24 00 00 00       	mov    $0x24,%esi
ffffffff80000639:	48 c7 c7 87 31 00 80 	mov    $0xffffffff80003187,%rdi
ffffffff80000640:	e8 1d 1a 00 00       	call   ffffffff80002062 <core::option::expect_failed>
ffffffff80000645:	e8 12 1a 00 00       	call   ffffffff8000205c <core::panicking::panic_fmt>

ffffffff8000064a <silicium_x86_64::paging::pml4::recursive_copy>:
ffffffff8000064a:	55                   	push   %rbp
ffffffff8000064b:	41 57                	push   %r15
ffffffff8000064d:	41 56                	push   %r14
ffffffff8000064f:	41 55                	push   %r13
ffffffff80000651:	41 54                	push   %r12
ffffffff80000653:	53                   	push   %rbx
ffffffff80000654:	48 83 ec 28          	sub    $0x28,%rsp
ffffffff80000658:	48 89 fb             	mov    %rdi,%rbx
ffffffff8000065b:	48 39 ce             	cmp    %rcx,%rsi
ffffffff8000065e:	48 0f 42 ce          	cmovb  %rsi,%rcx
ffffffff80000662:	45 31 ff             	xor    %r15d,%r15d
ffffffff80000665:	41 0f b6 c0          	movzbl %r8b,%eax
ffffffff80000669:	48 89 44 24 20       	mov    %rax,0x20(%rsp)
ffffffff8000066e:	48 89 4c 24 18       	mov    %rcx,0x18(%rsp)
ffffffff80000673:	48 89 54 24 10       	mov    %rdx,0x10(%rsp)
ffffffff80000678:	49 39 cf             	cmp    %rcx,%r15
ffffffff8000067b:	0f 83 cd 00 00 00    	jae    ffffffff8000074e <silicium_x86_64::paging::pml4::recursive_copy+0x104>
ffffffff80000681:	4e 8b 24 fa          	mov    (%rdx,%r15,8),%r12
ffffffff80000685:	41 f6 c4 01          	test   $0x1,%r12b
ffffffff80000689:	0f 84 b7 00 00 00    	je     ffffffff80000746 <silicium_x86_64::paging::pml4::recursive_copy+0xfc>
ffffffff8000068f:	4e 89 24 fb          	mov    %r12,(%rbx,%r15,8)
ffffffff80000693:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
ffffffff80000698:	ff 24 c5 20 30 00 80 	jmp    *-0x7fffcfe0(,%rax,8)
ffffffff8000069f:	b0 01                	mov    $0x1,%al
ffffffff800006a1:	eb 0f                	jmp    ffffffff800006b2 <silicium_x86_64::paging::pml4::recursive_copy+0x68>
ffffffff800006a3:	b0 02                	mov    $0x2,%al
ffffffff800006a5:	eb 02                	jmp    ffffffff800006a9 <silicium_x86_64::paging::pml4::recursive_copy+0x5f>
ffffffff800006a7:	b0 03                	mov    $0x3,%al
ffffffff800006a9:	45 84 e4             	test   %r12b,%r12b
ffffffff800006ac:	0f 88 9c 00 00 00    	js     ffffffff8000074e <silicium_x86_64::paging::pml4::recursive_copy+0x104>
ffffffff800006b2:	88 44 24 0f          	mov    %al,0xf(%rsp)
ffffffff800006b6:	4d 89 e6             	mov    %r12,%r14
ffffffff800006b9:	48 b8 00 f0 ff ff ff 	movabs $0xffffffffff000,%rax
ffffffff800006c0:	ff 0f 00 
ffffffff800006c3:	49 21 c6             	and    %rax,%r14
ffffffff800006c6:	bf 00 10 00 00       	mov    $0x1000,%edi
ffffffff800006cb:	be 00 10 00 00       	mov    $0x1000,%esi
ffffffff800006d0:	e8 ba fe ff ff       	call   ffffffff8000058f <silicium_x86_64::boot::allocate_align_physical>
ffffffff800006d5:	49 89 c5             	mov    %rax,%r13
ffffffff800006d8:	48 b8 00 00 00 00 00 	movabs $0xffff800000000000,%rax
ffffffff800006df:	80 ff ff 
ffffffff800006e2:	4a 8d 2c 28          	lea    (%rax,%r13,1),%rbp
ffffffff800006e6:	49 01 c6             	add    %rax,%r14
ffffffff800006e9:	ba 00 10 00 00       	mov    $0x1000,%edx
ffffffff800006ee:	48 89 ef             	mov    %rbp,%rdi
ffffffff800006f1:	4c 89 f6             	mov    %r14,%rsi
ffffffff800006f4:	e8 f9 1a 00 00       	call   ffffffff800021f2 <memcpy>
ffffffff800006f9:	48 b8 ff 0e 00 00 00 	movabs $0xfff0000000000eff,%rax
ffffffff80000700:	00 f0 ff 
ffffffff80000703:	49 21 c4             	and    %rax,%r12
ffffffff80000706:	48 b8 00 f0 ff ff ff 	movabs $0xffffffffff000,%rax
ffffffff8000070d:	ff 0f 00 
ffffffff80000710:	49 21 c5             	and    %rax,%r13
ffffffff80000713:	4b 8d 04 2c          	lea    (%r12,%r13,1),%rax
ffffffff80000717:	48 05 00 01 00 00    	add    $0x100,%rax
ffffffff8000071d:	4a 89 04 fb          	mov    %rax,(%rbx,%r15,8)
ffffffff80000721:	44 0f b6 44 24 0f    	movzbl 0xf(%rsp),%r8d
ffffffff80000727:	be 00 02 00 00       	mov    $0x200,%esi
ffffffff8000072c:	b9 00 02 00 00       	mov    $0x200,%ecx
ffffffff80000731:	48 89 ef             	mov    %rbp,%rdi
ffffffff80000734:	4c 89 f2             	mov    %r14,%rdx
ffffffff80000737:	e8 0e ff ff ff       	call   ffffffff8000064a <silicium_x86_64::paging::pml4::recursive_copy>
ffffffff8000073c:	48 8b 54 24 10       	mov    0x10(%rsp),%rdx
ffffffff80000741:	48 8b 4c 24 18       	mov    0x18(%rsp),%rcx
ffffffff80000746:	49 ff c7             	inc    %r15
ffffffff80000749:	e9 2a ff ff ff       	jmp    ffffffff80000678 <silicium_x86_64::paging::pml4::recursive_copy+0x2e>
ffffffff8000074e:	48 83 c4 28          	add    $0x28,%rsp
ffffffff80000752:	5b                   	pop    %rbx
ffffffff80000753:	41 5c                	pop    %r12
ffffffff80000755:	41 5d                	pop    %r13
ffffffff80000757:	41 5e                	pop    %r14
ffffffff80000759:	41 5f                	pop    %r15
ffffffff8000075b:	5d                   	pop    %rbp
ffffffff8000075c:	c3                   	ret

ffffffff8000075d <silicium_x86_64::percpu::setup>:
ffffffff8000075d:	41 57                	push   %r15
ffffffff8000075f:	41 56                	push   %r14
ffffffff80000761:	41 54                	push   %r12
ffffffff80000763:	53                   	push   %rbx
ffffffff80000764:	50                   	push   %rax
ffffffff80000765:	48 89 fb             	mov    %rdi,%rbx
ffffffff80000768:	48 c7 c0 10 09 00 80 	mov    $0xffffffff80000910,%rax
ffffffff8000076f:	49 c7 c6 d8 09 00 80 	mov    $0xffffffff800009d8,%r14
ffffffff80000776:	49 29 c6             	sub    %rax,%r14
ffffffff80000779:	be 10 00 00 00       	mov    $0x10,%esi
ffffffff8000077e:	4c 89 f7             	mov    %r14,%rdi
ffffffff80000781:	e8 09 fe ff ff       	call   ffffffff8000058f <silicium_x86_64::boot::allocate_align_physical>
ffffffff80000786:	49 89 c7             	mov    %rax,%r15
ffffffff80000789:	49 bc 00 00 00 00 00 	movabs $0xffff800000000000,%r12
ffffffff80000790:	80 ff ff 
ffffffff80000793:	49 01 c4             	add    %rax,%r12
ffffffff80000796:	4c 89 e7             	mov    %r12,%rdi
ffffffff80000799:	48 c7 c6 10 09 00 80 	mov    $0xffffffff80000910,%rsi
ffffffff800007a0:	4c 89 f2             	mov    %r14,%rdx
ffffffff800007a3:	e8 4a 1a 00 00       	call   ffffffff800021f2 <memcpy>
ffffffff800007a8:	b9 02 01 00 c0       	mov    $0xc0000102,%ecx
ffffffff800007ad:	31 c0                	xor    %eax,%eax
ffffffff800007af:	31 d2                	xor    %edx,%edx
ffffffff800007b1:	0f 30                	wrmsr
ffffffff800007b3:	4c 89 e2             	mov    %r12,%rdx
ffffffff800007b6:	48 c1 ea 20          	shr    $0x20,%rdx
ffffffff800007ba:	b9 01 01 00 c0       	mov    $0xc0000101,%ecx
ffffffff800007bf:	44 89 f8             	mov    %r15d,%eax
ffffffff800007c2:	0f 30                	wrmsr
ffffffff800007c4:	65 4c 89 24 25 00 00 	mov    %r12,%gs:0x0
ffffffff800007cb:	00 00 
ffffffff800007cd:	65 48 89 1c 25 08 00 	mov    %rbx,%gs:0x8
ffffffff800007d4:	00 00 
ffffffff800007d6:	48 83 c4 08          	add    $0x8,%rsp
ffffffff800007da:	5b                   	pop    %rbx
ffffffff800007db:	41 5c                	pop    %r12
ffffffff800007dd:	41 5e                	pop    %r14
ffffffff800007df:	41 5f                	pop    %r15
ffffffff800007e1:	c3                   	ret

ffffffff800007e2 <silicium_x86_64::simd::setup>:
ffffffff800007e2:	b8 01 00 00 00       	mov    $0x1,%eax
ffffffff800007e7:	31 c9                	xor    %ecx,%ecx
ffffffff800007e9:	48 89 de             	mov    %rbx,%rsi
ffffffff800007ec:	0f a2                	cpuid
ffffffff800007ee:	48 87 f3             	xchg   %rsi,%rbx
ffffffff800007f1:	0f ba e1 1a          	bt     $0x1a,%ecx
ffffffff800007f5:	73 2e                	jae    ffffffff80000825 <silicium_x86_64::simd::setup+0x43>
ffffffff800007f7:	0f 20 c0             	mov    %cr0,%rax
ffffffff800007fa:	48 83 e0 fb          	and    $0xfffffffffffffffb,%rax
ffffffff800007fe:	0f 22 c0             	mov    %rax,%cr0
ffffffff80000801:	0f 20 c0             	mov    %cr0,%rax
ffffffff80000804:	48 83 c8 02          	or     $0x2,%rax
ffffffff80000808:	0f 22 c0             	mov    %rax,%cr0
ffffffff8000080b:	0f 20 e0             	mov    %cr4,%rax
ffffffff8000080e:	48 0d 00 06 04 00    	or     $0x40600,%rax
ffffffff80000814:	0f 22 e0             	mov    %rax,%cr4
ffffffff80000817:	31 c9                	xor    %ecx,%ecx
ffffffff80000819:	0f 01 d0             	xgetbv
ffffffff8000081c:	83 c8 03             	or     $0x3,%eax
ffffffff8000081f:	31 c9                	xor    %ecx,%ecx
ffffffff80000821:	0f 01 d1             	xsetbv
ffffffff80000824:	c3                   	ret
ffffffff80000825:	50                   	push   %rax
ffffffff80000826:	e8 31 18 00 00       	call   ffffffff8000205c <core::panicking::panic_fmt>

ffffffff8000082b <silicium_x86_64::smp::ap_start>:
ffffffff8000082b:	41 57                	push   %r15
ffffffff8000082d:	41 56                	push   %r14
ffffffff8000082f:	53                   	push   %rbx
ffffffff80000830:	48 83 ec 10          	sub    $0x10,%rsp
ffffffff80000834:	8b 7f 04             	mov    0x4(%rdi),%edi
ffffffff80000837:	e8 21 ff ff ff       	call   ffffffff8000075d <silicium_x86_64::percpu::setup>
ffffffff8000083c:	48 89 e0             	mov    %rsp,%rax
ffffffff8000083f:	66 c7 00 ff 0f       	movw   $0xfff,(%rax)
ffffffff80000844:	48 c7 40 02 38 40 00 	movq   $0xffffffff80004038,0x2(%rax)
ffffffff8000084b:	80 
ffffffff8000084c:	0f 01 18             	lidt   (%rax)
ffffffff8000084f:	65 48 8b 0c 25 00 00 	mov    %gs:0x0,%rcx
ffffffff80000856:	00 00 
ffffffff80000858:	49 c7 c7 10 09 00 80 	mov    $0xffffffff80000910,%r15
ffffffff8000085f:	48 c7 c3 30 09 00 80 	mov    $0xffffffff80000930,%rbx
ffffffff80000866:	4c 29 fb             	sub    %r15,%rbx
ffffffff80000869:	48 01 d9             	add    %rbx,%rcx
ffffffff8000086c:	66 c7 00 3f 00       	movw   $0x3f,(%rax)
ffffffff80000871:	48 89 48 02          	mov    %rcx,0x2(%rax)
ffffffff80000875:	49 89 e6             	mov    %rsp,%r14
ffffffff80000878:	41 0f 01 16          	lgdt   (%r14)
ffffffff8000087c:	e8 7f f7 ff ff       	call   ffffffff80000000 <__init_start>
ffffffff80000881:	65 48 8b 04 25 00 00 	mov    %gs:0x0,%rax
ffffffff80000888:	00 00 
ffffffff8000088a:	4c 29 f8             	sub    %r15,%rax
ffffffff8000088d:	48 8d 80 70 09 00 80 	lea    -0x7ffff690(%rax),%rax
ffffffff80000894:	49 89 06             	mov    %rax,(%r14)
ffffffff80000897:	44 89 f0             	mov    %r14d,%eax
ffffffff8000089a:	25 00 00 00 ff       	and    $0xff000000,%eax
ffffffff8000089f:	48 c1 e0 20          	shl    $0x20,%rax
ffffffff800008a3:	44 89 f1             	mov    %r14d,%ecx
ffffffff800008a6:	81 e1 f8 ff ff 00    	and    $0xfffff8,%ecx
ffffffff800008ac:	48 c1 e1 10          	shl    $0x10,%rcx
ffffffff800008b0:	48 09 c1             	or     %rax,%rcx
ffffffff800008b3:	48 b8 67 00 00 00 00 	movabs $0x890000000067,%rax
ffffffff800008ba:	89 00 00 
ffffffff800008bd:	48 09 c8             	or     %rcx,%rax
ffffffff800008c0:	65 48 8b 0c 25 00 00 	mov    %gs:0x0,%rcx
ffffffff800008c7:	00 00 
ffffffff800008c9:	48 89 44 19 30       	mov    %rax,0x30(%rcx,%rbx,1)
ffffffff800008ce:	49 c1 ee 20          	shr    $0x20,%r14
ffffffff800008d2:	65 48 8b 04 25 00 00 	mov    %gs:0x0,%rax
ffffffff800008d9:	00 00 
ffffffff800008db:	4c 89 74 18 38       	mov    %r14,0x38(%rax,%rbx,1)
ffffffff800008e0:	66 b8 30 00          	mov    $0x30,%ax
ffffffff800008e4:	0f 00 d8             	ltr    %ax
ffffffff800008e7:	e8 f6 fe ff ff       	call   ffffffff800007e2 <silicium_x86_64::simd::setup>
ffffffff800008ec:	48 b8 f0 00 e0 fe 00 	movabs $0xffff8000fee000f0,%rax
ffffffff800008f3:	80 ff ff 
ffffffff800008f6:	c7 00 ff 01 00 00    	movl   $0x1ff,(%rax)
ffffffff800008fc:	e8 df 17 00 00       	call   ffffffff800020e0 <silicium_x86_64::paging::pml4::Pml4::set_current>
ffffffff80000901:	f0 48 ff 05 67 47 00 	lock incq 0x4767(%rip)        # ffffffff80005070 <silicium_x86_64::smp::CPU_COUNT>
ffffffff80000908:	00 
ffffffff80000909:	fa                   	cli
ffffffff8000090a:	f4                   	hlt
ffffffff8000090b:	eb fc                	jmp    ffffffff80000909 <silicium_x86_64::smp::ap_start+0xde>
ffffffff8000090d:	cc                   	int3
ffffffff8000090e:	cc                   	int3
ffffffff8000090f:	cc                   	int3

ffffffff80000910 <__percpu_start>:
ffffffff80000910:	cc                   	int3
ffffffff80000911:	cc                   	int3
ffffffff80000912:	cc                   	int3
ffffffff80000913:	cc                   	int3
ffffffff80000914:	cc                   	int3
ffffffff80000915:	cc                   	int3
ffffffff80000916:	cc                   	int3
ffffffff80000917:	cc                   	int3
ffffffff80000918:	cc                   	int3
ffffffff80000919:	cc                   	int3
ffffffff8000091a:	cc                   	int3
ffffffff8000091b:	cc                   	int3
ffffffff8000091c:	cc                   	int3
ffffffff8000091d:	cc                   	int3
ffffffff8000091e:	cc                   	int3
ffffffff8000091f:	cc                   	int3
ffffffff80000920:	cc                   	int3
ffffffff80000921:	cc                   	int3
ffffffff80000922:	cc                   	int3
ffffffff80000923:	cc                   	int3
ffffffff80000924:	cc                   	int3
ffffffff80000925:	cc                   	int3
ffffffff80000926:	cc                   	int3
ffffffff80000927:	cc                   	int3
ffffffff80000928:	cc                   	int3
ffffffff80000929:	cc                   	int3
ffffffff8000092a:	cc                   	int3
ffffffff8000092b:	cc                   	int3
ffffffff8000092c:	cc                   	int3
ffffffff8000092d:	cc                   	int3
ffffffff8000092e:	cc                   	int3
ffffffff8000092f:	cc                   	int3

ffffffff80000930 <silicium_x86_64::gdt::TABLE>:
	...
ffffffff80000938:	ff ff 00 00 00 9b af 00 ff ff 00 00 00 93 af 00     ................
ffffffff80000948:	ff ff 00 00 00 fb 8f 00 ff ff 00 00 00 f3 af 00     ................
ffffffff80000958:	ff ff 00 00 00 fb af 00 00 00 00 00 00 00 00 00     ................
	...

ffffffff80000970 <silicium_x86_64::tss::TSS>:
	...
ffffffff800009d4:	00 00 68 00                                         ..h.

Déassemblage de la section .text :

ffffffff80001000 <interrupt_common>:
ffffffff80001000:	48 83 7c 24 18 08    	cmpq   $0x8,0x18(%rsp)
ffffffff80001006:	74 03                	je     ffffffff8000100b <interrupt_common+0xb>
ffffffff80001008:	0f 01 f8             	swapgs
ffffffff8000100b:	41 53                	push   %r11
ffffffff8000100d:	41 52                	push   %r10
ffffffff8000100f:	41 51                	push   %r9
ffffffff80001011:	41 50                	push   %r8
ffffffff80001013:	57                   	push   %rdi
ffffffff80001014:	56                   	push   %rsi
ffffffff80001015:	52                   	push   %rdx
ffffffff80001016:	51                   	push   %rcx
ffffffff80001017:	50                   	push   %rax
ffffffff80001018:	41 57                	push   %r15
ffffffff8000101a:	41 56                	push   %r14
ffffffff8000101c:	41 55                	push   %r13
ffffffff8000101e:	41 54                	push   %r12
ffffffff80001020:	53                   	push   %rbx
ffffffff80001021:	55                   	push   %rbp
ffffffff80001022:	48 89 e7             	mov    %rsp,%rdi
ffffffff80001025:	e8 5c 10 00 00       	call   ffffffff80002086 <irq_handler>
ffffffff8000102a:	5d                   	pop    %rbp
ffffffff8000102b:	5b                   	pop    %rbx
ffffffff8000102c:	41 5c                	pop    %r12
ffffffff8000102e:	41 5d                	pop    %r13
ffffffff80001030:	41 5e                	pop    %r14
ffffffff80001032:	41 5f                	pop    %r15
ffffffff80001034:	58                   	pop    %rax
ffffffff80001035:	59                   	pop    %rcx
ffffffff80001036:	5a                   	pop    %rdx
ffffffff80001037:	5e                   	pop    %rsi
ffffffff80001038:	5f                   	pop    %rdi
ffffffff80001039:	41 58                	pop    %r8
ffffffff8000103b:	41 59                	pop    %r9
ffffffff8000103d:	41 5a                	pop    %r10
ffffffff8000103f:	41 5b                	pop    %r11
ffffffff80001041:	48 83 7c 24 18 08    	cmpq   $0x8,0x18(%rsp)
ffffffff80001047:	74 03                	je     ffffffff8000104c <interrupt_common+0x4c>
ffffffff80001049:	0f 01 f8             	swapgs
ffffffff8000104c:	48 83 c4 10          	add    $0x10,%rsp
ffffffff80001050:	48 cf                	iretq
ffffffff80001052:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
ffffffff80001059:	1f 84 00 00 00 00 00 

ffffffff80001060 <interrupt_handlers>:
ffffffff80001060:	6a 00                	push   $0x0
ffffffff80001062:	6a 00                	push   $0x0
ffffffff80001064:	eb 9a                	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001066:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
ffffffff8000106d:	00 00 00 

ffffffff80001070 <irq_1>:
ffffffff80001070:	6a 00                	push   $0x0
ffffffff80001072:	6a 01                	push   $0x1
ffffffff80001074:	eb 8a                	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001076:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
ffffffff8000107d:	00 00 00 

ffffffff80001080 <irq_2>:
ffffffff80001080:	6a 00                	push   $0x0
ffffffff80001082:	6a 02                	push   $0x2
ffffffff80001084:	e9 77 ff ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001089:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001090 <irq_3>:
ffffffff80001090:	6a 00                	push   $0x0
ffffffff80001092:	6a 03                	push   $0x3
ffffffff80001094:	e9 67 ff ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001099:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800010a0 <irq_4>:
ffffffff800010a0:	6a 00                	push   $0x0
ffffffff800010a2:	6a 04                	push   $0x4
ffffffff800010a4:	e9 57 ff ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800010a9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800010b0 <irq_5>:
ffffffff800010b0:	6a 00                	push   $0x0
ffffffff800010b2:	6a 05                	push   $0x5
ffffffff800010b4:	e9 47 ff ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800010b9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800010c0 <irq_6>:
ffffffff800010c0:	6a 00                	push   $0x0
ffffffff800010c2:	6a 06                	push   $0x6
ffffffff800010c4:	e9 37 ff ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800010c9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800010d0 <irq_7>:
ffffffff800010d0:	6a 00                	push   $0x0
ffffffff800010d2:	6a 07                	push   $0x7
ffffffff800010d4:	e9 27 ff ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800010d9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800010e0 <irq_8>:
ffffffff800010e0:	6a 08                	push   $0x8
ffffffff800010e2:	e9 19 ff ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800010e7:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
ffffffff800010ee:	00 00 

ffffffff800010f0 <irq_9>:
ffffffff800010f0:	6a 00                	push   $0x0
ffffffff800010f2:	6a 09                	push   $0x9
ffffffff800010f4:	e9 07 ff ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800010f9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001100 <irq_10>:
ffffffff80001100:	6a 0a                	push   $0xa
ffffffff80001102:	e9 f9 fe ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001107:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
ffffffff8000110e:	00 00 

ffffffff80001110 <irq_11>:
ffffffff80001110:	6a 0b                	push   $0xb
ffffffff80001112:	e9 e9 fe ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001117:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
ffffffff8000111e:	00 00 

ffffffff80001120 <irq_12>:
ffffffff80001120:	6a 0c                	push   $0xc
ffffffff80001122:	e9 d9 fe ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001127:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
ffffffff8000112e:	00 00 

ffffffff80001130 <irq_13>:
ffffffff80001130:	6a 0d                	push   $0xd
ffffffff80001132:	e9 c9 fe ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001137:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
ffffffff8000113e:	00 00 

ffffffff80001140 <irq_14>:
ffffffff80001140:	6a 0e                	push   $0xe
ffffffff80001142:	e9 b9 fe ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001147:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
ffffffff8000114e:	00 00 

ffffffff80001150 <irq_15>:
ffffffff80001150:	6a 00                	push   $0x0
ffffffff80001152:	6a 0f                	push   $0xf
ffffffff80001154:	e9 a7 fe ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001159:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001160 <irq_16>:
ffffffff80001160:	6a 00                	push   $0x0
ffffffff80001162:	6a 10                	push   $0x10
ffffffff80001164:	e9 97 fe ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001169:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001170 <irq_17>:
ffffffff80001170:	6a 11                	push   $0x11
ffffffff80001172:	e9 89 fe ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001177:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
ffffffff8000117e:	00 00 

ffffffff80001180 <irq_18>:
ffffffff80001180:	6a 00                	push   $0x0
ffffffff80001182:	6a 12                	push   $0x12
ffffffff80001184:	e9 77 fe ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001189:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001190 <irq_19>:
ffffffff80001190:	6a 00                	push   $0x0
ffffffff80001192:	6a 13                	push   $0x13
ffffffff80001194:	e9 67 fe ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001199:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800011a0 <irq_20>:
ffffffff800011a0:	6a 00                	push   $0x0
ffffffff800011a2:	6a 14                	push   $0x14
ffffffff800011a4:	e9 57 fe ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800011a9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800011b0 <irq_21>:
ffffffff800011b0:	6a 15                	push   $0x15
ffffffff800011b2:	e9 49 fe ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800011b7:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
ffffffff800011be:	00 00 

ffffffff800011c0 <irq_22>:
ffffffff800011c0:	6a 00                	push   $0x0
ffffffff800011c2:	6a 16                	push   $0x16
ffffffff800011c4:	e9 37 fe ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800011c9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800011d0 <irq_23>:
ffffffff800011d0:	6a 00                	push   $0x0
ffffffff800011d2:	6a 17                	push   $0x17
ffffffff800011d4:	e9 27 fe ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800011d9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800011e0 <irq_24>:
ffffffff800011e0:	6a 00                	push   $0x0
ffffffff800011e2:	6a 18                	push   $0x18
ffffffff800011e4:	e9 17 fe ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800011e9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800011f0 <irq_25>:
ffffffff800011f0:	6a 00                	push   $0x0
ffffffff800011f2:	6a 19                	push   $0x19
ffffffff800011f4:	e9 07 fe ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800011f9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001200 <irq_26>:
ffffffff80001200:	6a 00                	push   $0x0
ffffffff80001202:	6a 1a                	push   $0x1a
ffffffff80001204:	e9 f7 fd ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001209:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001210 <irq_27>:
ffffffff80001210:	6a 00                	push   $0x0
ffffffff80001212:	6a 1b                	push   $0x1b
ffffffff80001214:	e9 e7 fd ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001219:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001220 <irq_28>:
ffffffff80001220:	6a 00                	push   $0x0
ffffffff80001222:	6a 1c                	push   $0x1c
ffffffff80001224:	e9 d7 fd ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001229:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001230 <irq_29>:
ffffffff80001230:	6a 1d                	push   $0x1d
ffffffff80001232:	e9 c9 fd ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001237:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
ffffffff8000123e:	00 00 

ffffffff80001240 <irq_30>:
ffffffff80001240:	6a 1e                	push   $0x1e
ffffffff80001242:	e9 b9 fd ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001247:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
ffffffff8000124e:	00 00 

ffffffff80001250 <irq_31>:
ffffffff80001250:	6a 00                	push   $0x0
ffffffff80001252:	6a 1f                	push   $0x1f
ffffffff80001254:	e9 a7 fd ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001259:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001260 <irq_32>:
ffffffff80001260:	6a 00                	push   $0x0
ffffffff80001262:	6a 20                	push   $0x20
ffffffff80001264:	e9 97 fd ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001269:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001270 <irq_33>:
ffffffff80001270:	6a 00                	push   $0x0
ffffffff80001272:	6a 21                	push   $0x21
ffffffff80001274:	e9 87 fd ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001279:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001280 <irq_34>:
ffffffff80001280:	6a 00                	push   $0x0
ffffffff80001282:	6a 22                	push   $0x22
ffffffff80001284:	e9 77 fd ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001289:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001290 <irq_35>:
ffffffff80001290:	6a 00                	push   $0x0
ffffffff80001292:	6a 23                	push   $0x23
ffffffff80001294:	e9 67 fd ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001299:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800012a0 <irq_36>:
ffffffff800012a0:	6a 00                	push   $0x0
ffffffff800012a2:	6a 24                	push   $0x24
ffffffff800012a4:	e9 57 fd ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800012a9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800012b0 <irq_37>:
ffffffff800012b0:	6a 00                	push   $0x0
ffffffff800012b2:	6a 25                	push   $0x25
ffffffff800012b4:	e9 47 fd ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800012b9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800012c0 <irq_38>:
ffffffff800012c0:	6a 00                	push   $0x0
ffffffff800012c2:	6a 26                	push   $0x26
ffffffff800012c4:	e9 37 fd ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800012c9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800012d0 <irq_39>:
ffffffff800012d0:	6a 00                	push   $0x0
ffffffff800012d2:	6a 27                	push   $0x27
ffffffff800012d4:	e9 27 fd ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800012d9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800012e0 <irq_40>:
ffffffff800012e0:	6a 00                	push   $0x0
ffffffff800012e2:	6a 28                	push   $0x28
ffffffff800012e4:	e9 17 fd ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800012e9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800012f0 <irq_41>:
ffffffff800012f0:	6a 00                	push   $0x0
ffffffff800012f2:	6a 29                	push   $0x29
ffffffff800012f4:	e9 07 fd ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800012f9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001300 <irq_42>:
ffffffff80001300:	6a 00                	push   $0x0
ffffffff80001302:	6a 2a                	push   $0x2a
ffffffff80001304:	e9 f7 fc ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001309:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001310 <irq_43>:
ffffffff80001310:	6a 00                	push   $0x0
ffffffff80001312:	6a 2b                	push   $0x2b
ffffffff80001314:	e9 e7 fc ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001319:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001320 <irq_44>:
ffffffff80001320:	6a 00                	push   $0x0
ffffffff80001322:	6a 2c                	push   $0x2c
ffffffff80001324:	e9 d7 fc ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001329:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001330 <irq_45>:
ffffffff80001330:	6a 00                	push   $0x0
ffffffff80001332:	6a 2d                	push   $0x2d
ffffffff80001334:	e9 c7 fc ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001339:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001340 <irq_46>:
ffffffff80001340:	6a 00                	push   $0x0
ffffffff80001342:	6a 2e                	push   $0x2e
ffffffff80001344:	e9 b7 fc ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001349:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001350 <irq_47>:
ffffffff80001350:	6a 00                	push   $0x0
ffffffff80001352:	6a 2f                	push   $0x2f
ffffffff80001354:	e9 a7 fc ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001359:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001360 <irq_48>:
ffffffff80001360:	6a 00                	push   $0x0
ffffffff80001362:	6a 30                	push   $0x30
ffffffff80001364:	e9 97 fc ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001369:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001370 <irq_49>:
ffffffff80001370:	6a 00                	push   $0x0
ffffffff80001372:	6a 31                	push   $0x31
ffffffff80001374:	e9 87 fc ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001379:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001380 <irq_50>:
ffffffff80001380:	6a 00                	push   $0x0
ffffffff80001382:	6a 32                	push   $0x32
ffffffff80001384:	e9 77 fc ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001389:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001390 <irq_51>:
ffffffff80001390:	6a 00                	push   $0x0
ffffffff80001392:	6a 33                	push   $0x33
ffffffff80001394:	e9 67 fc ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001399:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800013a0 <irq_52>:
ffffffff800013a0:	6a 00                	push   $0x0
ffffffff800013a2:	6a 34                	push   $0x34
ffffffff800013a4:	e9 57 fc ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800013a9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800013b0 <irq_53>:
ffffffff800013b0:	6a 00                	push   $0x0
ffffffff800013b2:	6a 35                	push   $0x35
ffffffff800013b4:	e9 47 fc ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800013b9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800013c0 <irq_54>:
ffffffff800013c0:	6a 00                	push   $0x0
ffffffff800013c2:	6a 36                	push   $0x36
ffffffff800013c4:	e9 37 fc ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800013c9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800013d0 <irq_55>:
ffffffff800013d0:	6a 00                	push   $0x0
ffffffff800013d2:	6a 37                	push   $0x37
ffffffff800013d4:	e9 27 fc ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800013d9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800013e0 <irq_56>:
ffffffff800013e0:	6a 00                	push   $0x0
ffffffff800013e2:	6a 38                	push   $0x38
ffffffff800013e4:	e9 17 fc ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800013e9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800013f0 <irq_57>:
ffffffff800013f0:	6a 00                	push   $0x0
ffffffff800013f2:	6a 39                	push   $0x39
ffffffff800013f4:	e9 07 fc ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800013f9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001400 <irq_58>:
ffffffff80001400:	6a 00                	push   $0x0
ffffffff80001402:	6a 3a                	push   $0x3a
ffffffff80001404:	e9 f7 fb ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001409:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001410 <irq_59>:
ffffffff80001410:	6a 00                	push   $0x0
ffffffff80001412:	6a 3b                	push   $0x3b
ffffffff80001414:	e9 e7 fb ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001419:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001420 <irq_60>:
ffffffff80001420:	6a 00                	push   $0x0
ffffffff80001422:	6a 3c                	push   $0x3c
ffffffff80001424:	e9 d7 fb ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001429:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001430 <irq_61>:
ffffffff80001430:	6a 00                	push   $0x0
ffffffff80001432:	6a 3d                	push   $0x3d
ffffffff80001434:	e9 c7 fb ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001439:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001440 <irq_62>:
ffffffff80001440:	6a 00                	push   $0x0
ffffffff80001442:	6a 3e                	push   $0x3e
ffffffff80001444:	e9 b7 fb ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001449:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001450 <irq_63>:
ffffffff80001450:	6a 00                	push   $0x0
ffffffff80001452:	6a 3f                	push   $0x3f
ffffffff80001454:	e9 a7 fb ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001459:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001460 <irq_64>:
ffffffff80001460:	6a 00                	push   $0x0
ffffffff80001462:	6a 40                	push   $0x40
ffffffff80001464:	e9 97 fb ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001469:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001470 <irq_65>:
ffffffff80001470:	6a 00                	push   $0x0
ffffffff80001472:	6a 41                	push   $0x41
ffffffff80001474:	e9 87 fb ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001479:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001480 <irq_66>:
ffffffff80001480:	6a 00                	push   $0x0
ffffffff80001482:	6a 42                	push   $0x42
ffffffff80001484:	e9 77 fb ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001489:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001490 <irq_67>:
ffffffff80001490:	6a 00                	push   $0x0
ffffffff80001492:	6a 43                	push   $0x43
ffffffff80001494:	e9 67 fb ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001499:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800014a0 <irq_68>:
ffffffff800014a0:	6a 00                	push   $0x0
ffffffff800014a2:	6a 44                	push   $0x44
ffffffff800014a4:	e9 57 fb ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800014a9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800014b0 <irq_69>:
ffffffff800014b0:	6a 00                	push   $0x0
ffffffff800014b2:	6a 45                	push   $0x45
ffffffff800014b4:	e9 47 fb ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800014b9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800014c0 <irq_70>:
ffffffff800014c0:	6a 00                	push   $0x0
ffffffff800014c2:	6a 46                	push   $0x46
ffffffff800014c4:	e9 37 fb ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800014c9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800014d0 <irq_71>:
ffffffff800014d0:	6a 00                	push   $0x0
ffffffff800014d2:	6a 47                	push   $0x47
ffffffff800014d4:	e9 27 fb ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800014d9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800014e0 <irq_72>:
ffffffff800014e0:	6a 00                	push   $0x0
ffffffff800014e2:	6a 48                	push   $0x48
ffffffff800014e4:	e9 17 fb ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800014e9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800014f0 <irq_73>:
ffffffff800014f0:	6a 00                	push   $0x0
ffffffff800014f2:	6a 49                	push   $0x49
ffffffff800014f4:	e9 07 fb ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800014f9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001500 <irq_74>:
ffffffff80001500:	6a 00                	push   $0x0
ffffffff80001502:	6a 4a                	push   $0x4a
ffffffff80001504:	e9 f7 fa ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001509:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001510 <irq_75>:
ffffffff80001510:	6a 00                	push   $0x0
ffffffff80001512:	6a 4b                	push   $0x4b
ffffffff80001514:	e9 e7 fa ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001519:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001520 <irq_76>:
ffffffff80001520:	6a 00                	push   $0x0
ffffffff80001522:	6a 4c                	push   $0x4c
ffffffff80001524:	e9 d7 fa ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001529:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001530 <irq_77>:
ffffffff80001530:	6a 00                	push   $0x0
ffffffff80001532:	6a 4d                	push   $0x4d
ffffffff80001534:	e9 c7 fa ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001539:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001540 <irq_78>:
ffffffff80001540:	6a 00                	push   $0x0
ffffffff80001542:	6a 4e                	push   $0x4e
ffffffff80001544:	e9 b7 fa ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001549:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001550 <irq_79>:
ffffffff80001550:	6a 00                	push   $0x0
ffffffff80001552:	6a 4f                	push   $0x4f
ffffffff80001554:	e9 a7 fa ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001559:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001560 <irq_80>:
ffffffff80001560:	6a 00                	push   $0x0
ffffffff80001562:	6a 50                	push   $0x50
ffffffff80001564:	e9 97 fa ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001569:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001570 <irq_81>:
ffffffff80001570:	6a 00                	push   $0x0
ffffffff80001572:	6a 51                	push   $0x51
ffffffff80001574:	e9 87 fa ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001579:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001580 <irq_82>:
ffffffff80001580:	6a 00                	push   $0x0
ffffffff80001582:	6a 52                	push   $0x52
ffffffff80001584:	e9 77 fa ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001589:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001590 <irq_83>:
ffffffff80001590:	6a 00                	push   $0x0
ffffffff80001592:	6a 53                	push   $0x53
ffffffff80001594:	e9 67 fa ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001599:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800015a0 <irq_84>:
ffffffff800015a0:	6a 00                	push   $0x0
ffffffff800015a2:	6a 54                	push   $0x54
ffffffff800015a4:	e9 57 fa ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800015a9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800015b0 <irq_85>:
ffffffff800015b0:	6a 00                	push   $0x0
ffffffff800015b2:	6a 55                	push   $0x55
ffffffff800015b4:	e9 47 fa ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800015b9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800015c0 <irq_86>:
ffffffff800015c0:	6a 00                	push   $0x0
ffffffff800015c2:	6a 56                	push   $0x56
ffffffff800015c4:	e9 37 fa ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800015c9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800015d0 <irq_87>:
ffffffff800015d0:	6a 00                	push   $0x0
ffffffff800015d2:	6a 57                	push   $0x57
ffffffff800015d4:	e9 27 fa ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800015d9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800015e0 <irq_88>:
ffffffff800015e0:	6a 00                	push   $0x0
ffffffff800015e2:	6a 58                	push   $0x58
ffffffff800015e4:	e9 17 fa ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800015e9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800015f0 <irq_89>:
ffffffff800015f0:	6a 00                	push   $0x0
ffffffff800015f2:	6a 59                	push   $0x59
ffffffff800015f4:	e9 07 fa ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800015f9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001600 <irq_90>:
ffffffff80001600:	6a 00                	push   $0x0
ffffffff80001602:	6a 5a                	push   $0x5a
ffffffff80001604:	e9 f7 f9 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001609:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001610 <irq_91>:
ffffffff80001610:	6a 00                	push   $0x0
ffffffff80001612:	6a 5b                	push   $0x5b
ffffffff80001614:	e9 e7 f9 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001619:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001620 <irq_92>:
ffffffff80001620:	6a 00                	push   $0x0
ffffffff80001622:	6a 5c                	push   $0x5c
ffffffff80001624:	e9 d7 f9 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001629:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001630 <irq_93>:
ffffffff80001630:	6a 00                	push   $0x0
ffffffff80001632:	6a 5d                	push   $0x5d
ffffffff80001634:	e9 c7 f9 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001639:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001640 <irq_94>:
ffffffff80001640:	6a 00                	push   $0x0
ffffffff80001642:	6a 5e                	push   $0x5e
ffffffff80001644:	e9 b7 f9 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001649:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001650 <irq_95>:
ffffffff80001650:	6a 00                	push   $0x0
ffffffff80001652:	6a 5f                	push   $0x5f
ffffffff80001654:	e9 a7 f9 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001659:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001660 <irq_96>:
ffffffff80001660:	6a 00                	push   $0x0
ffffffff80001662:	6a 60                	push   $0x60
ffffffff80001664:	e9 97 f9 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001669:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001670 <irq_97>:
ffffffff80001670:	6a 00                	push   $0x0
ffffffff80001672:	6a 61                	push   $0x61
ffffffff80001674:	e9 87 f9 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001679:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001680 <irq_98>:
ffffffff80001680:	6a 00                	push   $0x0
ffffffff80001682:	6a 62                	push   $0x62
ffffffff80001684:	e9 77 f9 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001689:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001690 <irq_99>:
ffffffff80001690:	6a 00                	push   $0x0
ffffffff80001692:	6a 63                	push   $0x63
ffffffff80001694:	e9 67 f9 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001699:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800016a0 <irq_100>:
ffffffff800016a0:	6a 00                	push   $0x0
ffffffff800016a2:	6a 64                	push   $0x64
ffffffff800016a4:	e9 57 f9 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800016a9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800016b0 <irq_101>:
ffffffff800016b0:	6a 00                	push   $0x0
ffffffff800016b2:	6a 65                	push   $0x65
ffffffff800016b4:	e9 47 f9 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800016b9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800016c0 <irq_102>:
ffffffff800016c0:	6a 00                	push   $0x0
ffffffff800016c2:	6a 66                	push   $0x66
ffffffff800016c4:	e9 37 f9 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800016c9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800016d0 <irq_103>:
ffffffff800016d0:	6a 00                	push   $0x0
ffffffff800016d2:	6a 67                	push   $0x67
ffffffff800016d4:	e9 27 f9 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800016d9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800016e0 <irq_104>:
ffffffff800016e0:	6a 00                	push   $0x0
ffffffff800016e2:	6a 68                	push   $0x68
ffffffff800016e4:	e9 17 f9 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800016e9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800016f0 <irq_105>:
ffffffff800016f0:	6a 00                	push   $0x0
ffffffff800016f2:	6a 69                	push   $0x69
ffffffff800016f4:	e9 07 f9 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800016f9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001700 <irq_106>:
ffffffff80001700:	6a 00                	push   $0x0
ffffffff80001702:	6a 6a                	push   $0x6a
ffffffff80001704:	e9 f7 f8 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001709:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001710 <irq_107>:
ffffffff80001710:	6a 00                	push   $0x0
ffffffff80001712:	6a 6b                	push   $0x6b
ffffffff80001714:	e9 e7 f8 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001719:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001720 <irq_108>:
ffffffff80001720:	6a 00                	push   $0x0
ffffffff80001722:	6a 6c                	push   $0x6c
ffffffff80001724:	e9 d7 f8 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001729:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001730 <irq_109>:
ffffffff80001730:	6a 00                	push   $0x0
ffffffff80001732:	6a 6d                	push   $0x6d
ffffffff80001734:	e9 c7 f8 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001739:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001740 <irq_110>:
ffffffff80001740:	6a 00                	push   $0x0
ffffffff80001742:	6a 6e                	push   $0x6e
ffffffff80001744:	e9 b7 f8 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001749:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001750 <irq_111>:
ffffffff80001750:	6a 00                	push   $0x0
ffffffff80001752:	6a 6f                	push   $0x6f
ffffffff80001754:	e9 a7 f8 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001759:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001760 <irq_112>:
ffffffff80001760:	6a 00                	push   $0x0
ffffffff80001762:	6a 70                	push   $0x70
ffffffff80001764:	e9 97 f8 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001769:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001770 <irq_113>:
ffffffff80001770:	6a 00                	push   $0x0
ffffffff80001772:	6a 71                	push   $0x71
ffffffff80001774:	e9 87 f8 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001779:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001780 <irq_114>:
ffffffff80001780:	6a 00                	push   $0x0
ffffffff80001782:	6a 72                	push   $0x72
ffffffff80001784:	e9 77 f8 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001789:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001790 <irq_115>:
ffffffff80001790:	6a 00                	push   $0x0
ffffffff80001792:	6a 73                	push   $0x73
ffffffff80001794:	e9 67 f8 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001799:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800017a0 <irq_116>:
ffffffff800017a0:	6a 00                	push   $0x0
ffffffff800017a2:	6a 74                	push   $0x74
ffffffff800017a4:	e9 57 f8 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800017a9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800017b0 <irq_117>:
ffffffff800017b0:	6a 00                	push   $0x0
ffffffff800017b2:	6a 75                	push   $0x75
ffffffff800017b4:	e9 47 f8 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800017b9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800017c0 <irq_118>:
ffffffff800017c0:	6a 00                	push   $0x0
ffffffff800017c2:	6a 76                	push   $0x76
ffffffff800017c4:	e9 37 f8 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800017c9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800017d0 <irq_119>:
ffffffff800017d0:	6a 00                	push   $0x0
ffffffff800017d2:	6a 77                	push   $0x77
ffffffff800017d4:	e9 27 f8 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800017d9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800017e0 <irq_120>:
ffffffff800017e0:	6a 00                	push   $0x0
ffffffff800017e2:	6a 78                	push   $0x78
ffffffff800017e4:	e9 17 f8 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800017e9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff800017f0 <irq_121>:
ffffffff800017f0:	6a 00                	push   $0x0
ffffffff800017f2:	6a 79                	push   $0x79
ffffffff800017f4:	e9 07 f8 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800017f9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001800 <irq_122>:
ffffffff80001800:	6a 00                	push   $0x0
ffffffff80001802:	6a 7a                	push   $0x7a
ffffffff80001804:	e9 f7 f7 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001809:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001810 <irq_123>:
ffffffff80001810:	6a 00                	push   $0x0
ffffffff80001812:	6a 7b                	push   $0x7b
ffffffff80001814:	e9 e7 f7 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001819:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001820 <irq_124>:
ffffffff80001820:	6a 00                	push   $0x0
ffffffff80001822:	6a 7c                	push   $0x7c
ffffffff80001824:	e9 d7 f7 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001829:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001830 <irq_125>:
ffffffff80001830:	6a 00                	push   $0x0
ffffffff80001832:	6a 7d                	push   $0x7d
ffffffff80001834:	e9 c7 f7 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001839:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001840 <irq_126>:
ffffffff80001840:	6a 00                	push   $0x0
ffffffff80001842:	6a 7e                	push   $0x7e
ffffffff80001844:	e9 b7 f7 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001849:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001850 <irq_127>:
ffffffff80001850:	6a 00                	push   $0x0
ffffffff80001852:	6a 7f                	push   $0x7f
ffffffff80001854:	e9 a7 f7 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001859:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)

ffffffff80001860 <irq_128>:
ffffffff80001860:	6a 00                	push   $0x0
ffffffff80001862:	68 80 00 00 00       	push   $0x80
ffffffff80001867:	e9 94 f7 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000186c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001870 <irq_129>:
ffffffff80001870:	6a 00                	push   $0x0
ffffffff80001872:	68 81 00 00 00       	push   $0x81
ffffffff80001877:	e9 84 f7 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000187c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001880 <irq_130>:
ffffffff80001880:	6a 00                	push   $0x0
ffffffff80001882:	68 82 00 00 00       	push   $0x82
ffffffff80001887:	e9 74 f7 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000188c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001890 <irq_131>:
ffffffff80001890:	6a 00                	push   $0x0
ffffffff80001892:	68 83 00 00 00       	push   $0x83
ffffffff80001897:	e9 64 f7 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000189c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff800018a0 <irq_132>:
ffffffff800018a0:	6a 00                	push   $0x0
ffffffff800018a2:	68 84 00 00 00       	push   $0x84
ffffffff800018a7:	e9 54 f7 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800018ac:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff800018b0 <irq_133>:
ffffffff800018b0:	6a 00                	push   $0x0
ffffffff800018b2:	68 85 00 00 00       	push   $0x85
ffffffff800018b7:	e9 44 f7 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800018bc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff800018c0 <irq_134>:
ffffffff800018c0:	6a 00                	push   $0x0
ffffffff800018c2:	68 86 00 00 00       	push   $0x86
ffffffff800018c7:	e9 34 f7 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800018cc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff800018d0 <irq_135>:
ffffffff800018d0:	6a 00                	push   $0x0
ffffffff800018d2:	68 87 00 00 00       	push   $0x87
ffffffff800018d7:	e9 24 f7 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800018dc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff800018e0 <irq_136>:
ffffffff800018e0:	6a 00                	push   $0x0
ffffffff800018e2:	68 88 00 00 00       	push   $0x88
ffffffff800018e7:	e9 14 f7 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800018ec:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff800018f0 <irq_137>:
ffffffff800018f0:	6a 00                	push   $0x0
ffffffff800018f2:	68 89 00 00 00       	push   $0x89
ffffffff800018f7:	e9 04 f7 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800018fc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001900 <irq_138>:
ffffffff80001900:	6a 00                	push   $0x0
ffffffff80001902:	68 8a 00 00 00       	push   $0x8a
ffffffff80001907:	e9 f4 f6 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000190c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001910 <irq_139>:
ffffffff80001910:	6a 00                	push   $0x0
ffffffff80001912:	68 8b 00 00 00       	push   $0x8b
ffffffff80001917:	e9 e4 f6 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000191c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001920 <irq_140>:
ffffffff80001920:	6a 00                	push   $0x0
ffffffff80001922:	68 8c 00 00 00       	push   $0x8c
ffffffff80001927:	e9 d4 f6 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000192c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001930 <irq_141>:
ffffffff80001930:	6a 00                	push   $0x0
ffffffff80001932:	68 8d 00 00 00       	push   $0x8d
ffffffff80001937:	e9 c4 f6 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000193c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001940 <irq_142>:
ffffffff80001940:	6a 00                	push   $0x0
ffffffff80001942:	68 8e 00 00 00       	push   $0x8e
ffffffff80001947:	e9 b4 f6 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000194c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001950 <irq_143>:
ffffffff80001950:	6a 00                	push   $0x0
ffffffff80001952:	68 8f 00 00 00       	push   $0x8f
ffffffff80001957:	e9 a4 f6 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000195c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001960 <irq_144>:
ffffffff80001960:	6a 00                	push   $0x0
ffffffff80001962:	68 90 00 00 00       	push   $0x90
ffffffff80001967:	e9 94 f6 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000196c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001970 <irq_145>:
ffffffff80001970:	6a 00                	push   $0x0
ffffffff80001972:	68 91 00 00 00       	push   $0x91
ffffffff80001977:	e9 84 f6 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000197c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001980 <irq_146>:
ffffffff80001980:	6a 00                	push   $0x0
ffffffff80001982:	68 92 00 00 00       	push   $0x92
ffffffff80001987:	e9 74 f6 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000198c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001990 <irq_147>:
ffffffff80001990:	6a 00                	push   $0x0
ffffffff80001992:	68 93 00 00 00       	push   $0x93
ffffffff80001997:	e9 64 f6 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000199c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff800019a0 <irq_148>:
ffffffff800019a0:	6a 00                	push   $0x0
ffffffff800019a2:	68 94 00 00 00       	push   $0x94
ffffffff800019a7:	e9 54 f6 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800019ac:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff800019b0 <irq_149>:
ffffffff800019b0:	6a 00                	push   $0x0
ffffffff800019b2:	68 95 00 00 00       	push   $0x95
ffffffff800019b7:	e9 44 f6 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800019bc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff800019c0 <irq_150>:
ffffffff800019c0:	6a 00                	push   $0x0
ffffffff800019c2:	68 96 00 00 00       	push   $0x96
ffffffff800019c7:	e9 34 f6 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800019cc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff800019d0 <irq_151>:
ffffffff800019d0:	6a 00                	push   $0x0
ffffffff800019d2:	68 97 00 00 00       	push   $0x97
ffffffff800019d7:	e9 24 f6 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800019dc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff800019e0 <irq_152>:
ffffffff800019e0:	6a 00                	push   $0x0
ffffffff800019e2:	68 98 00 00 00       	push   $0x98
ffffffff800019e7:	e9 14 f6 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800019ec:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff800019f0 <irq_153>:
ffffffff800019f0:	6a 00                	push   $0x0
ffffffff800019f2:	68 99 00 00 00       	push   $0x99
ffffffff800019f7:	e9 04 f6 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff800019fc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001a00 <irq_154>:
ffffffff80001a00:	6a 00                	push   $0x0
ffffffff80001a02:	68 9a 00 00 00       	push   $0x9a
ffffffff80001a07:	e9 f4 f5 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001a0c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001a10 <irq_155>:
ffffffff80001a10:	6a 00                	push   $0x0
ffffffff80001a12:	68 9b 00 00 00       	push   $0x9b
ffffffff80001a17:	e9 e4 f5 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001a1c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001a20 <irq_156>:
ffffffff80001a20:	6a 00                	push   $0x0
ffffffff80001a22:	68 9c 00 00 00       	push   $0x9c
ffffffff80001a27:	e9 d4 f5 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001a2c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001a30 <irq_157>:
ffffffff80001a30:	6a 00                	push   $0x0
ffffffff80001a32:	68 9d 00 00 00       	push   $0x9d
ffffffff80001a37:	e9 c4 f5 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001a3c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001a40 <irq_158>:
ffffffff80001a40:	6a 00                	push   $0x0
ffffffff80001a42:	68 9e 00 00 00       	push   $0x9e
ffffffff80001a47:	e9 b4 f5 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001a4c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001a50 <irq_159>:
ffffffff80001a50:	6a 00                	push   $0x0
ffffffff80001a52:	68 9f 00 00 00       	push   $0x9f
ffffffff80001a57:	e9 a4 f5 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001a5c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001a60 <irq_160>:
ffffffff80001a60:	6a 00                	push   $0x0
ffffffff80001a62:	68 a0 00 00 00       	push   $0xa0
ffffffff80001a67:	e9 94 f5 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001a6c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001a70 <irq_161>:
ffffffff80001a70:	6a 00                	push   $0x0
ffffffff80001a72:	68 a1 00 00 00       	push   $0xa1
ffffffff80001a77:	e9 84 f5 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001a7c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001a80 <irq_162>:
ffffffff80001a80:	6a 00                	push   $0x0
ffffffff80001a82:	68 a2 00 00 00       	push   $0xa2
ffffffff80001a87:	e9 74 f5 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001a8c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001a90 <irq_163>:
ffffffff80001a90:	6a 00                	push   $0x0
ffffffff80001a92:	68 a3 00 00 00       	push   $0xa3
ffffffff80001a97:	e9 64 f5 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001a9c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001aa0 <irq_164>:
ffffffff80001aa0:	6a 00                	push   $0x0
ffffffff80001aa2:	68 a4 00 00 00       	push   $0xa4
ffffffff80001aa7:	e9 54 f5 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001aac:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001ab0 <irq_165>:
ffffffff80001ab0:	6a 00                	push   $0x0
ffffffff80001ab2:	68 a5 00 00 00       	push   $0xa5
ffffffff80001ab7:	e9 44 f5 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001abc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001ac0 <irq_166>:
ffffffff80001ac0:	6a 00                	push   $0x0
ffffffff80001ac2:	68 a6 00 00 00       	push   $0xa6
ffffffff80001ac7:	e9 34 f5 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001acc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001ad0 <irq_167>:
ffffffff80001ad0:	6a 00                	push   $0x0
ffffffff80001ad2:	68 a7 00 00 00       	push   $0xa7
ffffffff80001ad7:	e9 24 f5 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001adc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001ae0 <irq_168>:
ffffffff80001ae0:	6a 00                	push   $0x0
ffffffff80001ae2:	68 a8 00 00 00       	push   $0xa8
ffffffff80001ae7:	e9 14 f5 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001aec:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001af0 <irq_169>:
ffffffff80001af0:	6a 00                	push   $0x0
ffffffff80001af2:	68 a9 00 00 00       	push   $0xa9
ffffffff80001af7:	e9 04 f5 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001afc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001b00 <irq_170>:
ffffffff80001b00:	6a 00                	push   $0x0
ffffffff80001b02:	68 aa 00 00 00       	push   $0xaa
ffffffff80001b07:	e9 f4 f4 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001b0c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001b10 <irq_171>:
ffffffff80001b10:	6a 00                	push   $0x0
ffffffff80001b12:	68 ab 00 00 00       	push   $0xab
ffffffff80001b17:	e9 e4 f4 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001b1c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001b20 <irq_172>:
ffffffff80001b20:	6a 00                	push   $0x0
ffffffff80001b22:	68 ac 00 00 00       	push   $0xac
ffffffff80001b27:	e9 d4 f4 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001b2c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001b30 <irq_173>:
ffffffff80001b30:	6a 00                	push   $0x0
ffffffff80001b32:	68 ad 00 00 00       	push   $0xad
ffffffff80001b37:	e9 c4 f4 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001b3c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001b40 <irq_174>:
ffffffff80001b40:	6a 00                	push   $0x0
ffffffff80001b42:	68 ae 00 00 00       	push   $0xae
ffffffff80001b47:	e9 b4 f4 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001b4c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001b50 <irq_175>:
ffffffff80001b50:	6a 00                	push   $0x0
ffffffff80001b52:	68 af 00 00 00       	push   $0xaf
ffffffff80001b57:	e9 a4 f4 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001b5c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001b60 <irq_176>:
ffffffff80001b60:	6a 00                	push   $0x0
ffffffff80001b62:	68 b0 00 00 00       	push   $0xb0
ffffffff80001b67:	e9 94 f4 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001b6c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001b70 <irq_177>:
ffffffff80001b70:	6a 00                	push   $0x0
ffffffff80001b72:	68 b1 00 00 00       	push   $0xb1
ffffffff80001b77:	e9 84 f4 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001b7c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001b80 <irq_178>:
ffffffff80001b80:	6a 00                	push   $0x0
ffffffff80001b82:	68 b2 00 00 00       	push   $0xb2
ffffffff80001b87:	e9 74 f4 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001b8c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001b90 <irq_179>:
ffffffff80001b90:	6a 00                	push   $0x0
ffffffff80001b92:	68 b3 00 00 00       	push   $0xb3
ffffffff80001b97:	e9 64 f4 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001b9c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001ba0 <irq_180>:
ffffffff80001ba0:	6a 00                	push   $0x0
ffffffff80001ba2:	68 b4 00 00 00       	push   $0xb4
ffffffff80001ba7:	e9 54 f4 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001bac:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001bb0 <irq_181>:
ffffffff80001bb0:	6a 00                	push   $0x0
ffffffff80001bb2:	68 b5 00 00 00       	push   $0xb5
ffffffff80001bb7:	e9 44 f4 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001bbc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001bc0 <irq_182>:
ffffffff80001bc0:	6a 00                	push   $0x0
ffffffff80001bc2:	68 b6 00 00 00       	push   $0xb6
ffffffff80001bc7:	e9 34 f4 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001bcc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001bd0 <irq_183>:
ffffffff80001bd0:	6a 00                	push   $0x0
ffffffff80001bd2:	68 b7 00 00 00       	push   $0xb7
ffffffff80001bd7:	e9 24 f4 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001bdc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001be0 <irq_184>:
ffffffff80001be0:	6a 00                	push   $0x0
ffffffff80001be2:	68 b8 00 00 00       	push   $0xb8
ffffffff80001be7:	e9 14 f4 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001bec:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001bf0 <irq_185>:
ffffffff80001bf0:	6a 00                	push   $0x0
ffffffff80001bf2:	68 b9 00 00 00       	push   $0xb9
ffffffff80001bf7:	e9 04 f4 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001bfc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001c00 <irq_186>:
ffffffff80001c00:	6a 00                	push   $0x0
ffffffff80001c02:	68 ba 00 00 00       	push   $0xba
ffffffff80001c07:	e9 f4 f3 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001c0c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001c10 <irq_187>:
ffffffff80001c10:	6a 00                	push   $0x0
ffffffff80001c12:	68 bb 00 00 00       	push   $0xbb
ffffffff80001c17:	e9 e4 f3 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001c1c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001c20 <irq_188>:
ffffffff80001c20:	6a 00                	push   $0x0
ffffffff80001c22:	68 bc 00 00 00       	push   $0xbc
ffffffff80001c27:	e9 d4 f3 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001c2c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001c30 <irq_189>:
ffffffff80001c30:	6a 00                	push   $0x0
ffffffff80001c32:	68 bd 00 00 00       	push   $0xbd
ffffffff80001c37:	e9 c4 f3 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001c3c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001c40 <irq_190>:
ffffffff80001c40:	6a 00                	push   $0x0
ffffffff80001c42:	68 be 00 00 00       	push   $0xbe
ffffffff80001c47:	e9 b4 f3 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001c4c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001c50 <irq_191>:
ffffffff80001c50:	6a 00                	push   $0x0
ffffffff80001c52:	68 bf 00 00 00       	push   $0xbf
ffffffff80001c57:	e9 a4 f3 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001c5c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001c60 <irq_192>:
ffffffff80001c60:	6a 00                	push   $0x0
ffffffff80001c62:	68 c0 00 00 00       	push   $0xc0
ffffffff80001c67:	e9 94 f3 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001c6c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001c70 <irq_193>:
ffffffff80001c70:	6a 00                	push   $0x0
ffffffff80001c72:	68 c1 00 00 00       	push   $0xc1
ffffffff80001c77:	e9 84 f3 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001c7c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001c80 <irq_194>:
ffffffff80001c80:	6a 00                	push   $0x0
ffffffff80001c82:	68 c2 00 00 00       	push   $0xc2
ffffffff80001c87:	e9 74 f3 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001c8c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001c90 <irq_195>:
ffffffff80001c90:	6a 00                	push   $0x0
ffffffff80001c92:	68 c3 00 00 00       	push   $0xc3
ffffffff80001c97:	e9 64 f3 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001c9c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001ca0 <irq_196>:
ffffffff80001ca0:	6a 00                	push   $0x0
ffffffff80001ca2:	68 c4 00 00 00       	push   $0xc4
ffffffff80001ca7:	e9 54 f3 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001cac:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001cb0 <irq_197>:
ffffffff80001cb0:	6a 00                	push   $0x0
ffffffff80001cb2:	68 c5 00 00 00       	push   $0xc5
ffffffff80001cb7:	e9 44 f3 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001cbc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001cc0 <irq_198>:
ffffffff80001cc0:	6a 00                	push   $0x0
ffffffff80001cc2:	68 c6 00 00 00       	push   $0xc6
ffffffff80001cc7:	e9 34 f3 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001ccc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001cd0 <irq_199>:
ffffffff80001cd0:	6a 00                	push   $0x0
ffffffff80001cd2:	68 c7 00 00 00       	push   $0xc7
ffffffff80001cd7:	e9 24 f3 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001cdc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001ce0 <irq_200>:
ffffffff80001ce0:	6a 00                	push   $0x0
ffffffff80001ce2:	68 c8 00 00 00       	push   $0xc8
ffffffff80001ce7:	e9 14 f3 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001cec:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001cf0 <irq_201>:
ffffffff80001cf0:	6a 00                	push   $0x0
ffffffff80001cf2:	68 c9 00 00 00       	push   $0xc9
ffffffff80001cf7:	e9 04 f3 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001cfc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001d00 <irq_202>:
ffffffff80001d00:	6a 00                	push   $0x0
ffffffff80001d02:	68 ca 00 00 00       	push   $0xca
ffffffff80001d07:	e9 f4 f2 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001d0c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001d10 <irq_203>:
ffffffff80001d10:	6a 00                	push   $0x0
ffffffff80001d12:	68 cb 00 00 00       	push   $0xcb
ffffffff80001d17:	e9 e4 f2 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001d1c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001d20 <irq_204>:
ffffffff80001d20:	6a 00                	push   $0x0
ffffffff80001d22:	68 cc 00 00 00       	push   $0xcc
ffffffff80001d27:	e9 d4 f2 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001d2c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001d30 <irq_205>:
ffffffff80001d30:	6a 00                	push   $0x0
ffffffff80001d32:	68 cd 00 00 00       	push   $0xcd
ffffffff80001d37:	e9 c4 f2 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001d3c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001d40 <irq_206>:
ffffffff80001d40:	6a 00                	push   $0x0
ffffffff80001d42:	68 ce 00 00 00       	push   $0xce
ffffffff80001d47:	e9 b4 f2 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001d4c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001d50 <irq_207>:
ffffffff80001d50:	6a 00                	push   $0x0
ffffffff80001d52:	68 cf 00 00 00       	push   $0xcf
ffffffff80001d57:	e9 a4 f2 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001d5c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001d60 <irq_208>:
ffffffff80001d60:	6a 00                	push   $0x0
ffffffff80001d62:	68 d0 00 00 00       	push   $0xd0
ffffffff80001d67:	e9 94 f2 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001d6c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001d70 <irq_209>:
ffffffff80001d70:	6a 00                	push   $0x0
ffffffff80001d72:	68 d1 00 00 00       	push   $0xd1
ffffffff80001d77:	e9 84 f2 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001d7c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001d80 <irq_210>:
ffffffff80001d80:	6a 00                	push   $0x0
ffffffff80001d82:	68 d2 00 00 00       	push   $0xd2
ffffffff80001d87:	e9 74 f2 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001d8c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001d90 <irq_211>:
ffffffff80001d90:	6a 00                	push   $0x0
ffffffff80001d92:	68 d3 00 00 00       	push   $0xd3
ffffffff80001d97:	e9 64 f2 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001d9c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001da0 <irq_212>:
ffffffff80001da0:	6a 00                	push   $0x0
ffffffff80001da2:	68 d4 00 00 00       	push   $0xd4
ffffffff80001da7:	e9 54 f2 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001dac:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001db0 <irq_213>:
ffffffff80001db0:	6a 00                	push   $0x0
ffffffff80001db2:	68 d5 00 00 00       	push   $0xd5
ffffffff80001db7:	e9 44 f2 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001dbc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001dc0 <irq_214>:
ffffffff80001dc0:	6a 00                	push   $0x0
ffffffff80001dc2:	68 d6 00 00 00       	push   $0xd6
ffffffff80001dc7:	e9 34 f2 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001dcc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001dd0 <irq_215>:
ffffffff80001dd0:	6a 00                	push   $0x0
ffffffff80001dd2:	68 d7 00 00 00       	push   $0xd7
ffffffff80001dd7:	e9 24 f2 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001ddc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001de0 <irq_216>:
ffffffff80001de0:	6a 00                	push   $0x0
ffffffff80001de2:	68 d8 00 00 00       	push   $0xd8
ffffffff80001de7:	e9 14 f2 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001dec:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001df0 <irq_217>:
ffffffff80001df0:	6a 00                	push   $0x0
ffffffff80001df2:	68 d9 00 00 00       	push   $0xd9
ffffffff80001df7:	e9 04 f2 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001dfc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001e00 <irq_218>:
ffffffff80001e00:	6a 00                	push   $0x0
ffffffff80001e02:	68 da 00 00 00       	push   $0xda
ffffffff80001e07:	e9 f4 f1 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001e0c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001e10 <irq_219>:
ffffffff80001e10:	6a 00                	push   $0x0
ffffffff80001e12:	68 db 00 00 00       	push   $0xdb
ffffffff80001e17:	e9 e4 f1 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001e1c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001e20 <irq_220>:
ffffffff80001e20:	6a 00                	push   $0x0
ffffffff80001e22:	68 dc 00 00 00       	push   $0xdc
ffffffff80001e27:	e9 d4 f1 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001e2c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001e30 <irq_221>:
ffffffff80001e30:	6a 00                	push   $0x0
ffffffff80001e32:	68 dd 00 00 00       	push   $0xdd
ffffffff80001e37:	e9 c4 f1 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001e3c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001e40 <irq_222>:
ffffffff80001e40:	6a 00                	push   $0x0
ffffffff80001e42:	68 de 00 00 00       	push   $0xde
ffffffff80001e47:	e9 b4 f1 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001e4c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001e50 <irq_223>:
ffffffff80001e50:	6a 00                	push   $0x0
ffffffff80001e52:	68 df 00 00 00       	push   $0xdf
ffffffff80001e57:	e9 a4 f1 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001e5c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001e60 <irq_224>:
ffffffff80001e60:	6a 00                	push   $0x0
ffffffff80001e62:	68 e0 00 00 00       	push   $0xe0
ffffffff80001e67:	e9 94 f1 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001e6c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001e70 <irq_225>:
ffffffff80001e70:	6a 00                	push   $0x0
ffffffff80001e72:	68 e1 00 00 00       	push   $0xe1
ffffffff80001e77:	e9 84 f1 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001e7c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001e80 <irq_226>:
ffffffff80001e80:	6a 00                	push   $0x0
ffffffff80001e82:	68 e2 00 00 00       	push   $0xe2
ffffffff80001e87:	e9 74 f1 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001e8c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001e90 <irq_227>:
ffffffff80001e90:	6a 00                	push   $0x0
ffffffff80001e92:	68 e3 00 00 00       	push   $0xe3
ffffffff80001e97:	e9 64 f1 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001e9c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001ea0 <irq_228>:
ffffffff80001ea0:	6a 00                	push   $0x0
ffffffff80001ea2:	68 e4 00 00 00       	push   $0xe4
ffffffff80001ea7:	e9 54 f1 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001eac:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001eb0 <irq_229>:
ffffffff80001eb0:	6a 00                	push   $0x0
ffffffff80001eb2:	68 e5 00 00 00       	push   $0xe5
ffffffff80001eb7:	e9 44 f1 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001ebc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001ec0 <irq_230>:
ffffffff80001ec0:	6a 00                	push   $0x0
ffffffff80001ec2:	68 e6 00 00 00       	push   $0xe6
ffffffff80001ec7:	e9 34 f1 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001ecc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001ed0 <irq_231>:
ffffffff80001ed0:	6a 00                	push   $0x0
ffffffff80001ed2:	68 e7 00 00 00       	push   $0xe7
ffffffff80001ed7:	e9 24 f1 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001edc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001ee0 <irq_232>:
ffffffff80001ee0:	6a 00                	push   $0x0
ffffffff80001ee2:	68 e8 00 00 00       	push   $0xe8
ffffffff80001ee7:	e9 14 f1 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001eec:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001ef0 <irq_233>:
ffffffff80001ef0:	6a 00                	push   $0x0
ffffffff80001ef2:	68 e9 00 00 00       	push   $0xe9
ffffffff80001ef7:	e9 04 f1 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001efc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001f00 <irq_234>:
ffffffff80001f00:	6a 00                	push   $0x0
ffffffff80001f02:	68 ea 00 00 00       	push   $0xea
ffffffff80001f07:	e9 f4 f0 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001f0c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001f10 <irq_235>:
ffffffff80001f10:	6a 00                	push   $0x0
ffffffff80001f12:	68 eb 00 00 00       	push   $0xeb
ffffffff80001f17:	e9 e4 f0 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001f1c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001f20 <irq_236>:
ffffffff80001f20:	6a 00                	push   $0x0
ffffffff80001f22:	68 ec 00 00 00       	push   $0xec
ffffffff80001f27:	e9 d4 f0 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001f2c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001f30 <irq_237>:
ffffffff80001f30:	6a 00                	push   $0x0
ffffffff80001f32:	68 ed 00 00 00       	push   $0xed
ffffffff80001f37:	e9 c4 f0 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001f3c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001f40 <irq_238>:
ffffffff80001f40:	6a 00                	push   $0x0
ffffffff80001f42:	68 ee 00 00 00       	push   $0xee
ffffffff80001f47:	e9 b4 f0 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001f4c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001f50 <irq_239>:
ffffffff80001f50:	6a 00                	push   $0x0
ffffffff80001f52:	68 ef 00 00 00       	push   $0xef
ffffffff80001f57:	e9 a4 f0 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001f5c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001f60 <irq_240>:
ffffffff80001f60:	6a 00                	push   $0x0
ffffffff80001f62:	68 f0 00 00 00       	push   $0xf0
ffffffff80001f67:	e9 94 f0 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001f6c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001f70 <irq_241>:
ffffffff80001f70:	6a 00                	push   $0x0
ffffffff80001f72:	68 f1 00 00 00       	push   $0xf1
ffffffff80001f77:	e9 84 f0 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001f7c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001f80 <irq_242>:
ffffffff80001f80:	6a 00                	push   $0x0
ffffffff80001f82:	68 f2 00 00 00       	push   $0xf2
ffffffff80001f87:	e9 74 f0 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001f8c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001f90 <irq_243>:
ffffffff80001f90:	6a 00                	push   $0x0
ffffffff80001f92:	68 f3 00 00 00       	push   $0xf3
ffffffff80001f97:	e9 64 f0 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001f9c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001fa0 <irq_244>:
ffffffff80001fa0:	6a 00                	push   $0x0
ffffffff80001fa2:	68 f4 00 00 00       	push   $0xf4
ffffffff80001fa7:	e9 54 f0 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001fac:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001fb0 <irq_245>:
ffffffff80001fb0:	6a 00                	push   $0x0
ffffffff80001fb2:	68 f5 00 00 00       	push   $0xf5
ffffffff80001fb7:	e9 44 f0 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001fbc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001fc0 <irq_246>:
ffffffff80001fc0:	6a 00                	push   $0x0
ffffffff80001fc2:	68 f6 00 00 00       	push   $0xf6
ffffffff80001fc7:	e9 34 f0 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001fcc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001fd0 <irq_247>:
ffffffff80001fd0:	6a 00                	push   $0x0
ffffffff80001fd2:	68 f7 00 00 00       	push   $0xf7
ffffffff80001fd7:	e9 24 f0 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001fdc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001fe0 <irq_248>:
ffffffff80001fe0:	6a 00                	push   $0x0
ffffffff80001fe2:	68 f8 00 00 00       	push   $0xf8
ffffffff80001fe7:	e9 14 f0 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001fec:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80001ff0 <irq_249>:
ffffffff80001ff0:	6a 00                	push   $0x0
ffffffff80001ff2:	68 f9 00 00 00       	push   $0xf9
ffffffff80001ff7:	e9 04 f0 ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff80001ffc:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80002000 <irq_250>:
ffffffff80002000:	6a 00                	push   $0x0
ffffffff80002002:	68 fa 00 00 00       	push   $0xfa
ffffffff80002007:	e9 f4 ef ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000200c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80002010 <irq_251>:
ffffffff80002010:	6a 00                	push   $0x0
ffffffff80002012:	68 fb 00 00 00       	push   $0xfb
ffffffff80002017:	e9 e4 ef ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000201c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80002020 <irq_252>:
ffffffff80002020:	6a 00                	push   $0x0
ffffffff80002022:	68 fc 00 00 00       	push   $0xfc
ffffffff80002027:	e9 d4 ef ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000202c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80002030 <irq_253>:
ffffffff80002030:	6a 00                	push   $0x0
ffffffff80002032:	68 fd 00 00 00       	push   $0xfd
ffffffff80002037:	e9 c4 ef ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000203c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80002040 <irq_254>:
ffffffff80002040:	6a 00                	push   $0x0
ffffffff80002042:	68 fe 00 00 00       	push   $0xfe
ffffffff80002047:	e9 b4 ef ff ff       	jmp    ffffffff80001000 <interrupt_common>
ffffffff8000204c:	0f 1f 40 00          	nopl   0x0(%rax)

ffffffff80002050 <irq_255>:
ffffffff80002050:	6a 00                	push   $0x0
ffffffff80002052:	68 ff 00 00 00       	push   $0xff
ffffffff80002057:	e9 a4 ef ff ff       	jmp    ffffffff80001000 <interrupt_common>

ffffffff8000205c <core::panicking::panic_fmt>:
ffffffff8000205c:	50                   	push   %rax
ffffffff8000205d:	e8 4d 00 00 00       	call   ffffffff800020af <rust_begin_unwind>

ffffffff80002062 <core::option::expect_failed>:
ffffffff80002062:	50                   	push   %rax
ffffffff80002063:	e8 00 00 00 00       	call   ffffffff80002068 <core::panicking::panic_str>

ffffffff80002068 <core::panicking::panic_str>:
ffffffff80002068:	48 83 ec 18          	sub    $0x18,%rsp
ffffffff8000206c:	48 8d 44 24 08       	lea    0x8(%rsp),%rax
ffffffff80002071:	48 89 38             	mov    %rdi,(%rax)
ffffffff80002074:	48 89 70 08          	mov    %rsi,0x8(%rax)
ffffffff80002078:	48 89 c7             	mov    %rax,%rdi
ffffffff8000207b:	e8 00 00 00 00       	call   ffffffff80002080 <core::panicking::panic_display>

ffffffff80002080 <core::panicking::panic_display>:
ffffffff80002080:	50                   	push   %rax
ffffffff80002081:	e8 d6 ff ff ff       	call   ffffffff8000205c <core::panicking::panic_fmt>

ffffffff80002086 <irq_handler>:
ffffffff80002086:	8a 47 78             	mov    0x78(%rdi),%al
ffffffff80002089:	3c 20                	cmp    $0x20,%al
ffffffff8000208b:	72 21                	jb     ffffffff800020ae <irq_handler+0x28>
ffffffff8000208d:	8a 0d 6d 3f 00 00    	mov    0x3f6d(%rip),%cl        # ffffffff80006000 <silicium_x86_64::apic::io::IRQ_COUNT>
ffffffff80002093:	84 c9                	test   %cl,%cl
ffffffff80002095:	74 17                	je     ffffffff800020ae <irq_handler+0x28>
ffffffff80002097:	80 c1 20             	add    $0x20,%cl
ffffffff8000209a:	38 c1                	cmp    %al,%cl
ffffffff8000209c:	76 10                	jbe    ffffffff800020ae <irq_handler+0x28>
ffffffff8000209e:	48 b8 b0 00 e0 fe 00 	movabs $0xffff8000fee000b0,%rax
ffffffff800020a5:	80 ff ff 
ffffffff800020a8:	c7 00 00 00 00 00    	movl   $0x0,(%rax)
ffffffff800020ae:	c3                   	ret

ffffffff800020af <rust_begin_unwind>:
ffffffff800020af:	50                   	push   %rax
ffffffff800020b0:	fa                   	cli
ffffffff800020b1:	8a 05 49 6f 00 00    	mov    0x6f49(%rip),%al        # ffffffff80009000 <silicium_x86_64::smp::AP_BOOTED>
ffffffff800020b7:	84 c0                	test   %al,%al
ffffffff800020b9:	74 21                	je     ffffffff800020dc <rust_begin_unwind+0x2d>
ffffffff800020bb:	48 b8 00 03 e0 fe 00 	movabs $0xffff8000fee00300,%rax
ffffffff800020c2:	80 ff ff 
ffffffff800020c5:	c7 40 10 00 00 00 00 	movl   $0x0,0x10(%rax)
ffffffff800020cc:	c7 00 00 02 0c 00    	movl   $0xc0200,(%rax)
ffffffff800020d2:	0f ba 20 0c          	btl    $0xc,(%rax)
ffffffff800020d6:	73 04                	jae    ffffffff800020dc <rust_begin_unwind+0x2d>
ffffffff800020d8:	f3 90                	pause
ffffffff800020da:	eb f6                	jmp    ffffffff800020d2 <rust_begin_unwind+0x23>
ffffffff800020dc:	fa                   	cli
ffffffff800020dd:	f4                   	hlt
ffffffff800020de:	eb fc                	jmp    ffffffff800020dc <rust_begin_unwind+0x2d>

ffffffff800020e0 <silicium_x86_64::paging::pml4::Pml4::set_current>:
ffffffff800020e0:	50                   	push   %rax
ffffffff800020e1:	48 83 3d 17 5f 00 00 	cmpq   $0x0,0x5f17(%rip)        # ffffffff80008000 <silicium_x86_64::paging::KERNEL_PML4+0x1000>
ffffffff800020e8:	00 
ffffffff800020e9:	74 0c                	je     ffffffff800020f7 <silicium_x86_64::paging::pml4::Pml4::set_current+0x17>
ffffffff800020eb:	48 8b 0d 16 5f 00 00 	mov    0x5f16(%rip),%rcx        # ffffffff80008008 <silicium_x86_64::paging::KERNEL_PML4+0x1008>
ffffffff800020f2:	e9 b6 00 00 00       	jmp    ffffffff800021ad <silicium_x86_64::paging::pml4::Pml4::set_current+0xcd>
ffffffff800020f7:	48 b8 00 f0 ff ff ff 	movabs $0xffffffffff000,%rax
ffffffff800020fe:	ff 0f 00 
ffffffff80002101:	48 c7 c1 00 70 00 80 	mov    $0xffffffff80007000,%rcx
ffffffff80002108:	48 c7 c2 00 70 00 80 	mov    $0xffffffff80007000,%rdx
ffffffff8000210f:	48 c1 ea 15          	shr    $0x15,%rdx
ffffffff80002113:	48 c7 c6 00 70 00 80 	mov    $0xffffffff80007000,%rsi
ffffffff8000211a:	48 c1 ee 1e          	shr    $0x1e,%rsi
ffffffff8000211e:	48 c7 c7 00 70 00 80 	mov    $0xffffffff80007000,%rdi
ffffffff80002125:	48 c1 ef 27          	shr    $0x27,%rdi
ffffffff80002129:	45 31 c9             	xor    %r9d,%r9d
ffffffff8000212c:	41 b0 01             	mov    $0x1,%r8b
ffffffff8000212f:	45 0f b6 d1          	movzbl %r9b,%r10d
ffffffff80002133:	49 89 f9             	mov    %rdi,%r9
ffffffff80002136:	42 ff 24 d5 00 30 00 	jmp    *-0x7fffd000(,%r10,8)
ffffffff8000213d:	80 
ffffffff8000213e:	41 b0 02             	mov    $0x2,%r8b
ffffffff80002141:	49 89 f1             	mov    %rsi,%r9
ffffffff80002144:	eb 06                	jmp    ffffffff8000214c <silicium_x86_64::paging::pml4::Pml4::set_current+0x6c>
ffffffff80002146:	41 b0 03             	mov    $0x3,%r8b
ffffffff80002149:	49 89 d1             	mov    %rdx,%r9
ffffffff8000214c:	41 81 e1 ff 01 00 00 	and    $0x1ff,%r9d
ffffffff80002153:	4e 8b 0c c9          	mov    (%rcx,%r9,8),%r9
ffffffff80002157:	41 f6 c1 01          	test   $0x1,%r9b
ffffffff8000215b:	74 55                	je     ffffffff800021b2 <silicium_x86_64::paging::pml4::Pml4::set_current+0xd2>
ffffffff8000215d:	49 21 c1             	and    %rax,%r9
ffffffff80002160:	48 b9 00 00 00 00 00 	movabs $0xffff800000000000,%rcx
ffffffff80002167:	80 ff ff 
ffffffff8000216a:	4c 01 c9             	add    %r9,%rcx
ffffffff8000216d:	45 0f b6 d0          	movzbl %r8b,%r10d
ffffffff80002171:	49 89 f9             	mov    %rdi,%r9
ffffffff80002174:	41 b0 01             	mov    $0x1,%r8b
ffffffff80002177:	42 ff 24 d5 00 30 00 	jmp    *-0x7fffd000(,%r10,8)
ffffffff8000217e:	80 
ffffffff8000217f:	48 c7 c2 00 70 00 80 	mov    $0xffffffff80007000,%rdx
ffffffff80002186:	c1 ea 09             	shr    $0x9,%edx
ffffffff80002189:	81 e2 f8 0f 00 00    	and    $0xff8,%edx
ffffffff8000218f:	48 8b 0c 11          	mov    (%rcx,%rdx,1),%rcx
ffffffff80002193:	f6 c1 01             	test   $0x1,%cl
ffffffff80002196:	74 1a                	je     ffffffff800021b2 <silicium_x86_64::paging::pml4::Pml4::set_current+0xd2>
ffffffff80002198:	48 21 c1             	and    %rax,%rcx
ffffffff8000219b:	48 c7 05 5a 5e 00 00 	movq   $0x1,0x5e5a(%rip)        # ffffffff80008000 <silicium_x86_64::paging::KERNEL_PML4+0x1000>
ffffffff800021a2:	01 00 00 00 
ffffffff800021a6:	48 89 0d 5b 5e 00 00 	mov    %rcx,0x5e5b(%rip)        # ffffffff80008008 <silicium_x86_64::paging::KERNEL_PML4+0x1008>
ffffffff800021ad:	0f 22 d9             	mov    %rcx,%cr3
ffffffff800021b0:	58                   	pop    %rax
ffffffff800021b1:	c3                   	ret
ffffffff800021b2:	be 2c 00 00 00       	mov    $0x2c,%esi
ffffffff800021b7:	48 c7 c7 40 31 00 80 	mov    $0xffffffff80003140,%rdi
ffffffff800021be:	e8 9f fe ff ff       	call   ffffffff80002062 <core::option::expect_failed>

ffffffff800021c3 <compiler_builtins::mem::memcpy>:
ffffffff800021c3:	48 89 f8             	mov    %rdi,%rax
ffffffff800021c6:	41 89 c0             	mov    %eax,%r8d
ffffffff800021c9:	41 f7 d8             	neg    %r8d
ffffffff800021cc:	41 83 e0 07          	and    $0x7,%r8d
ffffffff800021d0:	49 39 d0             	cmp    %rdx,%r8
ffffffff800021d3:	4c 0f 43 c2          	cmovae %rdx,%r8
ffffffff800021d7:	4c 89 c1             	mov    %r8,%rcx
ffffffff800021da:	f3 a4                	rep movsb %ds:(%rsi),%es:(%rdi)
ffffffff800021dc:	4c 29 c2             	sub    %r8,%rdx
ffffffff800021df:	48 89 d1             	mov    %rdx,%rcx
ffffffff800021e2:	48 c1 e9 03          	shr    $0x3,%rcx
ffffffff800021e6:	f3 48 a5             	rep movsq %ds:(%rsi),%es:(%rdi)
ffffffff800021e9:	83 e2 07             	and    $0x7,%edx
ffffffff800021ec:	48 89 d1             	mov    %rdx,%rcx
ffffffff800021ef:	f3 a4                	rep movsb %ds:(%rsi),%es:(%rdi)
ffffffff800021f1:	c3                   	ret

ffffffff800021f2 <memcpy>:
ffffffff800021f2:	e9 cc ff ff ff       	jmp    ffffffff800021c3 <compiler_builtins::mem::memcpy>

ffffffff800021f7 <compiler_builtins::mem::memset>:
ffffffff800021f7:	49 89 d0             	mov    %rdx,%r8
ffffffff800021fa:	48 89 fa             	mov    %rdi,%rdx
ffffffff800021fd:	40 0f b6 ce          	movzbl %sil,%ecx
ffffffff80002201:	48 b8 01 01 01 01 01 	movabs $0x101010101010101,%rax
ffffffff80002208:	01 01 01 
ffffffff8000220b:	48 0f af c1          	imul   %rcx,%rax
ffffffff8000220f:	89 d1                	mov    %edx,%ecx
ffffffff80002211:	f7 d9                	neg    %ecx
ffffffff80002213:	83 e1 07             	and    $0x7,%ecx
ffffffff80002216:	4c 39 c1             	cmp    %r8,%rcx
ffffffff80002219:	49 0f 43 c8          	cmovae %r8,%rcx
ffffffff8000221d:	49 29 c8             	sub    %rcx,%r8
ffffffff80002220:	4c 89 c6             	mov    %r8,%rsi
ffffffff80002223:	48 c1 ee 03          	shr    $0x3,%rsi
ffffffff80002227:	41 83 e0 07          	and    $0x7,%r8d
ffffffff8000222b:	f3 aa                	rep stos %al,%es:(%rdi)
ffffffff8000222d:	48 89 f1             	mov    %rsi,%rcx
ffffffff80002230:	f3 48 ab             	rep stos %rax,%es:(%rdi)
ffffffff80002233:	4c 89 c1             	mov    %r8,%rcx
ffffffff80002236:	f3 aa                	rep stos %al,%es:(%rdi)
ffffffff80002238:	48 89 d0             	mov    %rdx,%rax
ffffffff8000223b:	c3                   	ret

ffffffff8000223c <memset>:
ffffffff8000223c:	e9 b6 ff ff ff       	jmp    ffffffff800021f7 <compiler_builtins::mem::memset>
