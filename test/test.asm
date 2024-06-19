    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov byte [rbp-1], 0

    mov rax, rbp
    add rax, -1
    mov qword [rbp-9], rax ; rbp - 1
    mov byte [rbp-10], 1

    mov rax, qword [rbp-9] ; rbp - 1
    mov al, byte [rax] ; 0
    or al, byte [rbp-10] ; al = 1
    mov rax, qword [rbp-9] ; rbp - 1
    mov byte [rax], al ;

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
    sub rsp, 11
    extern printf
    call printf
    add rsp, 11
    mov qword [rbp-19], 2
    mov rax, qword [rbp-19]
    leave
    ret


section .data_readonly
    __7_f_fstr db `Boolean: False`,0
    __7_t_fstr db `Boolean: True`,0