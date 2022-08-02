CC				= clang
LD				= ld.lld
OC				= llvm-objcopy

CFLAGS			= --target=riscv32 -march=rv32i -c
LFLAGS			= -nostdlib -Map $(OUTPUT).elf.map --script=$(LSCRIPT)
OFLAGS			= -O binary

CSRC			= $(wildcard *.c)
SSRC			= $(wildcard *.s)

COBJ			= $(addprefix $(BUILDDIR)/, $(CSRC:.c=.c.o))
SOBJ			= $(addprefix $(BUILDDIR)/, $(SSRC:.s=.s.o))

BUILDDIR		= build

LSCRIPT			= rivemu_rv32i.ld

OUTPUT			= rivemu_rv32i


all: setup $(OUTPUT)

setup:
	mkdir -p $(BUILDDIR)

$(OUTPUT): $(OUTPUT).elf
	$(OC) $(OFLAGS) $< $@

$(OUTPUT).elf: $(COBJ) $(SOBJ)
	$(LD) $(LFLAGS) -o $(OUTPUT).elf $(COBJ) $(SOBJ)

$(BUILDDIR)/%.c.o: %.c
	$(CC) $(CFLAGS) -o $@ $<

$(BUILDDIR)/%.s.o: %.s
	$(CC) $(CFLAGS) -o $@ $<

clean:
	rm -r $(BUILDDIR)

distclean: clean
	rm -f $(OUTPUT) $(OUTPUT).elf $(OUTPUT).elf.map

.PHONY: clean distclean