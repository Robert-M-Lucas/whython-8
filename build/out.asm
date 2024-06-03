    global main

    section .text

main:
    push rbp
    mov  rbp, rsp
    sub  rsp, 16
    
	mov qword [rbp-8], 12
	mov rax, qword [rbp-8]
	leave
	ret