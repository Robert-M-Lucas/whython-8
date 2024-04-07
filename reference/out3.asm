    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern WriteConsoleA
    extern WriteConsoleW
    section .text

__14:
	push rbp
	mov rbp, rsp
	sub rsp, 64

	mov qword [rbp-8], 0D0Ah
	mov qword [rbp-16], "true"
	mov rax, [rbp+16]
	cmp rax, 0
	jz ._14.true
	mov qword [rbp-8], `0D0A0065h`
	mov qword [rbp-16], "fals"
	._14.true:

	mov ecx, -11
	call GetStdHandle

    ; You have to reserve space for these despite not being on the stack!
	mov rcx, rax ; STD Handle
	mov rdx, rbp ; Data pointer
	sub rdx, 16 ; cont.
	mov r8, 16 ; Bytes to write
	mov qword [rbp - 24], 0 ; optional out bytes written
	mov r9, rbp
	sub r9, 24 ; contd.
	mov qword [rbp - 32], 0 ; optional lpOverlapped
	call WriteFile
	leave
	ret

main:
	push rbp
	mov rbp, rsp
	sub rsp, 16
	mov qword [rbp-8], 1
	mov rax, qword [rbp-8]
	push qword 0
	push rax
	call __14
	add rsp, 16
	mov qword [rbp-16], 1
	mov rcx, [rbp-16]
	call ExitProcess