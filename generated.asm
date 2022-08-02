	global	_main
_main:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16

	mov	[rbp - 8], 100
	mov	rax, [rbp - 8]
	add	rax, 155
	mov	[rbp - 8], rax
	mov	rax, [rbp - 8]
	mul	10
	mov	[rbp - 8], rax

	xor	eax, eax
	add	rsp, 16
	pop	rbp
	ret
