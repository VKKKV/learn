# indirect_addressing_with_registers: accessing data in mem using pointers

.section .data
	constant:
		.int 5,8,17,44,50,52,60,65,70,77,80

.section .text
	.globl _start

_start:
	nop

indirect_addressing_with_registers:
	movl constant,%eax
	movl $constant,%edi
	movl $25,4(%edi)
	movl $1,%edi
	movl constant(,%edi,4),%ebx

exit:
	movl $1,%eax
	movl $0,%ebx
	int $0x80
