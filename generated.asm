	global	_test
_test:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16

	mov	[rbp - 8], esi
	mov	eax, 255
	add	rsp, 16
	pop	rbp
	ret

	global	_main
_main:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16

	mov	esi, 255
	call	_test
	mov	[rbp - 8], eax
	mov	eax, [rbp - 8]
	add	rsp, 16
	pop	rbp
	ret

