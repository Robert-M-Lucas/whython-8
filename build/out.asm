    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 13
    mov qword [rbp-32], 190
    mov qword [rbp-24], 6
    mov qword [rbp-16], 8
    mov rax, rbp
    add rax, -32
    mov qword [rbp-40], rax
    mov rax, qword [rbp-40]
    mov qword [rbp-48], rax
    sub rsp, 48
    call _4
    add rsp, 48
    mov rax, rbp
    add rax, -8
    mov qword [rbp-48], rax
    mov rax, qword [rbp-24]
    mov qword [rbp-56], rax
    mov rax, qword [rbp-48]
    mov rdx, qword [rbp-56]
    add qword [rax], rdx
    mov rax, qword [rbp-8]
    mov qword [rbp-64], rax
    mov rax, qword [rbp-64]
    leave
    ret


_4:
    push rbp
    mov rbp, rsp
    mov rdx, qword [rbp+16]
    add rdx, 0
    mov rax, qword [rdx+0]
    mov qword [rbp-8], rax
    mov rdi, __8_fstr
    mov rsi, [rbp-8]
    mov al, 0
    sub rsp, 8
    extern printf
    call printf
    add rsp, 8

leave
ret

section .data_readonly
    __8_fstr db `Integer: %ld\n`,0