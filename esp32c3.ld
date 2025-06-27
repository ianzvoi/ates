OUTPUT_ARCH(riscv)
ENTRY(_start)


# MEMORY  MAPPING   CHART
# ------------------------
EXT_DAT =      0x3C000000;
INT_DAT_SRAM = 0x3fc80000;
INT_INS_SRAM = 0x40000000;
EXT_INS =      0x42000000;
INT_DATINS =   0x50000000;
INT_PHERIP =   0x60000000;



SECTIONS
{
    . = EXT_INS;
    start = .;
    .text.magic : {
        KEEP(*(.text.magic))
    }
    .text : {
        stext = .;
        *(.text.entry)
        *(.text .text.*)
        . = ALIGN(4K);
        etext = .;
    }

    .rodata : {
        srodata = .;
        *(.rodata .rodata.*)
        . = ALIGN(4K);
        erodata = .;
    }

    .data : {
        sdata = .;
        *(.data .data.*)
        . = ALIGN(4K);
        edata = .;
    }

    .bss : {
        sbss = .;
        *(.bss .bss.*)
        . = ALIGN(4K);
        ebss = .;
    }
    end = .;
}