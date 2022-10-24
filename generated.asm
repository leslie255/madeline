extern  _printf
_fmt:   db 0x25, 0x6C, 0x6C, 0x75, 0x0A, 0x00
        global  _main
_main:
        push    rbp
        mov     rbp, rsp
        sub     rsp, 16


        mov     rcx, 0
        cmp     rcx, qword [rbp - 8]


        mov     rcx, 0
        cmp     0, rcx


        mov     rcx, 0
        mov     rdx, 1
        cmp     rcx, rdx


        xor     eax, eax
        add     rsp, 16
        pop     rbp
        ret

