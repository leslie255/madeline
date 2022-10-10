extern	_printf
_fmt:	db 0x25, 0x6C, 0x6C, 0x75, 0x0A, 0x00
_NUMBER:	dq 0
	global	_main
_main:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, 255
	mov	qword [rel _NUMBER], rax
	mov	rax, qword [rel _NUMBER]
	mov	qword [rbp - 8], rax
	xor	eax, eax
	add	rsp, 16
	pop	rbp
	ret
