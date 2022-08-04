	global	_main
_main:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16

	mov	rax, [rbp - 8]
	dec	rax
	mov	[rbp - 8], rax

	mov	eax, [rbp - 8]
	add	rsp, 16
	pop	rbp
	ret
