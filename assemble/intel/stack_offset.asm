.intel_syntax noprefix
.global _start

_start:
        mov rdi, [rsp+128]
        mov rax, 60

        syscall

