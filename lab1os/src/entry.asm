    .section .text.entry
    .globl _start
_start:
    la sp, boot_stark_top
    call rust_main

    .section .bss.stack
    .globl boot_stark_lower_bound
boot_stark_lower_bound:
    .space 4096 * 16
    .globl boot_stark_top
boot_stark_top:
    