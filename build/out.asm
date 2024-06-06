    global main

    section .text

main:
    push rbp
    mov  rbp, rsp
    sub  rsp, 64
    
    mov qword [rbp-16], 100
    mov qword [rbp-24], 100
    mov rax, qword [rbp-16]
    add rax, qword [rbp-24]
    mov qword [rbp-8], rax
    mov rax, qword [rbp-8]
    mov qword [rbp-40], rax
    mov qword [rbp-48], 10
    mov rax, qword [rbp-40]
    add rax, qword [rbp-48]
    mov qword [rbp-32], rax
    mov rax, qword [rbp-32]
    mov qword [rbp-56], rax
	mov rax, qword [rbp-56]
	leave
	ret