extern  _printf
_fmt:   db 0x25, 0x6C, 0x6C, 0x75, 0x0A, 0x00
        global  _main
_main:
        push    rbp
        mov     rbp, rsp
        sub     rsp, 32
        mov     qword [rbp - 8], 42
        lea     rax, [rbp - 8]
        mov     qword [rbp - 16], rax
        mov     rax, qword [rbp - 16]
        mov     rax, qword [rax]
        mov     qword [rbp - 24], rax
        mov     rdi, _fmt
        mov     rsi, qword [rbp - 24]
        call    _printf
        xor     eax, eax
        add     rsp, 32
        pop     rbp
        ret
