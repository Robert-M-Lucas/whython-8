    global main

section .text

_10:
    push rbp
    mov rbp, rsp
    mov rax, [rbp+16]
    add rax, 8
    mov qword [rbp-9], rax
    mov rdx, qword [rbp-9]
    mov al, byte [rdx+0]
    mov byte [rbp-1], al
    cmp byte [rbp-1], 0
    jz _10_6
    mov rax, [rbp+16]
    add rax, 9
    mov qword [rbp-17], rax
    mov rdx, qword [rbp-17]
    mov rax, qword [rdx+0]
    mov qword [rbp-25], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-33], rax
    mov rax, qword [rbp-33]
    mov qword [rbp-57], rax
    mov rax, qword [rbp-25]
    mov qword [rbp-65], rax
    sub rsp, 65
    call _10
    add rsp, 65
    jmp _10_5
    _10_6:
    mov rax, [rbp+16]
    add rax, 8
    mov qword [rbp-17], rax
    mov byte [rbp-18], 1
    mov rdx, qword [rbp-17]
    mov al, byte [rbp-18]
    mov byte [rdx+0], al
    mov rax, [rbp+16]
    add rax, 9
    mov qword [rbp-26], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-42], rax
    mov rax, qword [rbp-42]
    mov qword [rbp-58], rax
    sub rsp, 58
    call _9
    add rsp, 58
    mov rax, qword [rbp-50]
    mov qword [rbp-34], rax
    mov rdx, qword [rbp-26]
    mov rax, qword [rbp-34]
    mov qword [rdx+0], rax
    _10_7:
    _10_5:

leave
ret

_6:
    push rbp
    mov rbp, rsp
    mov byte [rbp+16], 0
    mov qword [rbp+17], 0
    leave
    ret


_7:
    push rbp
    mov rbp, rsp
    mov rax, [rbp+16]
    add rax, 0
    mov qword [rbp-9], rax
    mov rdx, qword [rbp-9]
    mov al, byte [rdx+0]
    mov byte [rbp-1], al
    cmp byte [rbp-1], 0
    jz _7_1
    mov rax, [rbp+16]
    add rax, 1
    mov qword [rbp-17], rax
    mov rdx, qword [rbp-17]
    mov rax, qword [rdx+0]
    mov qword [rbp-25], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-33], rax
    mov rax, qword [rbp-33]
    mov qword [rbp-57], rax
    mov rax, qword [rbp-25]
    mov qword [rbp-65], rax
    sub rsp, 65
    call _10
    add rsp, 65
    jmp _7_0
    _7_1:
    mov rax, [rbp+16]
    add rax, 0
    mov qword [rbp-17], rax
    mov byte [rbp-18], 1
    mov rdx, qword [rbp-17]
    mov al, byte [rbp-18]
    mov byte [rdx+0], al
    mov rax, [rbp+16]
    add rax, 1
    mov qword [rbp-26], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-42], rax
    mov rax, qword [rbp-42]
    mov qword [rbp-58], rax
    sub rsp, 58
    call _9
    add rsp, 58
    mov rax, qword [rbp-50]
    mov qword [rbp-34], rax
    mov rdx, qword [rbp-26]
    mov rax, qword [rbp-34]
    mov qword [rdx+0], rax
    _7_2:
    _7_0:

leave
ret

main:
    push rbp
    mov rbp, rsp
    sub rsp, 25
    call _6
    add rsp, 25
    mov rax, qword [rbp-25]
    mov qword [rbp-9], rax
    mov al, byte [rbp-17]
    mov byte [rbp-1], al
    mov rax, rbp
    add rax, -9
    mov qword [rbp-33], rax
    mov qword [rbp-41], 2
    mov rax, qword [rbp-41]
    mov qword [rbp-65], rax
    mov rax, qword [rbp-33]
    mov qword [rbp-73], rax
    sub rsp, 73
    call _7
    add rsp, 73
    mov rax, rbp
    add rax, -9
    mov qword [rbp-65], rax
    mov qword [rbp-73], 3
    mov rax, qword [rbp-73]
    mov qword [rbp-97], rax
    mov rax, qword [rbp-65]
    mov qword [rbp-105], rax
    sub rsp, 105
    call _7
    add rsp, 105
    mov rax, rbp
    add rax, -9
    mov qword [rbp-97], rax
    mov qword [rbp-105], 2
    mov rax, qword [rbp-105]
    mov qword [rbp-129], rax
    mov rax, qword [rbp-97]
    mov qword [rbp-137], rax
    sub rsp, 137
    call _7
    add rsp, 137
    mov rax, rbp
    add rax, -9
    mov qword [rbp-129], rax
    mov qword [rbp-137], 1
    mov rax, qword [rbp-137]
    mov qword [rbp-161], rax
    mov rax, qword [rbp-129]
    mov qword [rbp-169], rax
    sub rsp, 169
    call _7
    add rsp, 169
    mov rax, rbp
    add rax, -9
    mov qword [rbp-161], rax
    mov rax, qword [rbp-161]
    mov qword [rbp-169], rax
    sub rsp, 169
    call _8
    add rsp, 169
    mov qword [rbp-169], 0
    mov rax, qword [rbp-169]
    leave
    ret


_8:
    push rbp
    mov rbp, rsp
    mov rax, [rbp+16]
    add rax, 0
    mov qword [rbp-9], rax
    mov rdx, qword [rbp-9]
    mov al, byte [rdx+0]
    mov byte [rbp-1], al
    cmp byte [rbp-1], 0
    jz _8_3
    mov rax, [rbp+16]
    add rax, 1
    mov qword [rbp-17], rax
    mov rdx, qword [rbp-17]
    mov rax, qword [rdx+0]
    mov qword [rbp-25], rax
    mov rax, qword [rbp-25]
    mov qword [rbp-33], rax
    sub rsp, 33
    call _11
    add rsp, 33
    _8_4:
    _8_3:

leave
ret

_9:
    push rbp
    mov rbp, rsp
    mov rax, qword [rbp+16]
    mov qword [rbp-17], rax
    mov byte [rbp-9], 0
    mov qword [rbp-8], 0
    mov rdi, 17
    sub rsp, 17
    extern malloc
    call malloc
    sub rsp, 17
    mov qword [rbp-25], rax
    mov rdx, qword [rbp-25]
    mov rax, qword [rbp-17]
    mov qword [rdx+0], rax
    mov rax, qword [rbp-9]
    mov qword [rdx+8], rax
    mov al, byte [rbp-1]
    mov byte [rdx+16], al
    mov rax, qword [rbp-25]
    mov qword [rbp+24], rax
    leave
    ret


_11:
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
    mov qword [rbp-25], rax
    mov rdx, qword [rbp-25]
    mov al, byte [rdx+0]
    mov byte [rbp-17], al
    cmp byte [rbp-17], 0
    jz _11_8
    mov rax, [rbp+16]
    add rax, 9
    mov qword [rbp-33], rax
    mov rdx, qword [rbp-33]
    mov rax, qword [rdx+0]
    mov qword [rbp-41], rax
    mov rax, qword [rbp-41]
    mov qword [rbp-49], rax
    sub rsp, 49
    call _11
    add rsp, 49
    _11_9:
    _11_8:

leave
ret

section .data_readonly
    __8_fstr db `Integer: %ld\n`,0