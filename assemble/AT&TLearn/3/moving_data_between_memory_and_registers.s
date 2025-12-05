# moving_data_between_memory_and_registers

.section .data
	constant:
		.int 10

.section .text
	.globl _start

_start:
	nop
mov_immediate_data_between_memory_and_registers:
	movl constant, %ecx

exit:
	movl $1,%eax
	movl $0,%ebx
	int $0x80
