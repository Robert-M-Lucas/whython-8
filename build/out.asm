    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 13
    mov qword [rbp-32], 190
    mov qword [rbp-24], 6
    mov qword [rbp-16], 8
    mov rax, rbp
    add rax, -8
    mov qword [rbp-40], rax
    mov rax, qword [rbp-24]
    mov qword [rbp-48], rax
    mov rdx, qword [rbp-40]
    mov rax, qword [rbp-48]
    mov qword [rdx+0], rax
    mov rax, qword [rbp-8]
    mov qword [rbp-56], rax
    mov rax, qword [rbp-56]
    leave
    ret


