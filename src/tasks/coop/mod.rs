//! # Cooperative Task Switching Utilities
//! In Cooperative Task Switching mode, Tasks won't be intrrupted by clock.
//! Instead, Task has to yeild their ownership of CPU to other tasks by calling
//! `_switch_forced` or `_yeild`.
//!
//! In this mode, only `S register` and `StackPointer register` is needed to save,
//! becaue `T register` `A register` are caller-save.

use core::arch::global_asm;
use crate::tasks::{Registers, TaskControlBlock};

global_asm!(include_str!("switcher.s"));

extern "C" {
    ///store context.
    fn _sw_store(store : * mut TaskControlBlock);

    /// this function (kinda of) is CALLED at appropriate position,
    /// only register s0 - s11 is needed to be saved.
    pub fn _switch_forced(store : * mut Registers, load : * const Registers);
    pub fn _start_tsk(store : * mut Registers, new : * const Registers) -> !;


    fn _create_tsk(stack : *mut u8, entrance : *mut u8, tsk : *mut Registers);
}

//TODO: inpliment `yeild` function.