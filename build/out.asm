    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-16], 12
    mov rax, rbp
    add rax, -16
    mov qword [rbp-8], rax
    mov qword [rbp-24], 1
    mov rax, qword [rbp-8]
    mov rdx, qword [rbp-24]
    add qword [rax], rdx
    mov qword [rbp-32], 1
    mov rax, qword [rbp-32]
    leave
    ret


