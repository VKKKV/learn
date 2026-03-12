.intel_syntax noprefix
.global _start

_start:
    xor rdi, rdi
    cmp qword ptr [rsp], 42
    setz dil

    mov rax, 60
    syscall

