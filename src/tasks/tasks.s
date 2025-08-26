    .global _taskman_sync
_taskman_sync:
    lw t0,  132(a0) # load stack top
    sw t0,     (a0) # savw stack top
    lw t0,  128(a0) # load ra
    sw t0,    4(a0) # save ra
    ret



    .global _run
_run:
    lw ra, 128(a0)
    lw sp, 132(a0)
    lw t0, 124(a0)
    csrsi  mstatus, 0x8
    jalr zero,t0,0
    
    
    .global task_exit_handler
task_exit_handler:
    loop:
    J loop
    