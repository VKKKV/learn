.intel_syntax noprefix
.global _start

_start:
    mov rax, 0
    mov rdi, 0
    mov rsi, rsp
    mov rdx, 64
    syscall

    mov rax, 1
    mov rdi, 1
    syscall

    mov rax, 60
    mov rdi, 42
    syscall

