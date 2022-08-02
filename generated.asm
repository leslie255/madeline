	global	_main
_main:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16

	mov	[rbp - 8], 0
_test:
	mov	rax, [rbp - 8]
	add	rax, 1
	mov	[rbp - 8], rax
	jmp	_test

	xor	rax, rax
	add	rsp, 16
	pop	rbp
	ret
