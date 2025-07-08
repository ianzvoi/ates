OUTPUT_ARCH(riscv)
ENTRY(_start)

_stack_size = 0x1000;
_heap_size = 0x1000;

MEMORY {
  ram   (wxa) : ORIGIN = 0x80000000, LENGTH = 128M
}

PHDRS {
  text PT_LOAD;
  data PT_LOAD;
  bss PT_LOAD;
}


SECTIONS {
  . = ORIGIN(ram);

  .text : {
    *(.text.init)
    *(.text .text.*)
  } >ram AT>ram :text

  _global_pointer = .;

  .rodata : {
    *(.rodata .rodata.*)
  } >ram AT>ram :text

  .data : {
    *(.sdata .sdata.*) *(.data .data.*)
  } >ram AT>ram :data

  .bss :{
    PROVIDE(_bss_start = .);
    *(.sbss .sbss.*) *(.bss .bss.*)
    PROVIDE(_bss_end = .);
  } >ram AT>ram :bss
  . = ALIGN(16);

  . = . + _stack_size;
  PROVIDE(_init_stack_top = .);

  . = ALIGN(16);
  PROVIDE(_init_heap_bottom = .);
  . = . + _heap_size;
  PROVIDE(_init_heap_top = .);

}
