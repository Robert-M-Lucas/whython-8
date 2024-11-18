    global main

section .text

_14:
    push rbp
    mov rbp, rsp
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000000
    mov qword [rbp-18], rax
    mov rdx, qword [rbp-18]
    mov rax, qword [rdx+0]
    mov qword [rbp-10], rax
    mov rax, qword [rbp-10]
    cmp rax, 0
    jz __36_18
    mov byte [rbp-2], 0
    jmp __36_19
    __36_18:
    mov byte [rbp-2], 1
    __36_19:
    mov al, byte [rbp-2]
    cmp al, 0
    jz __62_20
    mov byte [rbp-1], 0
    jmp __62_21
    __62_20:
    mov byte [rbp-1], 1
    __62_21:
    cmp byte [rbp-1], 0
    jz _14_22
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000000
    mov qword [rbp-26], rax
    mov rdx, qword [rbp-26]
    mov rax, qword [rdx+0]
    mov qword [rbp-34], rax
    mov rax, qword [rbp-34]
    mov qword [rbp-42], rax
    sub rsp, 0x000000000000002a
    call _19
    add rsp, 0x000000000000002a
    _14_23:
    _14_22:

leave
ret

_11:
    push rbp
    mov rbp, rsp
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000000
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
    jz _11_3
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000000
    mov qword [rbp-25], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-41], rax
    mov rax, qword [rbp-41]
    mov qword [rbp-57], rax
    sub rsp, 0x0000000000000039
    call _15
    add rsp, 0x0000000000000039
    mov rax, qword [rbp-49]
    mov qword [rbp-33], rax
    mov rdx, qword [rbp-25]
    mov rax, qword [rbp-33]
    mov qword [rdx+0], rax
    jmp _11_2
    _11_3:
    mov rax, qword [rbp+16]
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
    call _16
    add rsp, 0x0000000000000049
    _11_4:
    _11_2:

leave
ret

_16:
    push rbp
    mov rbp, rsp
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000008
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
    jz _16_8
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000008
    mov qword [rbp-25], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-41], rax
    mov rax, qword [rbp-41]
    mov qword [rbp-57], rax
    sub rsp, 0x0000000000000039
    call _15
    add rsp, 0x0000000000000039
    mov rax, qword [rbp-49]
    mov qword [rbp-33], rax
    mov rdx, qword [rbp-25]
    mov rax, qword [rbp-33]
    mov qword [rdx+0], rax
    jmp _16_7
    _16_8:
    mov rax, qword [rbp+16]
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
    call _16
    add rsp, 0x0000000000000049
    _16_9:
    _16_7:

leave
ret

_18:
    push rbp
    mov rbp, rsp
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000008
    mov qword [rbp-17], rax
    mov rdx, qword [rbp-17]
    mov rax, qword [rdx+0]
    mov qword [rbp-25], rax
    mov rax, qword [rbp-25]
    add rax, 0x0000000000000008
    mov qword [rbp-33], rax
    mov rdx, qword [rbp-33]
    mov rax, qword [rdx+0]
    mov qword [rbp-9], rax
    mov rax, qword [rbp-9]
    cmp rax, 0
    jz __36_14
    mov byte [rbp-1], 0
    jmp __36_15
    __36_14:
    mov byte [rbp-1], 1
    __36_15:
    cmp byte [rbp-1], 0
    jz _18_16
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000008
    mov qword [rbp-49], rax
    mov rdx, qword [rbp-49]
    mov rax, qword [rdx+0]
    mov qword [rbp-57], rax
    mov rax, qword [rbp-57]
    add rax, 0x0000000000000000
    mov qword [rbp-65], rax
    mov rdx, qword [rbp-65]
    mov rax, qword [rdx+0]
    mov qword [rbp-41], rax
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000008
    mov qword [rbp-81], rax
    mov rdx, qword [rbp-81]
    mov rax, qword [rdx+0]
    mov qword [rbp-73], rax
    mov rdi, qword [rbp-73]
    sub rsp, 81
    extern free
    call free
    add rsp, 81
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000008
    mov qword [rbp-89], rax
    mov qword [rbp-97], 0
    mov rdx, qword [rbp-89]
    mov rax, qword [rbp-97]
    mov qword [rdx+0], rax
    mov rax, qword [rbp-41]
    mov qword [rbp+24], rax
    leave
    ret
    _18_17:
    _18_16:
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000008
    mov qword [rbp-41], rax
    mov rdx, qword [rbp-41]
    mov rax, qword [rdx+0]
    mov qword [rbp-49], rax
    mov rax, qword [rbp-49]
    mov qword [rbp-65], rax
    sub rsp, 0x0000000000000041
    call _18
    add rsp, 0x0000000000000041
    mov rax, qword [rbp-57]
    mov qword [rbp+24], rax
    leave
    ret


_19:
    push rbp
    mov rbp, rsp
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000000
    mov qword [rbp-16], rax
    mov rdx, qword [rbp-16]
    mov rax, qword [rdx+0]
    mov qword [rbp-8], rax
    mov rdi, __61_fstr
    mov rsi, [rbp-8]
    mov al, 0
    sub rsp, 16
    extern printf
    call printf
    add rsp, 16
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000008
    mov qword [rbp-34], rax
    mov rdx, qword [rbp-34]
    mov rax, qword [rdx+0]
    mov qword [rbp-26], rax
    mov rax, qword [rbp-26]
    cmp rax, 0
    jz __36_24
    mov byte [rbp-18], 0
    jmp __36_25
    __36_24:
    mov byte [rbp-18], 1
    __36_25:
    mov al, byte [rbp-18]
    cmp al, 0
    jz __62_26
    mov byte [rbp-17], 0
    jmp __62_27
    __62_26:
    mov byte [rbp-17], 1
    __62_27:
    cmp byte [rbp-17], 0
    jz _19_28
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000008
    mov qword [rbp-42], rax
    mov rdx, qword [rbp-42]
    mov rax, qword [rdx+0]
    mov qword [rbp-50], rax
    mov rax, qword [rbp-50]
    mov qword [rbp-58], rax
    sub rsp, 0x000000000000003a
    call _19
    add rsp, 0x000000000000003a
    _19_29:
    _19_28:

leave
ret

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
    mov qword [rbp-40], 1
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
    mov qword [rbp-72], 2
    mov rax, qword [rbp-72]
    mov qword [rbp-96], rax
    mov rax, qword [rbp-64]
    mov qword [rbp-104], rax
    sub rsp, 0x0000000000000068
    call _11
    add rsp, 0x0000000000000068
    mov rax, rbp
    add rax, 0xfffffffffffffff8
    mov qword [rbp-96], rax
    mov qword [rbp-104], 3
    mov rax, qword [rbp-104]
    mov qword [rbp-128], rax
    mov rax, qword [rbp-96]
    mov qword [rbp-136], rax
    sub rsp, 0x0000000000000088
    call _11
    add rsp, 0x0000000000000088
    mov rax, rbp
    add rax, 0xfffffffffffffff8
    mov qword [rbp-128], rax
    mov rax, qword [rbp-128]
    mov qword [rbp-136], rax
    sub rsp, 0x0000000000000088
    call _14
    add rsp, 0x0000000000000088
    mov rdi, __42_fstr
    mov al, 0
    sub rsp, 128
    extern printf
    call printf
    add rsp, 128
    mov rax, rbp
    add rax, 0xfffffffffffffff8
    mov qword [rbp-144], rax
    mov rax, qword [rbp-144]
    mov qword [rbp-160], rax
    sub rsp, 0x00000000000000a0
    call _13
    add rsp, 0x00000000000000a0
    mov rax, qword [rbp-152]
    mov qword [rbp-136], rax
    mov rax, rbp
    add rax, 0xfffffffffffffff8
    mov qword [rbp-160], rax
    mov qword [rbp-168], 4
    mov rax, qword [rbp-168]
    mov qword [rbp-192], rax
    mov rax, qword [rbp-160]
    mov qword [rbp-200], rax
    sub rsp, 0x00000000000000c8
    call _11
    add rsp, 0x00000000000000c8
    mov rax, rbp
    add rax, 0xfffffffffffffff8
    mov qword [rbp-192], rax
    mov rax, qword [rbp-192]
    mov qword [rbp-200], rax
    sub rsp, 0x00000000000000c8
    call _14
    add rsp, 0x00000000000000c8
    mov qword [rbp-200], 0
    mov rax, qword [rbp-200]
    leave
    ret


_13:
    push rbp
    mov rbp, rsp
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000000
    mov qword [rbp-17], rax
    mov rdx, qword [rbp-17]
    mov rax, qword [rdx+0]
    mov qword [rbp-25], rax
    mov rax, qword [rbp-25]
    add rax, 0x0000000000000008
    mov qword [rbp-33], rax
    mov rdx, qword [rbp-33]
    mov rax, qword [rdx+0]
    mov qword [rbp-9], rax
    mov rax, qword [rbp-9]
    cmp rax, 0
    jz __36_10
    mov byte [rbp-1], 0
    jmp __36_11
    __36_10:
    mov byte [rbp-1], 1
    __36_11:
    cmp byte [rbp-1], 0
    jz _13_12
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000000
    mov qword [rbp-49], rax
    mov rdx, qword [rbp-49]
    mov rax, qword [rdx+0]
    mov qword [rbp-57], rax
    mov rax, qword [rbp-57]
    add rax, 0x0000000000000000
    mov qword [rbp-65], rax
    mov rdx, qword [rbp-65]
    mov rax, qword [rdx+0]
    mov qword [rbp-41], rax
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000000
    mov qword [rbp-81], rax
    mov rdx, qword [rbp-81]
    mov rax, qword [rdx+0]
    mov qword [rbp-73], rax
    mov rdi, qword [rbp-73]
    sub rsp, 81
    extern free
    call free
    add rsp, 81
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000000
    mov qword [rbp-89], rax
    mov qword [rbp-97], 0
    mov rdx, qword [rbp-89]
    mov rax, qword [rbp-97]
    mov qword [rdx+0], rax
    mov rax, qword [rbp-41]
    mov qword [rbp+24], rax
    leave
    ret
    _13_13:
    _13_12:
    mov rax, qword [rbp+16]
    add rax, 0x0000000000000000
    mov qword [rbp-41], rax
    mov rdx, qword [rbp-41]
    mov rax, qword [rdx+0]
    mov qword [rbp-49], rax
    mov rax, qword [rbp-49]
    mov qword [rbp-65], rax
    sub rsp, 0x0000000000000041
    call _18
    add rsp, 0x0000000000000041
    mov rax, qword [rbp-57]
    mov qword [rbp+24], rax
    leave
    ret


_10:
    push rbp
    mov rbp, rsp
    mov qword [rbp+16], 0
    leave
    ret


_15:
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


section .data_readonly
    __42_fstr db `\n`,0
    __61_fstr db `Integer: %ld\n`,0