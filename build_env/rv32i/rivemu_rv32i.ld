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