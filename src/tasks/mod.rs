mod ith;
mod task;
mod coop;
mod itctrl;
pub mod locks;

use alloc::collections::VecDeque;
use core::arch::{asm, global_asm};
use crate::tasks::task::{TaskContext};

#[repr(C)]
struct TaskControlBlock {
    task_context: TaskContext,
    entry : u32,
    /// default to &task_exit_handler
    return_addr : u32,
    stack  : u32,
    pc   : u32,
    next : u32
}


static mut READY : VecDeque<TaskControlBlock> = VecDeque::new();



global_asm!(include_str!("tasks.s"));

extern "C" {
    fn _taskman_sync (target : * mut TaskControlBlock);

    fn _run (target : * const TaskControlBlock) -> !;
    
    fn task_exit_handler();
}



pub fn create_task(entry : fn(), stack : u32) {
    let newtsk = TaskControlBlock{
        task_context : TaskContext::new(),
        entry : entry as u32, //124
        return_addr: task_exit_handler as u32, //128
        stack, // 132
        pc : entry as u32, // 136
        next: 0  // 140
    };

    unsafe{
        READY.push_back(newtsk);
        READY[READY.len()-1].next = &READY[0] as *const TaskControlBlock as u32;
        if(READY.len() > 1) {
            READY[READY.len()-2].next = &READY[READY.len()-1] as *const TaskControlBlock as u32;
        }
        _taskman_sync(READY.back_mut().unwrap());
    }
}



pub fn start_routing() {

    unsafe {
        asm!(
            "la t1, _clinr_mtime",
            "lw t0, (t1)",

            "add t0, t0, {delta_init}",

            "la t1, _clint_mtimecmpr",
            "sw t0, (t1)",
            delta_init = in(reg) 100000
        );
    }

    unsafe {
        asm!(
            "la t0, _it_handler",
            "csrw mtvec, t0",

            "li   t0, 0x80",
            "csrs mie, t0",

            "csrsi  mstatus, 0x8",
        )
    }

    unsafe {
        asm!(
        "csrw mscratch, {}",
        in(reg) &READY[0]
        );
        _run(&READY[0])
    }
}

