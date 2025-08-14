pub(crate) mod clint;
pub(crate) mod ITH;
pub(crate) mod tsk;


use core::arch::asm;
use crate::uart;




pub fn start_routing() {

    unsafe {
        asm!(
        "la t1, _clinr_mtime",
        "lw t0, (t1)",

        "add t0, t0, {}",

        "la t1, _clint_mtimecmpr",
        "sw t0, (t1)",
        in(reg) 100000
        );
    }

    unsafe {
        asm!(
        "la a0, _it_handler",
        "csrw mtvec, a0",


        "la a0, _tcb_debug_head",
        "csrw  mscratch, a0",

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