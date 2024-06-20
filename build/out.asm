    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-9], 2
    mov qword [rbp-17], 3
    mov rax, qword [rbp-17]
    cmp rax, qword [rbp-9]
    jg __5_0
    mov byte [rbp-1], 0
    jmp __5_1
    __5_0:
    mov byte [rbp-1], 1
    __5_1:
    mov al, byte [rbp-1]
    cmp al, 0
    jz __7_2
    mov rdi, __7_t_fstr
    jmp __7_3
    __7_2:
    mov rdi, __7_f_fstr
    __7_3:
    mov rsi, 0
    mov al, 0
    sub rsp, 17
    extern printf
    call printf
    add rsp, 17
    mov qword [rbp-25], 2
    mov rax, qword [rbp-25]
    leave
    ret


section .data_readonly
    __7_f_fstr db `Boolean: False`,0
    __7_t_fstr db `Boolean: True`,0