    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern WriteConsoleA
    extern WriteConsoleW
    section .text

__4:
	push rbp
	mov rbp, rsp
	sub rsp, 80

	mov rcx, rbp
	mov rax, qword [rbp+16]
	mov qword [rbp-24], ""
	mov qword [rbp-16], ""
    mov qword [rbp-8], ""
	cmp rax, 0
	jg ._test
	mov dword [rbp-20], "-"
	mov r8, rax
	mov rax, 0
	sub rax, r8
	._test
	mov rbx, 10
	mov r8, rbp
    sub r8, 24
	._4.loop:
	xor rdx, rdx
	div rbx
	dec rcx
	add rdx, '0'
	mov [rcx], dl
	cmp rcx, r8
	jz ._4.loop
	test rax, rax
	jnz ._4.loop

	mov ecx, -11
	call GetStdHandle

	; You have to reserve space for these despite not being on the stack!
    mov rcx, rax ; STD Handle
    mov rdx, rbp ; Data pointer
    sub rdx, 24 ; cont.
    mov r8, 24 ; Bytes to write
    mov qword [rbp - 40], 0 ; optional out bytes written
    mov r9, rbp
    sub r9, 24 ; contd.
    mov qword [rbp - 48], 0 ; optional lpOverlapped
    call WriteFile

    leave
    ret

main:
	push rbp
	mov rbp, rsp
	sub rsp, 16
	mov rax, qword -18446744073709515615
	mov qword [rbp-8], rax
	push qword 0
	mov rax, qword [rbp-8]
	push rax
	call __4
	add rsp, 8
	mov qword [rbp-16], 1
	mov rcx, [rbp-8]
	call ExitProcess
