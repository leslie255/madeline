extern	_printf
_message:	db 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x20, 0x00
	global	_say_hello
_say_hello:
	push	rbp
	mov	rbp, rsp

	mov	rdi, _message
	call	_printf

	pop	rbp
	ret
	global	_return_a_number
_return_a_number:
	push	rbp
	mov	rbp, rsp


	mov	rax, 255
	pop	rbp
	ret
	global	_main
_main:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16

	call	_return_a_number
	mov	qword [rbp - 8], rax
_loop_start:
	call	_say_hello
	mov	rax, qword [rbp - 8]
	inc	rax
	mov	qword [rbp - 8], rax
	cmp	qword [rbp - 8], 300
	jne	_loop_start

	xor	eax, eax
	add	rsp, 16
	pop	rbp
	ret
