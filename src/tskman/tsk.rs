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

#[repr(C)]
struct TaskControlBlock {
    task_context: TaskContext,
    
    /// default to &task_exit_handler as *const u8 as u64;
    return_addr : u32,
    stack_base  : u32,
    tid : u32
}


impl TaskContext {
    pub const fn new() -> TaskContext {
        TaskContext {reg: [0; 31]}
    }
}



static READY : VecDeque<TaskControlBlock> = VecDeque::new();
static WAITING : VecDeque<TaskControlBlock> = VecDeque::new();
static BLOCKING : VecDeque<TaskControlBlock> = VecDeque::new();

mod task_manager {
    use crate::tskman::tsk::{TaskContext, TaskControlBlock};

    fn spawn(entry : fn(), stack : u32, tid : u32) {
        let mut newtsk = TaskControlBlock{
            task_context : TaskContext::new(),
            return_addr: 0,
            stack_base: stack,
            tid,
        };
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



#[no_mangle]
fn task_exit_handler() -> ! {
    Uart::get().write(b"PANIC.\n",31);
    power::failure();
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