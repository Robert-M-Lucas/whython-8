    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 1
    mov rax, qword [rbp-8]
    leave
    ret


