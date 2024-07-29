    global main

section .text

main:
    push rbp
    mov rbp, rsp
    sub rsp, 24
    call _8
    add rsp, 24
    mov rax, qword [rbp-24]
    mov qword [rbp-8], rax
    mov rax, rbp
    add rax, -8
    mov qword [rbp-32], rax
    mov qword [rbp-40], 2
    mov rax, qword [rbp-40]
    mov qword [rbp-64], rax
    mov rax, qword [rbp-32]
    mov qword [rbp-72], rax
    sub rsp, 72
    call _9
    add rsp, 72
    mov rax, rbp
    add rax, -8
    mov qword [rbp-64], rax
    mov qword [rbp-72], 3
    mov rax, qword [rbp-72]
    mov qword [rbp-96], rax
    mov rax, qword [rbp-64]
    mov qword [rbp-104], rax
    sub rsp, 104
    call _9
    add rsp, 104
    mov rax, rbp
    add rax, -8
    mov qword [rbp-96], rax
    mov qword [rbp-104], 2
    mov rax, qword [rbp-104]
    mov qword [rbp-128], rax
    mov rax, qword [rbp-96]
    mov qword [rbp-136], rax
    sub rsp, 136
    call _9
    add rsp, 136
    mov rax, rbp
    add rax, -8
    mov qword [rbp-128], rax
    mov qword [rbp-136], 4
    mov rax, qword [rbp-136]
    mov qword [rbp-160], rax
    mov rax, qword [rbp-128]
    mov qword [rbp-168], rax
    sub rsp, 168
    call _9
    add rsp, 168
    mov rax, rbp
    add rax, -8
    mov qword [rbp-160], rax
    mov qword [rbp-168], 7
    mov rax, qword [rbp-168]
    mov qword [rbp-192], rax
    mov rax, qword [rbp-160]
    mov qword [rbp-200], rax
    sub rsp, 200
    call _9
    add rsp, 200
    mov rax, rbp
    add rax, -8
    mov qword [rbp-192], rax
    mov qword [rbp-200], 8
    mov rax, qword [rbp-200]
    mov qword [rbp-224], rax
    mov rax, qword [rbp-192]
    mov qword [rbp-232], rax
    sub rsp, 232
    call _9
    add rsp, 232
    mov rax, rbp
    add rax, -8
    mov qword [rbp-224], rax
    mov qword [rbp-232], 4
    mov rax, qword [rbp-232]
    mov qword [rbp-256], rax
    mov rax, qword [rbp-224]
    mov qword [rbp-264], rax
    sub rsp, 264
    call _9
    add rsp, 264
    mov rax, rbp
    add rax, -8
    mov qword [rbp-256], rax
    mov rax, qword [rbp-256]
    mov qword [rbp-264], rax
    sub rsp, 264
    call _10
    add rsp, 264
    mov qword [rbp-264], 0
    mov rax, qword [rbp-264]
    leave
    ret


_8:
    push rbp
    mov rbp, rsp
    mov qword [rbp+16], 0
    leave
    ret


_9:
    push rbp
    mov rbp, rsp
    mov rax, [rbp+16]
    add rax, 0
    mov qword [rbp-17], rax
    mov rdx, qword [rbp-17]
    mov rax, qword [rdx+0]
    mov qword [rbp-9], rax
    mov rax, qword [rbp-9]
    cmp rax, 0
    jz __36_0
    mov byte [rbp-1], 0
    jmp __36_1
    __36_0:
    mov byte [rbp-1], 1
    __36_1:
    cmp byte [rbp-1], 0
    jz _9_3
    mov rax, [rbp+16]
    add rax, 0
    mov qword [rbp-25], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-41], rax
    mov rax, qword [rbp-41]
    mov qword [rbp-57], rax
    sub rsp, 57
    call _11
    add rsp, 57
    mov rax, qword [rbp-49]
    mov qword [rbp-33], rax
    mov rdx, qword [rbp-25]
    mov rax, qword [rbp-33]
    mov qword [rdx+0], rax
    jmp _9_2
    _9_3:
    mov rax, [rbp+16]
    add rax, 0
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
    sub rsp, 73
    call _12
    add rsp, 73
    _9_4:
    _9_2:

leave
ret

_10:
    push rbp
    mov rbp, rsp
    mov rax, [rbp+16]
    add rax, 0
    mov qword [rbp-18], rax
    mov rdx, qword [rbp-18]
    mov rax, qword [rdx+0]
    mov qword [rbp-10], rax
    mov rax, qword [rbp-10]
    cmp rax, 0
    jz __36_10
    mov byte [rbp-2], 0
    jmp __36_11
    __36_10:
    mov byte [rbp-2], 1
    __36_11:
    mov al, byte [rbp-2]
    cmp al, 0
    jz __23_12
    mov byte [rbp-1], 0
    jmp __23_13
    __23_12:
    mov byte [rbp-1], 1
    __23_13:
    cmp byte [rbp-1], 0
    jz _10_14
    mov rax, [rbp+16]
    add rax, 0
    mov qword [rbp-26], rax
    mov rdx, qword [rbp-26]
    mov rax, qword [rdx+0]
    mov qword [rbp-34], rax
    mov rax, qword [rbp-34]
    mov qword [rbp-42], rax
    sub rsp, 42
    call _13
    add rsp, 42
    _10_15:
    _10_14:

leave
ret

_11:
    push rbp
    mov rbp, rsp
    mov rax, qword [rbp+16]
    mov qword [rbp-16], rax
    mov qword [rbp-8], 0
    mov rdi, 16
    sub rsp, 16
    extern malloc
    call malloc
    sub rsp, 16
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


_12:
    push rbp
    mov rbp, rsp
    mov rax, [rbp+16]
    add rax, 8
    mov qword [rbp-17], rax
    mov rdx, qword [rbp-17]
    mov rax, qword [rdx+0]
    mov qword [rbp-9], rax
    mov rax, qword [rbp-9]
    cmp rax, 0
    jz __36_5
    mov byte [rbp-1], 0
    jmp __36_6
    __36_5:
    mov byte [rbp-1], 1
    __36_6:
    cmp byte [rbp-1], 0
    jz _12_8
    mov rax, [rbp+16]
    add rax, 8
    mov qword [rbp-25], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-41], rax
    mov rax, qword [rbp-41]
    mov qword [rbp-57], rax
    sub rsp, 57
    call _11
    add rsp, 57
    mov rax, qword [rbp-49]
    mov qword [rbp-33], rax
    mov rdx, qword [rbp-25]
    mov rax, qword [rbp-33]
    mov qword [rdx+0], rax
    jmp _12_7
    _12_8:
    mov rax, [rbp+16]
    add rax, 8
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
    sub rsp, 73
    call _12
    add rsp, 73
    _12_9:
    _12_7:

leave
ret

_13:
    push rbp
    mov rbp, rsp
    mov rax, [rbp+16]
    add rax, 0
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
    add rax, 8
    mov qword [rbp-34], rax
    mov rdx, qword [rbp-34]
    mov rax, qword [rdx+0]
    mov qword [rbp-26], rax
    mov rax, qword [rbp-26]
    cmp rax, 0
    jz __36_16
    mov byte [rbp-18], 0
    jmp __36_17
    __36_16:
    mov byte [rbp-18], 1
    __36_17:
    mov al, byte [rbp-18]
    cmp al, 0
    jz __23_18
    mov byte [rbp-17], 0
    jmp __23_19
    __23_18:
    mov byte [rbp-17], 1
    __23_19:
    cmp byte [rbp-17], 0
    jz _13_20
    mov rax, [rbp+16]
    add rax, 8
    mov qword [rbp-42], rax
    mov rdx, qword [rbp-42]
    mov rax, qword [rdx+0]
    mov qword [rbp-50], rax
    mov rax, qword [rbp-50]
    mov qword [rbp-58], rax
    sub rsp, 58
    call _13
    add rsp, 58
    _13_21:
    _13_20:

leave
ret

section .data_readonly
    __8_fstr db `Integer: %ld\n`,0