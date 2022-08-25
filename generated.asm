extern	_printf
_fmt:	db 0x25, 0x6C, 0x6C, 0x75, 0x0A, 0x00
	global	_main
_main:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16

	mov	qword [rbp - 16], 255
	mov	rdi, _fmt
	mov	rsi, qword [rbp - 16]
	call	_printf

	xor	eax, eax
	add	rsp, 16
	pop	rbp
	ret
