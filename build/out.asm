    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-16], 3
    mov qword [rbp-8], 4
    mov rax, rbp
    add rax, -16
    mov qword [rbp-24], rax
    mov rax, rbp
    add rax, -16
    mov qword [rbp-32], rax
    mov rax, qword [rbp-32]
    mov qword [rbp-40], rax
    sub rsp, 40
    call _3
    add rsp, 40
    mov rax, rbp
    add rax, -16
    mov qword [rbp-40], rax
    mov rax, qword [rbp-40]
    mov qword [rbp-48], rax
    sub rsp, 48
    call _4
    add rsp, 48
    mov qword [rbp-48], 2
    mov rax, qword [rbp-24]
    mov rdx, qword [rbp-48]
    add qword [rax], rdx
    mov rax, rbp
    add rax, -16
    mov qword [rbp-56], rax
    mov rax, qword [rbp-56]
    mov qword [rbp-64], rax
    sub rsp, 64
    call _3
    add rsp, 64
    mov qword [rbp-64], 7
    mov rax, qword [rbp-64]
    leave
    ret


_3:
    push rbp
    mov rbp, rsp
    mov rax, [rbp+16]
    add rax, 0
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
    mov rax, [rbp+16]
    add rax, 8
    mov qword [rbp-32], rax
    mov rdx, qword [rbp-32]
    mov rax, qword [rdx+0]
    mov qword [rbp-24], rax
    mov rdi, __8_fstr
    mov rsi, [rbp-24]
    mov al, 0
    sub rsp, 32
    extern printf
    call printf
    add rsp, 32

leave
ret

_4:
    push rbp
    mov rbp, rsp
    mov rax, [rbp+16]
    add rax, 0
    mov qword [rbp-8], rax
    mov qword [rbp-16], 1
    mov rax, qword [rbp-8]
    mov rdx, qword [rbp-16]
    add qword [rax], rdx
    mov rax, [rbp+16]
    add rax, 8
    mov qword [rbp-24], rax
    mov qword [rbp-32], 1
    mov rax, qword [rbp-24]
    mov rdx, qword [rbp-32]
    add qword [rax], rdx

leave
ret

section .data_readonly
    __8_fstr db `Integer: %ld\n`,0