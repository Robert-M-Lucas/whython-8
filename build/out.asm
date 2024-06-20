    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-9], 1
    mov qword [rbp-17], 2
    mov rax, qword [rbp-9]
    cmp rax, qword [rbp-17]
    jnz __2_0
    mov byte [rbp-1], 0
    jmp __2_1
    __2_0:
    mov byte [rbp-1], 1
    __2_1:
    mov al, byte [rbp-1]
    cmp al, 0
    jz __23_2
    mov rdi, __23_t_fstr
    jmp __23_3
    __23_2:
    mov rdi, __23_f_fstr
    __23_3:
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
    __23_f_fstr db `Boolean: False`,0
    __23_t_fstr db `Boolean: True`,0