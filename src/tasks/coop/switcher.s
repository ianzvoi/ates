    .section .text




# store context only.
    .global _sw_store
_sw_store:
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
    ret



# this function (kinda of) is CALLED at appropriate position,
# register sp s0 - s11 is needed to be saved,
# t0-t7, a0-a7 is stored by caller.
# ra points to the ret point for current task.

    .global _switch_forced
_switch_forced:
_save_tsk:
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
_load_tsk:
    lw sp ,    (a1)
    lw ra ,   4(a1)
    lw s0 ,   8(a1)
    lw s1 ,  12(a1)
    lw s2 ,  16(a1)
    lw s3 ,  20(a1)
    lw s4 ,  24(a1)
    lw s5 ,  28(a1)
    lw s6 ,  32(a1)
    lw s7 ,  36(a1)
    lw s8 ,  40(a1)
    lw s9 ,  44(a1)
    lw s10,  48(a1)
    lw s11,  52(a1)
    ret





    .global _start_tsk
_start_tsk:
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

    lw sp, (a1)
    la ra, task_exit_handler
    lw t0, 4(a1) #todo INCORRECT
    jalr x0, t0, 0