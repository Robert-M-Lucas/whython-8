    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov byte [rbp-1], 1
    cmp byte [rbp-1], 0
    jz main_0
    main_1:
    main_0:
    mov qword [rbp-9], 0
    mov rax, qword [rbp-9]
    leave
    ret


