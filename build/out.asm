    global main

section .text

main:
    push rbp
    mov rbp, rsp
    
    mov qword [rbp-8], 3
    mov rax, qword [rbp-8]
    mov qword [rbp-16], rax
    push 0
    mov rdi, __10_fstr
    mov rsi, [rbp-16]
    mov al, 0
    sub rsp, 16
    extern printf
    call printf
    add rsp, 16
    
    mov rax, qword [rbp-8]
    mov qword [rbp-24], rax
    push 0
    mov rdi, __10_fstr
    mov rsi, [rbp-24]
    mov al, 0
    sub rsp, 24
    extern printf
    call printf
    add rsp, 24
    
    mov rax, qword [rbp-8]
    mov qword [rbp-32], rax
    push 0
    mov rdi, __10_fstr
    mov rsi, [rbp-32]
    mov al, 0
    sub rsp, 32
    extern printf
    call printf
    add rsp, 32
    
    mov rax, qword [rbp-8]
    mov qword [rbp-40], rax
    push 0
    mov rdi, __10_fstr
    mov rsi, [rbp-40]
    mov al, 0
    sub rsp, 40
    extern printf
    call printf
    add rsp, 40
    
    mov rax, qword [rbp-8]
    mov qword [rbp-48], rax
    push 0
    mov rdi, __10_fstr
    mov rsi, [rbp-48]
    mov al, 0
    sub rsp, 48
    extern printf
    call printf
    add rsp, 48
    
    mov rax, qword [rbp-8]
    mov qword [rbp-56], rax
    push 0
    mov rdi, __10_fstr
    mov rsi, [rbp-56]
    mov al, 0
    sub rsp, 56
    extern printf
    call printf
    add rsp, 56
    
    mov rax, qword [rbp-8]
    mov qword [rbp-64], rax
    push 0
    mov rdi, __10_fstr
    mov rsi, [rbp-64]
    mov al, 0
    sub rsp, 64
    extern printf
    call printf
    add rsp, 64
    
    mov rax, qword [rbp-8]
    mov qword [rbp-72], rax
    push 0
    mov rdi, __10_fstr
    mov rsi, [rbp-72]
    mov al, 0
    sub rsp, 72
    extern printf
    call printf
    add rsp, 72
    
    mov rax, qword [rbp-8]
    mov qword [rbp-88], rax
    mov rax, qword [rbp-88]
    mov qword [rbp-104], rax
    sub rsp, 104
    call _2
    add rsp, 104
    mov rax, qword [rbp-96]
    mov qword [rbp-80], rax
	mov rax, qword [rbp-80]
	leave
	ret

_2:
    push rbp
    mov rbp, rsp
    
    mov rax, qword [rbp+16]
    mov qword [rbp-16], rax
    mov rax, qword [rbp-16]
    mov qword [rbp-32], rax
    sub rsp, 32
    call _1
    add rsp, 32
    mov rax, qword [rbp-24]
    mov qword [rbp-8], rax
    mov qword [rbp-32], 1
    mov rax, qword [rbp-8]
    add rax, qword [rbp-32]
    mov qword [rbp+24], rax
    leave
    ret

_1:
    push rbp
    mov rbp, rsp
    
    mov rax, qword [rbp+16]
    mov qword [rbp-8], rax
    mov rax, qword [rbp+16]
    mov qword [rbp-16], rax
    mov rax, qword [rbp-8]
    add rax, qword [rbp-16]
    mov qword [rbp+24], rax
    leave
    ret

section .data
    __10_fstr db `Integer: %d\n`,0