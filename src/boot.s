.section .text.init

_start:
    csrr a0, mhartid
    addi t0, a0, 1
    slli t0, t0, 12  # Giving a 2^12 = 4 KiB stack to each hart
    la sp, stack_bottom
    add sp, sp, t0
    addi a1, sp, 0

    bnez a0, secondary

    jal main
loop:
    wfi
    j loop

secondary:
    jal main_secondary
secondary_loop:
    wfi
    j secondary_loop
