    global main

section .text

main:
    push rbp
    mov rbp, rsp
    
    mov qword [rbp-8], 0
    mov qword [rbp-16], 1
    mov rax, qword [rbp-16]
    mov qword [rbp-40], rax
    mov rax, qword [rbp-8]
    mov qword [rbp-48], rax
    sub rsp, 48
    call _1
    add rsp, 48
    mov qword [rbp-40], 255
	mov rax, qword [rbp-40]
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
    mov rax, qword [rbp+24]
    mov qword [rbp-32], rax
    mov rax, qword [rbp-24]
    add rax, qword [rbp-32]
    mov qword [rbp-16], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-40], rax
    mov qword [rbp-48], 12
    mov rax, 60
    mov rdi, [rbp-48]
    syscall
    mov rax, qword [rbp-40]
    mov qword [rbp-56], rax
    mov rax, qword [rbp-16]
    mov qword [rbp-64], rax
    mov rax, qword [rbp-64]
    mov qword [rbp-88], rax
    mov rax, qword [rbp-56]
    mov qword [rbp-96], rax
    sub rsp, 96
    call _1
    add rsp, 96
	leave
	ret

section .data
    __10_fstr db `Integer: %d\n`,0