    global main

    section .text

main:
    push rbp
    mov rbp, rsp
    
    mov qword [rbp-8], -12
    mov rax, qword [rbp-8]
    mov qword [rbp-24], rax
    mov rax, qword [rbp-24]
    mov qword [rbp-40], rax
    sub rsp, 40
    call _1
    add rsp, 40
    mov rax, qword [rbp-32]
    mov qword [rbp-16], rax
	mov rax, qword [rbp-16]
	leave
	ret

_1:
    push rbp
    mov rbp, rsp
    
    mov qword [rbp-8], 0
    mov rax, qword [rbp+16]
    mov qword [rbp-16], rax
    mov rax, qword [rbp-8]
    sub rax, qword [rbp-16]
    mov qword [rbp+24], rax
    leave
    ret

