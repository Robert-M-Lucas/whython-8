    global main

section .text

main:
    push rbp
    mov rbp, rsp
    mov dword [rbp-8], 0xb7891800
    mov dword [rbp-4], 0xffffffe8
    mov rdi, __4_fstr
    mov rsi, [rbp-8]
    mov al, 0
    sub rsp, 8
    extern printf
    call printf
    add rsp, 8
    mov dword [rbp-16], 0x00000002
    mov dword [rbp-12], 0x00000000
    mov rax, 60
    mov rdi, [rbp-16]
    syscall
    mov dword [rbp-24], 0x00000000
    mov dword [rbp-20], 0x00000000
    mov dword [rbp-32], 0x00000001
    mov dword [rbp-28], 0x00000000
    mov rax, qword [rbp-32]
    mov qword [rbp-56], rax
    mov rax, qword [rbp-24]
    mov qword [rbp-64], rax
    sub rsp, 64
    call _1
    add rsp, 64
    mov dword [rbp-56], 0x00000001
    mov dword [rbp-52], 0x00000000
    mov rax, qword [rbp-56]
    leave
    ret


_1:
    push rbp
    mov rbp, rsp
    mov rax, qword [rbp+24]
    mov qword [rbp-8], rax
    mov rdi, __4_fstr
    mov rsi, [rbp-8]
    mov al, 0
    sub rsp, 8
    extern printf
    call printf
    add rsp, 8
    mov rax, qword [rbp+16]
    mov qword [rbp-17], rax
    mov dword [rbp-25], 0x01709e79
    mov dword [rbp-21], 0x00000000
    mov rax, qword [rbp-17]
    cmp rax, qword [rbp-25]
    jz __5_0
    mov byte [rbp-9], 0
    jmp __5_1
    __5_0:
    mov byte [rbp-9], 1
    __5_1:
    cmp byte [rbp-9], 0
    jz _1_2
    mov dword [rbp-33], 0x0000000a
    mov dword [rbp-29], 0x00000000
    mov rax, 60
    mov rdi, [rbp-33]
    syscall
    _1_2:
    mov rax, qword [rbp+24]
    mov qword [rbp-33], rax
    mov rax, qword [rbp+16]
    mov qword [rbp-49], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-57], rax
    mov rax, qword [rbp-49]
    add rax, qword [rbp-57]
    mov qword [rbp-41], rax
    mov rax, qword [rbp-33]
    mov qword [rbp-65], rax
    mov rax, qword [rbp-41]
    mov qword [rbp-73], rax
    mov rax, qword [rbp-73]
    mov qword [rbp-97], rax
    mov rax, qword [rbp-65]
    mov qword [rbp-105], rax
    sub rsp, 105
    call _1
    add rsp, 105


section .data_readonly
    __4_fstr db `Integer: %ld\n`,0