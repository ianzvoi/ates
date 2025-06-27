OUTPUT_ARCH(riscv)
ENTRY(_start)

MEMORY {
    ram   (wxa) : ORIGIN = 0x80000000, LENGTH = 128M
}

PHDRS {
  text PT_LOAD;
  data PT_LOAD;
  bss PT_LOAD;
}


SECTIONS {
  . = ORIGIN(ram); # start at 0x8000_0000

  .text : { # put code first
    *(.text.init) # start with anything in the .text.init section
    *(.text .text.*) # then put anything else in .text
  } >ram AT>ram :text # put this section into the text segment

  PROVIDE(_global_pointer = .); # this is magic, google "linker relaxation"

  .rodata : { # next, read-only data
    *(.rodata .rodata.*)
  } >ram AT>ram :text # goes into the text segment as well (since instructions are generally read-only)

  .data : { # and the data section
    *(.sdata .sdata.*) *(.data .data.*)
  } >ram AT>ram :data # this will go into the data segment

  .bss :{ # finally, the BSS
    PROVIDE(_bss_start = .); # define a variable for the start of this section
    *(.sbss .sbss.*) *(.bss .bss.*)
    PROVIDE(_bss_end = .); # ... and one at the end
  } >ram AT>ram :bss # and this goes into the bss segment
  . = ALIGN(16); # our stack needs to be 16-byte aligned, per the C calling convention
  PROVIDE(_init_stack_top = . + 0x1000); # reserve 0x1000 bytes for the initialisation stack
}
