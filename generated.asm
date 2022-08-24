	global	_main
_main:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 80


	xor	eax, eax
	add	rsp, 80
	pop	rbp
	ret
