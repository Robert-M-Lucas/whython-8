    global main

    section .text

main:
    push rbp
    mov  rbp, rsp
    sub  rsp, 16
    
	mov qword [rbp-16], 12
	mov rax, qword [rbp-16]
	leave
	ret