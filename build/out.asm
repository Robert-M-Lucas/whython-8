    global main

section .text

_2:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 1
    mov rax, rbp
    add rax, -8
    mov qword [rbp+16], rax
    leave
    ret


main:
    push rbp
    mov rbp, rsp
    sub rsp, 24
    call _2
    add rsp, 24
    mov rax, qword [rbp-24]
    mov qword [rbp-8], rax
    mov byte [rbp-25], 0
    mov qword [rbp-33], 123
    mov qword [rbp-41], 123
    mov qword [rbp-49], 123
    mov qword [rbp-57], 123
    mov rdx, qword [rbp-8]
    mov rax, qword [rdx+0]
    mov qword [rbp-65], rax
    mov rax, qword [rbp-65]
    mov qword [rbp-73], rax
    mov rdi, __8_fstr
    mov rsi, [rbp-73]
    mov al, 0
    sub rsp, 73
    extern printf
    call printf
    add rsp, 73
    mov qword [rbp-81], 1
    mov rax, qword [rbp-81]
    leave
    ret


section .data_readonly
    __8_fstr db `Integer: %ld\n`,0