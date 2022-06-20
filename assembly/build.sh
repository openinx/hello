#!/bin/bash

# To assemble the .asm file into hello.o using the
# Mach-O 64bit object file notation format

nasm -f macho64 hello.asm -o bin/hello.o

# To link the hello_world.o file and turn it into an
# executable named ‘hello_world’

ld -macosx_version_min 10.7.0 -o bin/hello bin/hello.o
