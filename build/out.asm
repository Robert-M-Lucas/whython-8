    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern HeapAlloc
    extern HeapFree
    extern GetProcessHeap
    extern printf
    section .text

__2: ; printi
	push rbp
	mov rbp, rsp
	sub rsp, 16
	; [inline asm]
	mov dword [rbp-4], 0x000a
	mov dword [rbp-8], 0x646C6C25
	mov rcx, rbp
	sub rcx, 8
	mov rdx, qword [rbp+16]
	sub rsp, 40
	call printf
	add rsp, 40
	leave
	ret

__3: ; printb
	push rbp
	mov rbp, rsp
	sub rsp, 32
	; [inline asm]
	mov dword [rbp-8], 0x65757274
	mov dword [rbp-4], 0x0D0A
	mov rax, qword [rbp+16]
	cmp rax, 0
	jz ._3.true
	mov dword [rbp-8], 0x736C6166
	mov dword [rbp-4], 0x0D0A65
	._3.true:
	mov rcx, rbp
	sub rcx, 8
	mov rdx, qword [rbp+16]
	sub rsp, 40
	call printf
	add rsp, 40
	leave
	ret

__37: ; printf
	push rbp
	mov rbp, rsp
	sub rsp, 16
	; [inline asm]
	mov dword [rbp-4], 0x00
	mov dword [rbp-8], 0x0a664C25
	mov rcx, rbp
	sub rcx, 8
	movsd xmm1, qword [rbp+16]
	mov rdx, qword [rbp+16]
	sub rsp, 40
	call printf
	add rsp, 40
	leave
	ret

main: ; main
	push rbp
	mov rbp, rsp
	sub rsp, 160
	; '    printb(true);'
	; [inline asm]
	mov qword [rbp-8], 0
	; [no return call] -3 , [(-8, 8)]
	sub rsp, 8
	mov rax, qword [rbp-8]
	mov qword [rbp-168], rax
	call __3
	add rsp, 8
	; '    printb(true);'
	; [inline asm]
	mov qword [rbp-16], 0
	; [no return call] -3 , [(-16, 8)]
	sub rsp, 8
	mov rax, qword [rbp-16]
	mov qword [rbp-168], rax
	call __3
	add rsp, 8
	; '    printb(false);'
	; [inline asm]
	mov qword [rbp-24], 1
	; [no return call] -3 , [(-24, 8)]
	sub rsp, 8
	mov rax, qword [rbp-24]
	mov qword [rbp-168], rax
	call __3
	add rsp, 8
	; '    printb(false);'
	; [inline asm]
	mov qword [rbp-32], 1
	; [no return call] -3 , [(-32, 8)]
	sub rsp, 8
	mov rax, qword [rbp-32]
	mov qword [rbp-168], rax
	call __3
	add rsp, 8
	; '    printi(9223372036854775808);'
	; [inline asm]
	mov dword [rbp-40], 0x00000000
	mov dword [rbp-36], 0x80000000
	; [no return call] -2 , [(-40, 8)]
	sub rsp, 8
	mov rax, qword [rbp-40]
	mov qword [rbp-168], rax
	call __2
	add rsp, 8
	; '    printi(-9223372036854775807);'
	; [inline asm]
	mov dword [rbp-48], 0xffffffff
	mov dword [rbp-44], 0x7fffffff
	; [inline asm]
	mov dword [rbp-56], 0x00000000
	mov dword [rbp-52], 0x00000000
	; [inline asm]
	; [inline asm]
	mov rax, qword [rbp-56]
	sub rax, [rbp-48]
	mov [rbp-64], rax
	; [no return call] -2 , [(-64, 8)]
	sub rsp, 8
	mov rax, qword [rbp-64]
	mov qword [rbp-168], rax
	call __2
	add rsp, 8
	; '    printf(1.12351234123);'
	; [inline asm]
	mov rax, __float64__(1.12351234123)
	mov qword [rbp-72], rax
	; [no return call] -37 , [(-72, 8)]
	sub rsp, 8
	mov rax, qword [rbp-72]
	mov qword [rbp-168], rax
	call __37
	add rsp, 8
	; '    printf(1.12351234124);'
	; [inline asm]
	mov rax, __float64__(1.12351234124)
	mov qword [rbp-80], rax
	; [no return call] -37 , [(-80, 8)]
	sub rsp, 8
	mov rax, qword [rbp-80]
	mov qword [rbp-168], rax
	call __37
	add rsp, 8
	; '    printf(-1.12351234123);'
	; [inline asm]
	mov rax, __float64__(1.12351234123)
	mov qword [rbp-88], rax
	; [inline asm]
	mov rax, __float64__(0.0)
	mov qword [rbp-96], rax
	; [inline asm]
	; [inline asm]
	movsd xmm0, qword [rbp-96]
	subsd xmm0, qword [rbp-88]
	movsd qword [rbp-104], xmm0
	; [no return call] -37 , [(-104, 8)]
	sub rsp, 8
	mov rax, qword [rbp-104]
	mov qword [rbp-168], rax
	call __37
	add rsp, 8
	; ''
	; '    printb(1.12351234123 == 1.12351234124);'
	; [inline asm]
	mov rax, __float64__(1.12351234123)
	mov qword [rbp-112], rax
	; [inline asm]
	mov rax, __float64__(1.12351234124)
	mov qword [rbp-120], rax
	; [inline asm]
	; [inline asm]
	movsd xmm0, qword [rbp-112]
	ucomisd xmm0, qword [rbp-120]
	mov qword [rbp-128], 0
	setne [rbp-128]
	; [no return call] -3 , [(-128, 8)]
	sub rsp, 8
	mov rax, qword [rbp-128]
	mov qword [rbp-168], rax
	call __3
	add rsp, 8
	; '    printb(1.12351234123 == 1.12351234123);'
	; [inline asm]
	mov rax, __float64__(1.12351234123)
	mov qword [rbp-136], rax
	; [inline asm]
	mov rax, __float64__(1.12351234123)
	mov qword [rbp-144], rax
	; [inline asm]
	; [inline asm]
	movsd xmm0, qword [rbp-136]
	ucomisd xmm0, qword [rbp-144]
	mov qword [rbp-152], 0
	setne [rbp-152]
	; [no return call] -3 , [(-152, 8)]
	sub rsp, 8
	mov rax, qword [rbp-152]
	mov qword [rbp-168], rax
	call __3
	add rsp, 8
	; ''
	; ''
	; '    return 7;'
	; [inline asm]
	mov dword [rbp-160], 0x00000007
	mov dword [rbp-156], 0x00000000
	; [return] Some((-160, 8))
	mov rcx, qword [rbp-160]
	call ExitProcess

formatStr:
	db `The int is %d\n`,0