    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 13
    sub rsp, 24
    call _4
    add rsp, 24
    mov rax, qword [rbp-8]
    mov qword [rbp-32], rax
    mov rax, qword [rbp-32]
    leave
    ret


_4:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 12
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