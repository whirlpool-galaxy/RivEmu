.global IRQH_reset
.weak IRQH_reset

.global IRQH_ebreak
.weak IRQH_ebreak

.global IRQH_m_ecall
.weak IRQH_m_ecall

.global IRQH_s_ecall
.weak IRQH_s_ecall

.global IRQH_u_ecall
.weak IRQH_u_ecall

.global IRQH_illegal_instruction
.weak IRQH_illegal_instruction

.global IRQH_instruction_address_misaligned
.weak IRQH_instruction_address_misaligned

.section .irq
.word IRQH_reset
.word IRQH_ebreak
.word IRQH_m_ecall
.word IRQH_s_ecall
.word IRQH_u_ecall
.word IRQH_illegal_instruction
.word IRQH_instruction_address_misaligned
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
    call main
    j END_OF_CODE

IRQH_ebreak:
    j END_OF_CODE

IRQH_ecall:
    j END_OF_CODE

IRQH_illegal_instruction:
    j END_OF_CODE

IRQH_instruction_address_misaligned:
    j END_OF_CODE

END_OF_CODE:
    j END_OF_CODE