    .section .text.it
    .global _it_handler
_it_handler:
    csrrw a0, mscratch, a0
    sw sp ,    (a0)
    sw ra ,   4(a0)

    sw s0 ,   8(a0)
    sw s1 ,  12(a0)
    sw s2 ,  16(a0)
    sw s3 ,  20(a0)
    sw s4 ,  24(a0)
    sw s5 ,  28(a0)
    sw s6 ,  32(a0)
    sw s7 ,  36(a0)
    sw s8 ,  40(a0)
    sw s9 ,  44(a0)
    sw s10,  48(a0)
    sw s11,  52(a0)
    sw gp,   56(a0)
    sw tp,   60(a0)
    sw t0,   64(a0)

    mv t0,   a0
    csrrw a0, mscratch, a0

    sw t1,   68(t0)
    sw t2,   72(t0)
    sw t3,   76(t0)
    sw t4,   80(t0)
    sw t5,   84(t0)
    sw t6,   88(t0)
    sw a0,   92(t0)
    sw a1,   96(t0)
    sw a2,  100(t0)
    sw a3,  104(t0)
    sw a4,  108(t0)
    sw a5,  112(t0)
    sw a6,  116(t0)
    sw a7,  120(t0)



    la sp, _int_handler_stack_top
    call it_handler

# restore_all_register

    csrr a0, mscratch

    lw sp ,    (a0)
    lw ra ,   4(a0)

    lw s0 ,   8(a0)
    lw s1 ,  12(a0)
    lw s2 ,  16(a0)
    lw s3 ,  20(a0)
    lw s4 ,  24(a0)
    lw s5 ,  28(a0)
    lw s6 ,  32(a0)
    lw s7 ,  36(a0)
    lw s8 ,  40(a0)
    lw s9 ,  44(a0)
    lw s10,  48(a0)
    lw s11,  52(a0)

    lw gp,   56(a0)
    lw tp,   60(a0)

    lw t0,   64(a0)
    lw t1,   68(a0)
    lw t2,   72(a0)
    lw t3,   76(a0)
    lw t4,   80(a0)
    lw t5,   84(a0)
    lw t6,   88(a0)

    lw a1,   96(a0)
    lw a2,   100(a0)
    lw a3,   104(a0)
    lw a4,   108(a0)
    lw a5,   112(a0)
    lw a6,   116(a0)
    lw a7,   120(a0)
    lw a0,    92(a0)
    mret