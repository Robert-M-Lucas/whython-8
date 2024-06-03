    global main

    section .text

main:
    push rbp
    mov  rbp, rsp
    sub  rsp, 0
    mov eax, 12
    leave
    ret