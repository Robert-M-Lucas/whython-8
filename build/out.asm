    global main

section .text

main:
    push rbp
    mov rbp, rsp
    
    mov qword [rbp-8], 1
    mov rax, qword [rbp-8]
    mov qword [rbp-16], rax
    sub rsp, 16
    call _1
    add rsp, 16
    mov qword [rbp-16], 255
	mov rax, qword [rbp-16]
	leave
	ret

_1:
    push rbp
    mov rbp, rsp
    
    mov rax, qword [rbp+16]
    mov qword [rbp-8], rax

    mov rdi, __10_fstr
    mov rsi, [rbp-8]
    mov al, 0
    sub rsp, 8
    extern printf
    call printf
    add rsp, 8
    
    mov rax, qword [rbp+16]
    mov qword [rbp-24], rax
    mov qword [rbp-32], 1
    mov rax, qword [rbp-24]
    add rax, qword [rbp-32]
    mov qword [rbp-16], rax
    mov rax, qword [rbp-16]
    mov qword [rbp-40], rax
    sub rsp, 40
    call _1
    add rsp, 40

section .data
    __10_fstr db `Integer: %d\n`,0