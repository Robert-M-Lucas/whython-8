    global main

section .text

main:
    push rbp
    mov rbp, rsp
    main_0:
    mov byte [rbp-1], 1
    cmp byte [rbp-1], 0
    jz main_1
    mov qword [rbp-17], 0
    mov rdi, 8
    sub rsp, 17
    extern malloc
    call malloc
    sub rsp, 17
    mov qword [rbp-25], rax
    mov rdx, qword [rbp-25]
    mov rax, qword [rbp-17]
    mov qword [rdx+0], rax
    mov rax, qword [rbp-25]
    mov qword [rbp-9], rax
    jmp main_0
    main_1:
    mov qword [rbp-9], 0
    mov rax, qword [rbp-9]
    leave
    ret


