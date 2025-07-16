#! /bin/bash

echo "building example-asm in $(pwd)"

nasm -f elf64 exit.asm -o exit.o
ld -o a.out exit.o

echo "example-asm built as a.out"
