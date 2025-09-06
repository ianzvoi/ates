//! ## Interrupt Handler
//! interrupt and exceptions will set PC to _it_handler,
//! - `_it_handler` should be registered in mtvec, it will save context and call it_handler
//! - `it_handler` will process the intrrupt type and give respective
//! response to the intrrupt.

use core::arch::{asm, global_asm};
use crate::tasks::{TaskControlBlock};

global_asm!(include_str!("./ITH.s"));



static INT_INTERVAL: usize = 1000;



//! # Explanation:
//! - 1st:higher word 
//! - 2nd:lower word
//! 
//! Always larger if no timer carry-over happens between reading two words.
//! 
//! If carry-over happens, discard the smaller one.
macro_rules! timer_add {

    () => {
        asm!(
        "la t0, _clint_mtime",

        "lw t2, 4(t0)", // high
        "lw t1, (t0)",  // low
        "lw t4, 4(t0)", // high
        "lw t3, (t0)",  // low

        // store the larger.
        "bltu t2, t4, 1f",
        "beq t2, t5, 3f",
        "J 2f",
        "3:",
        "bltu t1, t3, 1f",
        "J 2f",
        "1:",
        "mv t2, t4",
        "mv t1, t3",
        "2:",

        // add( u64 , u32) -> u64
        "add t3, t1, {delta}",
        "bltu t1, t3, 1f",
        "addi t2, t2, 1",
        "1:",

        // store.
        "la t0, _clint_mtimecmpr",
        "sw t3, (t0)",
        "sw t2, 4(t0)",
        delta = in(reg) INT_INTERVAL
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn it_handler() {
    let reason: u32;
    asm!(
        "csrr {reason} ,mcause",
        reason = out(reg) reason,
    );

    if (reason & 0x80000000u32) != 0 {
        // Interrupt
        if reason & 0b111u32 == 0b111u32 {
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
            
            timer_add!();
            return;
        }
    } 
    else {
        if reason == 11u32{
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


                    timer_add!();
                    
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
