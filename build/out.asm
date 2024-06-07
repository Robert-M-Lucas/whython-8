    global main

    section .text

main:
    push rbp
    mov rbp, rsp
    
    mov qword [rbp-16], 3
    mov qword [rbp-24], 2
    mov rax, qword [rbp-16]
    mov qword [rbp-48], rax
    mov rax, qword [rbp-24]
    mov qword [rbp-56], rax
    sub rsp, 56
    call _1
    add rsp, 56
    mov rax, qword [rbp-40]
    mov qword [rbp-8], rax
	mov rax, qword [rbp-8]
	leave
	ret

_1:
    push rbp
    mov rbp, rsp
    
    mov rax, qword [rbp+16]
    mov qword [rbp-8], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-16], rax
    mov rax, qword [rbp-8]
    mov qword [rbp-40], rax
    mov rax, qword [rbp-16]
    mov qword [rbp-48], rax
    sub rsp, 48
    call _2
    add rsp, 48
    mov rax, qword [rbp-32]
    mov qword [rbp+32], rax
    leave
    ret

_2:
    push rbp
    mov rbp, rsp
    
    mov rax, qword [rbp+16]
    mov qword [rbp-16], rax
    mov rax, qword [rbp+24]
    mov qword [rbp-24], rax
    mov rax, qword [rbp-16]
    add rax, qword [rbp-24]
    mov qword [rbp-8], rax
    mov qword [rbp-32], 1
    mov rax, qword [rbp-8]
    add rax, qword [rbp-32]
    mov qword [rbp+32], rax
    leave
    ret

