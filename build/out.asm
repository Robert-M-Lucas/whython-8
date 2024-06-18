    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 0
    main_0:
    mov byte [rbp-9], 1
    cmp byte [rbp-9], 0
    jz main_1
    mov rax, rbp
    add rax, -8
    mov qword [rbp-17], rax
    mov qword [rbp-25], 1
    mov rax, qword [rbp-17]
    mov rdx, qword [rbp-25]
    add qword [rax], rdx
    mov rax, qword [rbp-8]
    mov qword [rbp-33], rax
    mov rdi, __4_fstr
    mov rsi, [rbp-33]
    mov al, 0
    sub rsp, 33
    extern printf
    call printf
    add rsp, 33
    mov rax, qword [rbp-8]
    mov qword [rbp-42], rax
    mov qword [rbp-50], 12
    mov rax, qword [rbp-42]
    cmp rax, qword [rbp-50]
    jz __5_2
    mov byte [rbp-34], 0
    jmp __5_3
    __5_2:
    mov byte [rbp-34], 1
    __5_3:
    cmp byte [rbp-34], 0
    jz main_4
    jmp main_1
    main_5:
    main_4:
    jmp main_0
    main_1:
    mov qword [rbp-17], 2
    mov rax, qword [rbp-17]
    leave
    ret


section .data_readonly
    __4_fstr db `Integer: %ld\n`,0