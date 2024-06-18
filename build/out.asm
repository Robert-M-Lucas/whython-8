    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 0
    mov qword [rbp-16], 0
    main_0:
    mov rax, qword [rbp-8]
    mov qword [rbp-25], rax
    mov qword [rbp-33], 0
    mov rax, qword [rbp-25]
    cmp rax, qword [rbp-33]
    jz __5_2
    mov byte [rbp-17], 0
    jmp __5_3
    __5_2:
    mov byte [rbp-17], 1
    __5_3:
    cmp byte [rbp-17], 0
    jz main_1
    mov rax, qword [rbp-8]
    mov qword [rbp-41], rax
    mov rdi, __4_fstr
    mov rsi, [rbp-41]
    mov al, 0
    sub rsp, 41
    extern printf
    call printf
    add rsp, 41
    mov rax, qword [rbp-16]
    mov qword [rbp-50], rax
    mov qword [rbp-58], 0
    mov rax, qword [rbp-50]
    cmp rax, qword [rbp-58]
    jz __5_4
    mov byte [rbp-42], 0
    jmp __5_5
    __5_4:
    mov byte [rbp-42], 1
    __5_5:
    cmp byte [rbp-42], 0
    jz main_6
    jmp main_1
    main_7:
    main_6:
    jmp main_0
    main_1:
    mov qword [rbp-41], 2
    mov rax, qword [rbp-41]
    leave
    ret


section .data_readonly
    __4_fstr db `Integer: %ld\n`,0