    .section .text.etry
    .globl _start
_start:
    la sp, book_stark_top
    call rust_main

    .section .bss.stack
    .globl book_stark_lower_bound
book_stark_lower_bound:
    .space 4096 * 16
    .globl book_stark_top
book_stark_top:
    