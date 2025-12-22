#!/bin/sh

riscv64-unknown-elf-objcopy $1  -O binary out.bin
minichlink -w ./out.bin flash -b 

