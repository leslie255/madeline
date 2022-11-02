        global  _main
_main:
        push    rbp
        mov     rbp, rsp
        sub     rsp, 16
        mov     qword [rbp-8], 42
        xor     rax, rax
        add     rsp, 16
        pop     rbp
        ret
