//! ## Interrupt Handler
//! interrupt and exceptions will set PC to _it_handler,
//! - `_it_handler` should be registered in mtvec, it will save context and call it_handler
//! - `it_handler` will process the intrrupt type and give respective
//! response to the intrrupt.

use core::arch::{asm, global_asm};
use crate::tasks::{TaskControlBlock};

global_asm!(include_str!("./ITH.s"));



static INT_INTERVAL: usize = 1000;


#[no_mangle]
pub unsafe extern "C" fn it_handler() {
    let reason: u32;
    asm!(
        "csrr {reason} ,mcause",
        reason = out(reg) reason,
    );

    if (reason & 0x80000000u32) != 0 {
        // Interrupt
        if (reason & 0b111u32) == 0b111u32 {

            asm!(
            "csrr t0,  mscratch",
            // save mepc to current TCB;
            "csrr t1,  mepc",
            "sw   t1,  136(t0)",
            // t0 points to next TCB;
            "lw   t0,  140(t0)",
            // load mscratch, mepc form TCB pointed by t0
            "csrw mscratch, t0",
            "lw   t1,  136(t0)",
            "csrw mepc, t1"
            );

            asm!(
            "la t1, _clint_mtime",
            "lw t0, (t1)",
            "add t0, t0, {delta}",
            "la t1, _clint_mtimecmpr",
            "sw t0, (t1)",
            delta = in(reg) INT_INTERVAL
            );

            return;
        }

    } 
    else {
        if(reason == 11u32){
            let choice : u32;
            asm!(
                "csrr t0, mscratch",
                "lw {}, 64(t0)",
                out(reg) choice
            );
            match choice {
                0x00 => { // YIELD.
                    asm!(
                    "csrr t0,  mscratch",

                    // save mepc to current TCB;
                    "csrr t1,  mepc",
                    // jumpover ecall
                    "addi t1, t1,  4",
                    "sw   t1,  136(t0)",

                    // t0 points to next TCB;
                    "lw   t0,  140(t0)",
                    // load mscratch, mepc form TCB pointed by t0
                    "csrw mscratch, t0",
                    "lw   t1,  136(t0)",
                    "csrw mepc, t1"
                    );


                    asm!(
                    "la t1, _clint_mtime",
                    "lw t0, (t1)",
                    "add t0, t0, {delta}",
                    "la t1, _clint_mtimecmpr",
                    "sw t0, (t1)",
                    delta = in(reg) INT_INTERVAL
                    );
                    return;
                }
                0xBEEF => {
                    asm!(
                    "csrr t0,  mscratch",
                    
                    "csrr t1,  mepc",
                    "addi t1, t1,  4",
                    "csrw mepc, t1",
                    
                    "lw  t1, 140(t0) #[t1 <- next tcb]",
                    "lw  t1, 136(t1) #[t1 <- mepc of next tcb]",
                    "sw  t1, 92(t0)  #[a0 of my tcb <- t1]",
                    );
                    return;
                }
                _ => {
                    
                }
            }
        }
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

        //TODO hanle exceptions
        //TIPS: can't use dynamic allocation to debug here.

        loop {}
    }
}
