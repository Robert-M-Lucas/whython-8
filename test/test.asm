    global main

section .text

main:
    push rbp
    mov rbp, rsp

    mov qword [rbp-8], 3
    mov qword [rbp-16], 4

    mov rax, qword [rbp-16]
    leave
    ret

    mov qword [rbp-17], 0
    mov al, byte [rbp-17]
    mov byte [rbp-18], al
    cmp byte [rbp-18], 0



    jz main_0

    mov rax, qword [rbp-8]
    mov qword [rbp-26], rax

    mov rdi, __4_fstr
    mov rsi, [rbp-26]
    mov al, 0
    sub rsp, 26
    extern printf
    call printf
    add rsp, 26

    mov qword [rbp-34], 13
	mov rax, qword [rbp-34]
    leave
    ret
    jmp main_1
    main_0:
    mov rax, qword [rbp-16]
    mov qword [rbp-26], rax

    mov rdi, __4_fstr
    mov rsi, [rbp-26]
    mov al, 0
    sub rsp, 26
    extern printf
    call printf
    add rsp, 26

    mov qword [rbp-34], 12
	mov rax, qword [rbp-34]
    leave
    ret
    main_1:

section .data
    __4_fstr db `Integer: %d\n`,0