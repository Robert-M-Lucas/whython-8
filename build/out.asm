    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 13
    mov rax, rbp
    add rax, -8
    mov qword [rbp-16], rax
    mov qword [rbp-24], 12
    mov rdx, qword [rbp-16]
    mov rax, qword [rbp-24]
    mov qword [rdx+0], rax
    mov rax, qword [rbp-8]
    mov qword [rbp-32], rax
    mov rdi, __8_fstr
    mov rsi, [rbp-32]
    mov al, 0
    sub rsp, 32
    extern printf
    call printf
    add rsp, 32
    mov qword [rbp-40], 1
    mov rax, qword [rbp-40]
    leave
    ret


section .data_readonly
    __8_fstr db `Integer: %ld\n`,0