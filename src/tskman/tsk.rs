use core::arch::{asm, global_asm};
use crate::{power, tskman};
use crate::tskman::tsk;
use crate::uart::Uart;

global_asm!(include_str!("switcher.s"));

#[repr(C)]
pub struct Tsk {
    sp : u64,
    pc : u64,
    reg : [u64; 12]
}
impl Tsk {
    pub const fn new() -> Tsk {
        Tsk{sp:0, pc:0, reg: [0; 12]}
    }
}
extern "C" {
    ///  store context only.
    pub fn _sw_store(store : * mut Tsk);

    /// this function (kinda of) is CALLED at appropriate position,
    /// only register s0 - s11 is needed to be saved.
    pub fn _switch_forced(store : * mut Tsk, load : * const Tsk );
    pub fn _start_tsk(store : * mut Tsk, new : * const Tsk);

    // TODO entrance should be saved elsewhere in TCB instead of ra.
    pub fn _create_tsk(stack : *mut u8, entrance : *mut u8, tsk : *mut Tsk);


}

#[no_mangle]
pub fn task_exit_handler() -> ! {
    Uart::get().write(b"You can't return a tsk. PANIC.\n",31);
    power::failure();
}


pub fn create_task(entrance : fn()) -> Tsk {
    let mut p : Tsk = Tsk{
        sp:0,
        pc:0,
        reg : [0; 12]
    };
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