.global IRQ_reset
.weak IRQ_reset

.global IRQ_ebreak
.weak IRQ_ebreak

.global IRQ_m_ecall
.weak IRQ_m_ecall

.global IRQ_s_ecall
.weak IRQ_s_ecall

.global IRQ_u_ecall
.weak IRQ_u_ecall

.global IRQ_illegal_instruction
.weak IRQ_illegal_instruction

.global print

.data
    print_out:
    .word 0x80000000

.section .irq
    .word IRQ_reset
    .word IRQ_ebreak
    .word IRQ_m_ecall
    .word IRQ_s_ecall
    .word IRQ_u_ecall
    .word IRQ_illegal_instruction
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

IRQ_reset:
IRQ_ebreak:
IRQ_m_ecall:
IRQ_s_ecall:
IRQ_u_ecall:
IRQ_illegal_instruction:
    j LOOP

LOOP:
    j LOOP


print:
    // t0 print_out
    // t1 current byte

    lw t0, print_out

loop:
    lb t1, (a0)
    beqz t1, end
    sb t1, 2(t0)
    addi a0, a0, 1
    j loop

end:
    sb zero, (t0)
    ret