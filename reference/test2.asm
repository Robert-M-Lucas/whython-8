global main
extern GetStdHandle
extern WriteFile

section .text
main:
    sub     rsp, 40          ; reserve shadow spaceand align the stack by 16
    mov     ecx, -11         ; GetStdHandle takes a DWORD arg, write it as 32-bit.  This is STD_OUTPUT_HANDLE
    call    GetStdHandle

    mov     rcx, rax
    mov     rdx, NtlpBuffer         ; or better, lea rdx, [rel NtlpBuffer]
    mov     r8, [NtnNBytesToWrite]  ; or better, make it an EQU constant for mov r8d, bytes_to_write
    mov     r9, NtlpNBytesWritten   ; first 4 args in regs
    mov     qword [rsp + 32], 00h   ; fifth arg on the stack above the shadow space.  Also, this is a pointer so it needs to be a qword store.
    call    WriteFile
    add     rsp, 40
ExitProgram:
    xor     eax, eax
    ret

section .data
NtlpBuffer:        db    'Hello, World!', 00h
NtnNBytesToWrite:  dq    0eh

section .bss
NtlpNBytesWritten: resd  01h