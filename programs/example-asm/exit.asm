section .text
        global _start

_start:
        ; print msg
        mov rax, 3
        mov rdi, 0
        mov rsi, msg
        mov rdx, len
        int 0x80

        ; exit
        mov rax, 5
        mov rdi, 0
        int 0x80

section .data
        msg db "Hello World from example-asm!", 0xa
        len equ $ -msg
