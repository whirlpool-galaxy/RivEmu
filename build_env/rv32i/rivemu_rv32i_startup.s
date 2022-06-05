.global IRQH_reset
.weak IRQH_reset

.global IRQH_ebreak
.weak IRQH_ebreak

.global IRQH_ecall
.weak IRQH_ecall

.global print

.data
    print_out:
    .word 0x80000000

.section .irq
    .word IRQH_reset
    .word IRQH_ebreak
    .word IRQH_ecall
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0

.text

IRQH_reset:
    j main

IRQH_ebreak:
    j main

IRQH_ecall:
    j main

print:
    // t0 print_out
    // t1 current byte

    lw t0, print_out

loop0:
    lb t1, (a0)
    beqz t1, end0
    sb t1, 2(t0)
    addi a0, a0, 1
    j loop0

end0:
    sb zero, (t0)
    ret

main:
    j main