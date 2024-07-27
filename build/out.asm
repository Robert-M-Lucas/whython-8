    global main

section .text

main:
    push rbp
    mov rbp, rsp
    ; Heap Alloc
    mov qword [rbp-24], 3
    mov qword [rbp-16], 4
    mov rdi, 16
    sub rsp, 24
    extern malloc
    call malloc
    sub rsp, 24
    mov qword [rbp-32], rax
    mov rdx, qword [rbp-32]
    mov rax, qword [rbp-24]
    mov qword [rdx+0], rax
    mov rax, qword [rbp-16]
    mov qword [rdx+8], rax
    mov rax, qword [rbp-32]
    mov qword [rbp-8], rax
    ; Unheap
    mov rdx, qword [rbp-8]
    mov rax, qword [rdx+0]
    mov qword [rbp-48], rax
    mov rax, qword [rdx+8]
    mov qword [rbp-40], rax
    ; Get member
    mov rax, rbp
    add rax, -48
    mov qword [rbp-56], rax
    ; Deref member
    mov rdx, qword [rbp-56]
    mov rax, qword [rdx+0]
    mov qword [rbp-64], rax
    ; Print
    mov rax, qword [rbp-64]
    mov qword [rbp-72], rax
    mov rdi, __8_fstr
    mov rsi, [rbp-72]
    mov al, 0
    sub rsp, 72
    extern printf
    call printf
    add rsp, 72
    ; Return
    mov qword [rbp-80], 7
    mov rax, qword [rbp-80]
    leave
    ret


section .data_readonly
    __8_fstr db `Integer: %ld\n`,0