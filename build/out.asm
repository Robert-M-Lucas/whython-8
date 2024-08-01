    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 3
    mov rax, qword [rbp-8]
    mov qword [rbp-16], rax
    sub rsp, 16
    call _9
    add rsp, 16
    mov rax, rbp
    add rax, -8
    mov qword [rbp-24], rax
    mov rdx, qword [rbp-24]
    mov rax, qword [rdx+0]
    mov qword [rbp-16], rax
    mov rdi, __8_fstr
    mov rsi, [rbp-16]
    mov al, 0
    sub rsp, 24
    extern printf
    call printf
    add rsp, 24
    mov qword [rbp-32], 8
    mov rax, qword [rbp-32]
    leave
    ret


_9:
    push rbp
    mov rbp, rsp
    mov rax, rbp
    add rax, 16
    mov qword [rbp-16], rax
    mov rdx, qword [rbp-16]
    mov rax, qword [rdx+0]
    mov qword [rbp-8], rax
    mov rdi, __8_fstr
    mov rsi, [rbp-8]
    mov al, 0
    sub rsp, 16
    extern printf
    call printf
    add rsp, 16
    mov rax, rbp
    add rax, 16
    mov qword [rbp-24], rax
    mov qword [rbp-32], 1
    mov rax, qword [rbp-24]
    mov rdx, qword [rbp-32]
    add qword [rax], rdx
    mov rax, rbp
    add rax, 16
    mov qword [rbp-48], rax
    mov rdx, qword [rbp-48]
    mov rax, qword [rdx+0]
    mov qword [rbp-40], rax
    mov rdi, __8_fstr
    mov rsi, [rbp-40]
    mov al, 0
    sub rsp, 48
    extern printf
    call printf
    add rsp, 48

leave
ret

section .data_readonly
    __8_fstr db `Integer: %ld\n`,0