section .text
        global _start

_start:
        ; print msg
        mov rax, 4
        mov rdi, 1
        mov rsi, msg
        mov rdx, len
        int 0x80

        ; exit
        mov rax, 1
        mov rdi, 0
        int 0x80
        
section .data
        msg db "Hello World from example-asm!", 0xa
        len equ $ -msg
