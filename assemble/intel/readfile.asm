.intel_syntax noprefix
.global _start

_start:
    mov rsi, 0
    mov rdi, [rsp+16]
    mov rax, 2
    syscall

    mov rdi, rax----
    mov rax, 0------
    mov rsi, rsp----
    mov rdx, 64-----
    syscall

    mov rax, 1
    mov rdi, 1
    syscall

    mov rax, 60
    mov rdi, 42
    syscall

