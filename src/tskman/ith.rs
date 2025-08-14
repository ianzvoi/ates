use core::arch::{asm, global_asm};
use core::task::Poll::Ready;
use crate::tskman::{tsk, TaskControlBlock, READY};

global_asm!(include_str!("./ith.s"));



static INT_INTERVAL: usize = 1000;

extern "C" {
    fn _swap_context( next : *const TaskControlBlock);
}



static mut curr_id : usize = 0;

#[no_mangle]
pub unsafe extern "C" fn it_handler() {
    let mut reason: u32;
    asm!(
        "csrr {reason} ,mcause",
        reason = out(reg) reason,
    );

    if (reason & 0b10000000000000000000000000000000u32) == 0 {
        // Exception
        let mut mepc: u32;
        let mut mstatus: u32;
        let mut mtval: u32;
        asm!(
            "csrr {mepc}   ,mepc",
            "csrr {mstatus}, mstatus",
            "csrr {mtval}, mtval",
            mtval = out(reg) mtval,
            mepc = out(reg) mepc,
            mstatus = out(reg) mstatus
        );
        // can't use dynamic allocation to debug here.
        loop {}

    } else {
        // Interrupt
        if (reason & 0b111u32) == 0b111u32 {

            asm!(
                "csrr t0,  mscratch",
                "csrr t1,  mepc",
                "sw   t1,  136(t0)",
            
                "lw   t0,  140(t0)",
            
                "lw   t1,  136(t0)",
            
                "csrw mscratch, t0",
                "csrw mepc, t1"
            );
            
            asm!(
                "la t1, _clinr_mtime",
                "lw t0, (t1)",
                "add t0, t0, {delta}",
                "la t1, _clint_mtimecmpr",
                "sw t0, (t1)",
                delta = in(reg) INT_INTERVAL
            );
            
        }
    }
}
