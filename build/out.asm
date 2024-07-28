    global main

section .text

main:
    push rbp
    mov rbp, rsp
    sub rsp, 25
    call _4
    add rsp, 25
    mov rax, qword [rbp-25]
    mov qword [rbp-9], rax
    mov al, byte [rbp-17]
    mov byte [rbp-1], al
    mov rax, rbp
    add rax, -9
    mov qword [rbp-33], rax
    mov rdx, qword [rbp-33]
    mov al, byte [rdx+0]
    mov byte [rbp-34], al
    mov al, byte [rbp-34]
    cmp al, 0
    jz __23_0
    mov rdi, __23_t_fstr
    jmp __23_1
    __23_0:
    mov rdi, __23_f_fstr
    __23_1:
    mov rsi, 0
    mov al, 0
    sub rsp, 34
    extern printf
    call printf
    add rsp, 34
    mov qword [rbp-42], 0
    mov rax, qword [rbp-42]
    leave
    ret


_4:
    push rbp
    mov rbp, rsp
    mov byte [rbp+16], 1
    mov qword [rbp+17], 0
    leave
    ret


section .data_readonly
    __23_f_fstr db `Boolean: False`,0
    __23_t_fstr db `Boolean: True`,0