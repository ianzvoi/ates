use core::arch::global_asm;
use crate::tskman;
use crate::tskman::tsk;

global_asm!(include_str!("switcher.s"));

#[repr(C)]
pub struct Tsk {
    sp : u64,
    ra : u64,
    reg : [u64; 12]
}

extern "C" {
    ///  store context only.
    pub fn _sw_store(store : * mut Tsk);

    /// this function (kinda of) is CALLED at appropriate position,
    /// only register s0 - s11 is needed to be saved.
    pub fn _switch_forced(store : * mut Tsk, load : * const Tsk );
}