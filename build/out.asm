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
    mov qword [rbp-40], 1
    mov rax, qword [rbp-40]
    leave
    ret


_1:
    push rbp
    mov rbp, rsp
    mov rax, qword [rbp+16]
    mov qword [rbp-8], rax
    mov rdi, __4_fstr
    mov rsi, [rbp-8]
    mov al, 0
    sub rsp, 8
    extern printf
    call printf
    add rsp, 8
    mov rax, qword [rbp+16]
    mov qword [rbp-17], rax
    mov qword [rbp-25], 233
    mov rax, qword [rbp-17]
    cmp rax, qword [rbp-25]
    jz __5_0
    mov byte [rbp-9], 0
    jmp __5_1
    __5_0:
    mov byte [rbp-9], 1
    __5_1:
    cmp byte [rbp-9], 0
    jz _1_2
    mov qword [rbp-33], 10
    mov rax, 60
    mov rdi, [rbp-33]
    syscall
    _1_2:
    mov rax, qword [rbp+24]
    mov qword [rbp-33], rax
    mov rax, qword [rbp+16]
    mov qword [rbp-49], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-57], rax
    mov rax, qword [rbp-49]
    add rax, qword [rbp-57]
    mov qword [rbp-41], rax
    mov rax, qword [rbp-33]
    mov qword [rbp-65], rax
    mov rax, qword [rbp-41]
    mov qword [rbp-73], rax
    mov rax, qword [rbp-73]
    mov qword [rbp-97], rax
    mov rax, qword [rbp-65]
    mov qword [rbp-105], rax
    sub rsp, 105
    call _1
    add rsp, 105


section .data_readonly
    __4_fstr db `Integer: %d\n`,0