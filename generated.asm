	global	_main
_main:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16

	mov [rbp - 8], rax 

	mov	eax, [rbp - 8]
	add	rsp, 16
	pop	rbp
	ret
