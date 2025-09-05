OUTPUT_ARCH(riscv)
ENTRY(_start)



/* devices addresses */
_clint_addr = 0x2000000;
_clint_mtimecmpr = 0x2004000;
_clint_mtime = 0x200BFF8;

_ns_addr    = 0x10000000;


/* memory constants */
_stack_size = 0x1000;
_heap_size = 0x1000;
_task_stack_size = 0x1000;

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

    . = ALIGN(0x40);
    *(.text.it)
    . = ALIGN(0x10);
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
  . = . + _task_stack_size;
  _int_handler_stack_top = .;
  
  . = ALIGN(16);
  . = . + _stack_size;
  _init_stack_top = .;


  . = ALIGN(16);
  _init_heap_bottom = .;
  . = . + _heap_size;
  _init_heap_top = .;


  . = ALIGN(16);
  . = . + _task_stack_size;
  _task1_debug_stack_top = .;

  . = ALIGN(16);
  . = . + _task_stack_size;
  _task2_debug_stack_top = .;
  
  
  . = ALIGN(16);
  . = . + _task_stack_size;
  _task3_debug_stack_top = .;
  
  . = ALIGN(16);
  . = . + _task_stack_size;
  _task4_debug_stack_top = .;

}
