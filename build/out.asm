    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 1
    mov rax, qword [rbp-8]
    mov qword [rbp-16], rax
    sub rsp, 16
    call _2
    add rsp, 16
    mov qword [rbp-16], 1
    mov rax, qword [rbp-16]
    leave
    ret


_2:
    push rbp
    mov rbp, rsp
    mov rax, qword [rbp+16]
    mov qword [rbp-8], rax
    mov rdi, __8_fstr
    mov rsi, [rbp-8]
    mov al, 0
    sub rsp, 8
    extern printf
    call printf
    add rsp, 8
    mov qword [rbp-16], 12
    mov qword [rbp-24], 12
    mov rax, rbp
    add rax, 16
    mov qword [rbp-32], rax
    mov qword [rbp-40], 1
    mov rax, qword [rbp-32]
    mov rdx, qword [rbp-40]
    add qword [rax], rdx
    mov rax, qword [rbp+16]
    mov qword [rbp-48], rax
    mov rax, qword [rbp-48]
    mov qword [rbp-56], rax
    sub rsp, 56
    call _2
    add rsp, 56

leave
ret

section .data_readonly
    __8_fstr db `Integer: %ld\n`,0