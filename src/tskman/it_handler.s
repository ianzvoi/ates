    .section .text.it
    .global _it_handler
_it_handler:
    csrrw a0, mscratch, a0

    sw sp ,    (a0)
    sw ra ,   8(a0)

    sw s0 ,  16(a0)
    sw s1 ,  24(a0)
    sw s2 ,  32(a0)
    sw s3 ,  40(a0)
    sw s4 ,  48(a0)
    sw s5 ,  56(a0)
    sw s6 ,  64(a0)
    sw s7 ,  72(a0)
    sw s8 ,  80(a0)
    sw s9 ,  88(a0)
    sw s10,  96(a0)
    sw s11, 104(a0)

    sw gp,  112(a0)
    sw tp,  120(a0)

    sw t0,  128(a0)
    sw t1,  136(a0)
    sw t2,  144(a0)
    sw t3,  152(a0)
    sw t4,  160(a0)
    sw t5,  168(a0)
    sw t6,  176(a0)

    mv a0, t0
    csrrw a0, mscratch, a0
    sw a0,  184(t0)
    csrrw a0, mscratch, a0

    sw a1,  192(a0)
    sw a2,  200(a0)
    sw a3,  208(a0)
    sw a4,  216(a0)
    sw a5,  224(a0)
    sw a6,  232(a0)
    sw a7,  240(a0)

    csrrw a0, mscratch, a0

_break_001:
    call it_handler

# restore_all_register
    csrrw a0, mscratch, a0

    lw sp ,    (a0)
    lw ra ,   8(a0)

    lw s0 ,  16(a0)
    lw s1 ,  24(a0)
    lw s2 ,  32(a0)
    lw s3 ,  40(a0)
    lw s4 ,  48(a0)
    lw s5 ,  56(a0)
    lw s6 ,  64(a0)
    lw s7 ,  72(a0)
    lw s8 ,  80(a0)
    lw s9 ,  88(a0)
    lw s10,  96(a0)
    lw s11, 104(a0)

    lw gp,  112(a0)
    lw tp,  120(a0)

    lw t0,  128(a0)
    lw t1,  136(a0)
    lw t2,  144(a0)
    lw t3,  152(a0)
    lw t4,  160(a0)
    lw t5,  168(a0)
    lw t6,  176(a0)

    mv a0, t0
    csrrw a0, mscratch, a0
    lw a0,  184(t0)
    csrrw a0, mscratch, a0

    lw a1,  192(a0)
    lw a2,  200(a0)
    lw a3,  208(a0)
    lw a4,  216(a0)
    lw a5,  224(a0)
    lw a6,  232(a0)
    lw a7,  240(a0)
    csrrw a0, mscratch, a0

    mret
