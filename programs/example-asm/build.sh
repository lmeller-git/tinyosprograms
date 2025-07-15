nasm -f elf64 exit.asm -o exit.o
ld -o a.out exit.o
