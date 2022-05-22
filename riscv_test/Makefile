all: test.s
	clang --target=riscv32 -march=rv32i test.s -c -o test.o
	ld.lld test.o -o test.bin --oformat binary
	cp test.bin ../