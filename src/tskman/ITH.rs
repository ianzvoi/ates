use alloc::fmt::format;
use alloc::format;
use core::arch::{asm, global_asm, naked_asm};
use core::ptr::write;
use crate::uart;

global_asm!(include_str!("./ITH.s"));



static INT_INTERVAL: usize = 1000;



#[no_mangle]
pub unsafe extern "C" fn it_handler() {
    let mut reason: u32;
    asm!(
        "csrr {reason} ,mcause",
        reason = out(reg) reason,
    );

    if (reason & 0b10000000000000000000000000000000u32) == 0 {
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
        if (reason & 0b111u32) == 0b111u32 {
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
