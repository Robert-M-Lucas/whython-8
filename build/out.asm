    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 30
    mov rax, rbp
    add rax, -8
    mov qword [rbp-16], rax
    mov qword [rbp-24], 7
    mov rcx, qword [rbp-16]
    mov rax, qword [rcx]
    mov rdx, 0
    mov rbx, qword [rbp-24]
    idiv rbx
    mov qword [rcx], rdx
    mov rax, qword [rbp-8]
    mov qword [rbp-32], rax
    mov rdi, __4_fstr
    mov rsi, [rbp-32]
    mov al, 0
    sub rsp, 32
    extern printf
    call printf
    add rsp, 32
    mov qword [rbp-40], 2
    mov rax, qword [rbp-40]
    leave
    ret


section .data_readonly
    __4_fstr db `Integer: %ld\n`,0