# Riv Emu

Easily extendable library to emulate RISC-V.

- [Git](https://gitlab.com/nerds-forge/rivemu)
- [Issues & Bugs](https://gitlab.com/nerds-forge/rivemu/-/issues/new)

## Features
List of planed and implemented features. And a list of features that are considered stable

### Implementation status

 - [x] rv32i
 - [ ] Zicsr
     - [ ] Machine
     - [ ] Supervisor
     - [ ] User
 - [ ] A
 - [ ] M
 - [ ] F
 - [ ] D

### Release status

 - [ ] rv32i
 - [ ] Zicsr
     - [ ] Machine
     - [ ] Supervisor
     - [ ] User
 - [ ] A
 - [ ] M
 - [ ] F
 - [ ] D

## Installation

    cargo install --git "https://gitlab.com/nerds-forge/rivemu.git" --branch "latest"

## Usage

    run_riv_emu <path/to/memory/image> [load address in hex [number of cycles]]

### Creation of a memory image

    clang --target=riscv32 -march=rv32i -c -o <output/file> <input/file>

    ld.lld -nostdlib --script=<linker/script> -o <output/file> <input/files>+

    llvm-objcopy -O binary <input/file> <output/file>

### Example linker script and startup.s

#### rivemu_rv32i.ld

```
    ENTRY(IRQH_reset)

    MEMORY
    {
        MEM (rwx): org = 0x00000000, len = 1M
    }

    SECTIONS
    {  

        .text : {
            . = 0x00000000;
            *(.irq)
            . = 0x00000040;
            */rivemu_rv32i_startup.o (.text)
            */rivemu_rv32i_startup.o (.data)
            */rivemu_rv32i_startup.o (.bss)
            *(.text)
            *(.data)
            *(.bss)
            *(.text*)
            *(.data*)
            *(.bss*)
        }

    }
```

#### rivemu_rv32i_startup.s

```
    .global IRQH_reset
    .weak IRQH_reset

    .global IRQH_ebreak
    .weak IRQH_ebreak

    .global IRQH_ecall
    .weak IRQH_ecall

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
``` 

## Autors and Copyright
Copyright (C) 2022 Jonathan Schild - All Rights Reserved

 - Jonathan Schild