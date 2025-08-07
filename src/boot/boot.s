    .section .text.init
    .global _start
_start:
    la gp, _global_pointer
    la sp, _init_stack_top
    J _start_utils