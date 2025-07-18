#! /bin/bash

make build

# gcc hello.c -nostdlib -nostartfiles -nodefaultlibs -ffreestanding -static -o hello -s -T linker.ld -no-pie -I. ../libtinyos/target/target/release/liblibtinyos.a /usr/lib/gcc/x86_64-linux-gnu/11/libgcc.a 
