
; set up the .text segment for the code
section .text

; global main is the entry point
global main
; note that there is no _ before printf here, unlike in OS X
extern printf

main:
    mov rcx, qword textformat
    movq xmm0, qword [pi]
    mov rax, qword 1      ; need to tell printf how many floats
    call printf

    ; note next step - this puts a zero in rax
    xor rax, rax
    ret ; this returns to the OS based on how Windows calls programs.
    ; this return causes a delay then the program exits.

textformat:
    db "hello, %lf!",0x0a, 0x00     ; friendly greeting


; data segment
section .data

pi dq __float64__(3.14159)

