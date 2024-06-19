    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov byte [rbp-1], 1

    mov al, byte [rbp-1]
    cmp al, 0
    jz __7_0
    mov rdi, __7_t_fstr
    jmp __7_1
    __7_0:
    mov rdi, __7_f_fstr
    __7_1:
    mov rsi, 0
    mov al, 0
    sub rsp, 1
    extern printf
    call printf
    add rsp, 1
    mov qword [rbp-9], 2
    mov rax, qword [rbp-9]
    leave
    ret


section .data_readonly
    __7_f_fstr db `Boolean: False`,0
    __7_t_fstr db `Boolean: True`,0