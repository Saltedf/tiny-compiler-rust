	.globl main
main:	 
	pushq %rbp
	movq %rsp, %rbp
	subq $16, %rsp
	movq $1, %rcx
	movq $42, -16(%rbp)
	addq $7, %rcx
	movq %rcx, -8(%rbp)
	addq -16(%rbp), %rcx
	negq -8(%rbp)
	addq -8(%rbp), %rcx
	movq %rcx, %rdi
	callq print_int
	addq $16, %rsp
	popq %rbp
	retq


