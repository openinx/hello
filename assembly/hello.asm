global    start                    ; Must be declared for linker (ld)

section   .text                    ; Section Text.

start:
    mov       rax, 0x02000004
    mov       rdi, 1
    mov       rsi, message
    mov       rdx, 13
    syscall
    mov       rax, 0x02000001
    xor       rdi, rdi
    syscall


section   .data                    ; Section Data.
message:
    db        "Hello, World", 10   ; string to be printed
