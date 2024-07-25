    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 3
    mov qword [rbp-24], 12
    mov qword [rbp-16], 13
    mov rax, qword [rbp-8]
    mov qword [rbp-32], rax
    mov rax, qword [rbp-32]
    leave
    ret


