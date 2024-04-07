mov dword [rbp-44], 0x00
    mov dword [rbp-48], 0x0a666C25
    mov rcx, rbp
    sub rcx, 48
    movq xmm1, qword [rbp-32]
    movq rdx, xmm1  ; duplicate into the integer register
    sub rsp, 40     ; allocate shadow space and alignment (32+8)
    call printf
    add rsp, 40     ; restore stack