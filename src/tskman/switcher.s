    .section .text




# store context only.
    .global _sw_store
_sw_store:
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
    ret



# this function (kinda of) is CALLED at appropriate position,
# only register s0 - s11 is needed to be saved.
    .global _switch_forced
_switch_forced:
_save_tsk:
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
_load_tsk:
    lw sp ,    (a1)
    lw ra ,   8(a1)
	lw s0 ,  16(a1)
	lw s1 ,  24(a1)
	lw s2 ,  32(a1)
	lw s3 ,  40(a1)
	lw s4 ,  48(a1)
	lw s5 ,  56(a1)
	lw s6 ,  64(a1)
	lw s7 ,  72(a1)
	lw s8 ,  80(a1)
	lw s9 ,  88(a1)
	lw s10,  96(a1)
	lw s11, 104(a1)
    ret

#create a task with a task stack
    .global _create_tsk
_create_tsk:
    sw a0, (a2)
    sw a1, 8(a2) #todo INCORRECT
    ret


    .global _start_tsk
_start_tsk:
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

    lw sp, (a1)
    la ra, task_exit_handler
    lw t0, 8(a1)
    jalr x0, t0, 0