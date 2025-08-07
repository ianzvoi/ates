OUTPUT_ARCH(riscv)
ENTRY(_start)



/* devices addresses */
_clint_addr = 0x2000000;
_ns_addr    = 0x10000000;


/* memory constants */
_stack_size = 0x1000;
_heap_size = 0x1000;


MEMORY {
  ram  (wxa) : ORIGIN = 0x80000000, LENGTH = 128M
}


PHDRS {
  text PT_LOAD;
  data PT_LOAD;
  bss  PT_LOAD;
}


SECTIONS {
  . = ORIGIN(ram);


  .text : {
    *(.text.init)

    . = ALIGN(8);
    *(.text.it)

    *(.text .text.*)

  } >ram AT> ram :text


  _global_pointer = .;


  .rodata : {
    *(.rodata .rodata.*)

  } >ram AT> ram :text


  .data : {
    *(.sdata .sdata.*)
    *(.data .data.*)

  } >ram AT> ram :data


  .bss :{
    *(.sbss .sbss.*)
    *(.bss .bss.*)

  } >ram AT> ram :bss



  . = ALIGN(16);
  . = . + _stack_size;
  _init_stack_top = .;


  . = ALIGN(16);
  _init_heap_bottom = .;
  . = . + _heap_size;
  _init_heap_top = .;

}
