    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov byte [rbp-1], 0
    mov rax, rbp
    add rax, -1
    mov qword [rbp-9], rax
    mov al, byte [rbp-1]
    mov byte [rbp-11], al
    mov al, byte [rbp-11]
    cmp al, 0
    jz __7_0
    mov byte [rbp-10], 0
    jmp __7_1
    __7_0:
    mov byte [rbp-10], 1
    __7_1:
    mov rdx, qword [rbp-9]
    mov al, byte [rbp-10]
    mov byte [rdx], al
    mov al, byte [rbp-1]
    mov byte [rbp-12], al
    mov al, byte [rbp-12]
    cmp al, 0
    jz __7_2
    mov rdi, __7_t_fstr
    jmp __7_3
    __7_2:
    mov rdi, __7_f_fstr
    __7_3:
    mov rsi, 0
    mov al, 0
    sub rsp, 12
    extern printf
    call printf
    add rsp, 12
    mov qword [rbp-20], 2
    mov rax, qword [rbp-20]
    leave
    ret


section .data_readonly
    __7_f_fstr db `Boolean: False`,0
    __7_t_fstr db `Boolean: True`,0