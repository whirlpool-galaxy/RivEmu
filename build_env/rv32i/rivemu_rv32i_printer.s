.global print

.data
    print_out:
    .word 0x80000000

.text

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