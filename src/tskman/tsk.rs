use alloc::collections::VecDeque;
use core::arch::{asm, global_asm};
use crate::{power, tskman};
use crate::tskman::tsk;
use crate::uart::Uart;

global_asm!(include_str!("switcher.s"));

#[repr(C)]
pub struct TaskContext {
    reg : [u32; 31],
}




impl TaskContext {
    pub const fn new() -> TaskContext {
        TaskContext {reg: [0; 31]}
    }
}





extern "C" {
    ///  store context only.
    fn _sw_store(store : * mut TaskContext);

    /// this function (kinda of) is CALLED at appropriate position,
    /// only register s0 - s11 is needed to be saved.
    pub fn _switch_forced(store : * mut TaskContext, load : * const TaskContext);
    pub fn _start_tsk(store : * mut TaskContext, new : * const TaskContext);

    // TODO entrance should be saved elsewhere in TCB instead of ra.
    fn _create_tsk(stack : *mut u8, entrance : *mut u8, tsk : *mut TaskContext);
}






pub fn create_task(entrance : fn()) -> TaskContext {

    let mut p : TaskContext = TaskContext::new();
    let mut stack: *mut u8;
    
    unsafe {
        asm!(
            "la {v}, _task1_debug_stack_top",
            v = out(reg) stack
        );
        _create_tsk(stack, entrance as *mut u8, &mut p);
    }
    p
}