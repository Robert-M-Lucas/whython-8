    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 12
    mov rax, qword [rbp-8]
    mov qword [rbp-16], rax
    sub rsp, 16
    call _3
    add rsp, 16
    sub rsp, 32
    call _2
    add rsp, 32
    mov rax, qword [rbp-32]
    mov qword [rbp-16], rax
    mov byte [rbp-33], 0
    mov qword [rbp-41], 123
    mov qword [rbp-49], 123
    mov qword [rbp-57], 123
    mov qword [rbp-65], 123
    mov rdx, qword [rbp-16]
    mov rax, qword [rdx+0]
    mov qword [rbp-73], rax
    mov rax, qword [rbp-73]
    mov qword [rbp-81], rax
    mov rdi, __8_fstr
    mov rsi, [rbp-81]
    mov al, 0
    sub rsp, 81
    extern printf
    call printf
    add rsp, 81
    mov qword [rbp-89], 1
    mov rax, qword [rbp-89]
    leave
    ret


_2:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 1
    mov rax, rbp
    add rax, -8
    mov qword [rbp+16], rax
    leave
    ret


_3:
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

leave
ret

section .data_readonly
    __8_fstr db `Integer: %ld\n`,0