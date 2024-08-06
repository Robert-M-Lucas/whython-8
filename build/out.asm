    global main

section .text

main:
    push rbp
    mov rbp, rsp
    sub rsp, 0x0000000000000018
    call _10
    add rsp, 0x0000000000000018
    mov rax, qword [rbp-24]
    mov qword [rbp-8], rax
    mov rax, rbp
    add rax, 0xfffffffffffffff8
    mov qword [rbp-32], rax
    mov qword [rbp-40], 12
    mov rax, qword [rbp-40]
    mov qword [rbp-64], rax
    mov rax, qword [rbp-32]
    mov qword [rbp-72], rax
    sub rsp, 0x0000000000000048
    call _11
    add rsp, 0x0000000000000048
    mov rax, rbp
    add rax, 0xfffffffffffffff8
    mov qword [rbp-64], rax
    mov rax, qword [rbp-64]
    mov qword [rbp-72], rax
    sub rsp, 0x0000000000000048
    call _12
    add rsp, 0x0000000000000048
    mov qword [rbp-72], 8
    mov rax, qword [rbp-72]
    leave
    ret


_10:
    push rbp
    mov rbp, rsp
    mov qword [rbp+16], 0
    leave
    ret


_11:
    push rbp
    mov rbp, rsp
    mov rax, [rbp+16]
    add rax, 0x0000000000000000
    mov qword [rbp-17], rax
    mov rdx, qword [rbp-17]
    mov rax, qword [rdx+0]
    mov qword [rbp-9], rax
    mov rax, qword [rbp-9]
    cmp rax, 0
    jz __36_12
    mov byte [rbp-1], 0
    jmp __36_13
    __36_12:
    mov byte [rbp-1], 1
    __36_13:
    cmp byte [rbp-1], 0
    jz _11_15
    mov rax, [rbp+16]
    add rax, 0x0000000000000000
    mov qword [rbp-25], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-41], rax
    mov rax, qword [rbp-41]
    mov qword [rbp-57], rax
    sub rsp, 0x0000000000000039
    call _13
    add rsp, 0x0000000000000039
    mov rax, qword [rbp-49]
    mov qword [rbp-33], rax
    mov rdx, qword [rbp-25]
    mov rax, qword [rbp-33]
    mov qword [rdx+0], rax
    jmp _11_14
    _11_15:
    mov rax, [rbp+16]
    add rax, 0x0000000000000000
    mov qword [rbp-25], rax
    mov rdx, qword [rbp-25]
    mov rax, qword [rdx+0]
    mov qword [rbp-33], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-41], rax
    mov rax, qword [rbp-41]
    mov qword [rbp-65], rax
    mov rax, qword [rbp-33]
    mov qword [rbp-73], rax
    sub rsp, 0x0000000000000049
    call _14
    add rsp, 0x0000000000000049
    _11_16:
    _11_14:

leave
ret

_12:
    push rbp
    mov rbp, rsp
    mov rax, [rbp+16]
    add rax, 0x0000000000000000
    mov qword [rbp-18], rax
    mov rdx, qword [rbp-18]
    mov rax, qword [rdx+0]
    mov qword [rbp-10], rax
    mov rax, qword [rbp-10]
    cmp rax, 0
    jz __36_0
    mov byte [rbp-2], 0
    jmp __36_1
    __36_0:
    mov byte [rbp-2], 1
    __36_1:
    mov al, byte [rbp-2]
    cmp al, 0
    jz __23_2
    mov byte [rbp-1], 0
    jmp __23_3
    __23_2:
    mov byte [rbp-1], 1
    __23_3:
    cmp byte [rbp-1], 0
    jz _12_4
    mov rax, [rbp+16]
    add rax, 0x0000000000000000
    mov qword [rbp-26], rax
    mov rdx, qword [rbp-26]
    mov rax, qword [rdx+0]
    mov qword [rbp-34], rax
    mov rax, qword [rbp-34]
    mov qword [rbp-42], rax
    sub rsp, 0x000000000000002a
    call _15
    add rsp, 0x000000000000002a
    _12_5:
    _12_4:

leave
ret

_13:
    push rbp
    mov rbp, rsp
    mov rax, qword [rbp+16]
    mov qword [rbp-16], rax
    mov qword [rbp-8], 0
    mov rdi, 16
    sub rsp, 16
    extern malloc
    call malloc
    add rsp, 16
    mov qword [rbp-24], rax
    mov rdx, qword [rbp-24]
    mov rax, qword [rbp-16]
    mov qword [rdx+0], rax
    mov rax, qword [rbp-8]
    mov qword [rdx+8], rax
    mov rax, qword [rbp-24]
    mov qword [rbp+24], rax
    leave
    ret


_14:
    push rbp
    mov rbp, rsp
    mov rax, [rbp+16]
    add rax, 0x0000000000000008
    mov qword [rbp-17], rax
    mov rdx, qword [rbp-17]
    mov rax, qword [rdx+0]
    mov qword [rbp-9], rax
    mov rax, qword [rbp-9]
    cmp rax, 0
    jz __36_17
    mov byte [rbp-1], 0
    jmp __36_18
    __36_17:
    mov byte [rbp-1], 1
    __36_18:
    cmp byte [rbp-1], 0
    jz _14_20
    mov rax, [rbp+16]
    add rax, 0x0000000000000008
    mov qword [rbp-25], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-41], rax
    mov rax, qword [rbp-41]
    mov qword [rbp-57], rax
    sub rsp, 0x0000000000000039
    call _13
    add rsp, 0x0000000000000039
    mov rax, qword [rbp-49]
    mov qword [rbp-33], rax
    mov rdx, qword [rbp-25]
    mov rax, qword [rbp-33]
    mov qword [rdx+0], rax
    jmp _14_19
    _14_20:
    mov rax, [rbp+16]
    add rax, 0x0000000000000008
    mov qword [rbp-25], rax
    mov rdx, qword [rbp-25]
    mov rax, qword [rdx+0]
    mov qword [rbp-33], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-41], rax
    mov rax, qword [rbp-41]
    mov qword [rbp-65], rax
    mov rax, qword [rbp-33]
    mov qword [rbp-73], rax
    sub rsp, 0x0000000000000049
    call _14
    add rsp, 0x0000000000000049
    _14_21:
    _14_19:

leave
ret

_15:
    push rbp
    mov rbp, rsp
    mov rax, [rbp+16]
    add rax, 0x0000000000000000
    mov qword [rbp-16], rax
    mov rdx, qword [rbp-16]
    mov rax, qword [rdx+0]
    mov qword [rbp-8], rax
    mov rdi, __8_fstr
    mov rsi, [rbp-8]
    mov al, 0
    sub rsp, 16
    extern printf
    call printf
    add rsp, 16
    mov rax, [rbp+16]
    add rax, 0x0000000000000008
    mov qword [rbp-34], rax
    mov rdx, qword [rbp-34]
    mov rax, qword [rdx+0]
    mov qword [rbp-26], rax
    mov rax, qword [rbp-26]
    cmp rax, 0
    jz __36_6
    mov byte [rbp-18], 0
    jmp __36_7
    __36_6:
    mov byte [rbp-18], 1
    __36_7:
    mov al, byte [rbp-18]
    cmp al, 0
    jz __23_8
    mov byte [rbp-17], 0
    jmp __23_9
    __23_8:
    mov byte [rbp-17], 1
    __23_9:
    cmp byte [rbp-17], 0
    jz _15_10
    mov rax, [rbp+16]
    add rax, 0x0000000000000008
    mov qword [rbp-42], rax
    mov rdx, qword [rbp-42]
    mov rax, qword [rdx+0]
    mov qword [rbp-50], rax
    mov rax, qword [rbp-50]
    mov qword [rbp-58], rax
    sub rsp, 0x000000000000003a
    call _15
    add rsp, 0x000000000000003a
    _15_11:
    _15_10:

leave
ret

section .data_readonly
    __8_fstr db `Integer: %ld\n`,0