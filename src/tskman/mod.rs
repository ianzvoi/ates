pub(crate) mod clint;
mod it;
pub(crate) mod tsk;


use core::arch::asm;
use crate::uart;




pub fn start_routing() {
    let mut a = 0;
    
    unsafe {
        asm!(
        "la a0, {itr}",
        "andi a0, a0, 0xFFFFFFFC",
        "csrw mtvec, a0",

        "csrsi  mstatus, 8",
        
        "la a0, 0x80050000",
        "csrw  mscratch, a0",
        "mv a0, {o}",

        "li  a0, 2176",
        "csrs mie, a0",

        itr = sym it::it_handler,
        o = out(reg) a,
        )
    }
    
    if (a > 0){
        uart::Uart::get().writec('s' as u8);
    }

    unsafe{ 
        //  clint::timer_start();
    }
}