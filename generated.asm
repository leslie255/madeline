        global  _main
_main:
        push    rbp
        mov     rbp, rsp
        sub     rsp, 16
        mov     dword [rbp-8], 42
        mov     eax, dword [rbp-8]
        add     rsp, 16
        pop     rbp
        ret
        ret
