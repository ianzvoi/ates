pub(crate) mod clint;
pub(crate) mod ith;
pub(crate) mod tsk;


use core::arch::asm;
use crate::uart;




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
    
            "la t0, _tcb_debug_head", // TODO USE DYNAMIC TCB
            "csrw  mscratch, t0",
    
            "li   t0, 0x80",
            "csrs mie, t0",
    
            "csrsi  mstatus, 0x8",
        )
    }
}


pub fn stop_routing() {
    unsafe {
        asm!(
            "csrci mstatus, 0x8",
        );
    }
}