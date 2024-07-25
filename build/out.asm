    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov qword [rbp-8], 13
    ; Test Start
    mov qword [rbp-32], 3
    mov qword [rbp-24], 5
    mov qword [rbp-16], 4
    ; Test End
    mov rax, qword [rbp-8]
    mov qword [rbp-40], rax
    mov rax, qword [rbp-40]
    leave
    ret


