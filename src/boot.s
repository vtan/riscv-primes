.section .text.init
.global main

_start:
    la sp, stack_top
    jal ra, main
loop:
    wfi
    j loop
