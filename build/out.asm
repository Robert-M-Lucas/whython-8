    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 3
    mov qword [rbp-16], 4
    mov byte [rbp-17], 1
    mov al, byte [rbp-17]
    mov byte [rbp-18], al
    mov rdi, __6_fstr
    mov rsi, 0
    mov sil, [rbp-18]
    mov al, 0
    sub rsp, 18
    extern printf
    call printf
    add rsp, 18
    mov al, byte [rbp-17]
    mov byte [rbp-19], al
    cmp byte [rbp-19], 0
    jz main_0
    mov rax, qword [rbp-8]
    mov qword [rbp-27], rax
    mov rdi, __4_fstr
    mov rsi, [rbp-27]
    mov al, 0
    sub rsp, 27
    extern printf
    call printf
    add rsp, 27
    mov qword [rbp-35], 13
    mov rax, qword [rbp-35]
    leave
    ret
    jmp main_1
    main_0:
    mov rax, qword [rbp-16]
    mov qword [rbp-27], rax
    mov rdi, __4_fstr
    mov rsi, [rbp-27]
    mov al, 0
    sub rsp, 27
    extern printf
    call printf
    add rsp, 27
    mov qword [rbp-35], 12
    mov rax, qword [rbp-35]
    leave
    ret
    main_1:


section .data_readonly
    __6_fstr db `Boolean: %d\n`,0
    __4_fstr db `Integer: %d\n`,0