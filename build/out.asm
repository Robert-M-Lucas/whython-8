    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 3
    mov qword [rbp-16], 8
    mov rax, qword [rbp-16]
    leave
    ret


