OUTPUT_ARCH(riscv)
ENTRY(_start)

MEMORY {
    EXT_DAT : ORIGIN = 0x3C000000, LENGTH = 8M
    INT_RAM : ORIGIN = 0x3fc80000, LENGTH = 384K
    ROM_DAT : ORIGIN = 0x3ff00000, LENGTH = 128K
    ROM_INS : ORIGIN = 0x40000000, LENGTH = 284K
    INT_INS : ORIGIN = 0x4037C000, LENGTH = 400K
    EXT_INS : ORIGIN = 0x42000000, LENGTH = 8M
    RTC     : ORIGIN = 0x50000000, LENGTH = 8K
    PER     : ORIGIN = 0x60000000, LENGTH = 836K
}

SECTIONS {
    .text : {
        KEEP(*(.magic))
        *(.text)
        *(.text .text.*)
        etext = .;
    } > EXT_INS

    .rodata : {
        srodata = .;
        *(.rodata .rodata.*)
        erodata = .;
    } > EXT_DAT

    .data : {
        sdata = .;
        *(.data .data.*)
        edata = .;
    } > INT_RAM

    .bss : {
        sbss = .;
        *(.bss .bss.*)
        ebss = .;
    } > INT_RAM

    end = .;
}