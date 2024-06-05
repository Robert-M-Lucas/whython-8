    global main

    section .text

main:
    push rbp
    mov  rbp, rsp
    sub  rsp, 32
    
	mov qword [rbp-8], 110
    mov rax, qword [rbp-8]
    mov qword [rbp-16], rax
    mov rax, qword [rbp-16]
    mov qword [rbp-24], rax
	mov rax, qword [rbp-24]
	leave
	ret