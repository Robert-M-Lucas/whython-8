    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 0
    main_0:
    mov rax, qword [rbp-8]
    mov qword [rbp-17], rax
    mov qword [rbp-25], 1000000000
    mov rax, qword [rbp-25]
    cmp rax, qword [rbp-17]
    jg __18_2
    mov byte [rbp-9], 0
    jmp __18_3
    __18_2:
    mov byte [rbp-9], 1
    __18_3:
    cmp byte [rbp-9], 0
    jz main_1
    mov rax, rbp
    add rax, -8
    mov qword [rbp-33], rax
    mov qword [rbp-41], 1
    mov rax, qword [rbp-33]
    mov rdx, qword [rbp-41]
    add qword [rax], rdx
    jmp main_0
    main_1:
    mov rax, qword [rbp-8]
    mov qword [rbp-33], rax
    mov rdi, __8_fstr
    mov rsi, [rbp-33]
    mov al, 0
    sub rsp, 33
    extern printf
    call printf
    add rsp, 33
    mov qword [rbp-41], 1
    mov rax, qword [rbp-41]
    leave
    ret


section .data_readonly
    __8_fstr db `Integer: %ld\n`,0