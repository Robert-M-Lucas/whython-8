    global main

section .text

main:
    push rbp
    mov rbp, rsp
    sub rsp, 0x0000000000000010
    call _8
    add rsp, 0x0000000000000010
    mov qword [rbp-24], 0
    mov rax, qword [rbp-24]
    leave
    ret


_8:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 7
    mov rdi, __8_fstr
    mov rsi, [rbp-8]
    mov al, 0
    sub rsp, 8
    extern printf
    call printf
    add rsp, 8

leave
ret

section .data_readonly
    __8_fstr db `Integer: %ld\n`,0