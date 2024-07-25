    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 3
    mov rax, qword [rbp-8]
    mov qword [rbp-16], rax
    mov rax, qword [rbp-16]
    leave
    ret


