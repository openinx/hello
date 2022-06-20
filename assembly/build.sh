#!/bin/bash

programs=(
    "hello.asm"
    "triangle.asm"
)

suffix=".asm"
for prog_file in ${programs[@]} ; do
    prog=${prog_file%$suffix}

    # To assemble the hello.asm file into hello.o using the
    # Mach-O 64bit object file notation format.
    # And to link the hello.o file and turn it into an
    # executable named ‘hello’

    echo "========================================="
    echo "   Assemble the program - $prog_file."

    nasm -f macho64 $prog.asm -o bin/$prog.o && ld -o bin/$prog bin/$prog.o

    if [ $? -ne 0 ]; then
        exit 1
    else
        echo "   - $prog_file is okay."
    fi

    echo "========================================="
    echo
done
