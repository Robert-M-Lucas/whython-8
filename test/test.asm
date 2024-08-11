    global main

section .text

main:
    mov rax, 0xF0000000
    cmp rax, 0
    jmp a
    mov rax, 1
    ret
    a:
    mov rax, 0
    ret

