.intel_syntax noprefix
.global _start

_start:
    mov BYTE PTR [rsp], '/'
    mov BYTE PTR [rsp+1], 'f'
    mov BYTE PTR [rsp+2], 'l'
    mov BYTE PTR [rsp+3], 'a'
    mov BYTE PTR [rsp+4], 'g'
    mov BYTE PTR [rsp+5], 0

    mov rsi, 0
    mov rdi, rsp
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

