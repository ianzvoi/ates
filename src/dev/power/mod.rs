//! # Power management utilities
//! In Qemu, no complicated power management is needed.
//! send 3 magic word to terminate qemu process.
//!
//!  qemu will intercept the write instruction to address 0x100000 and check if it is
//!  one of these magic words:
//!
//!     reset     - 0x7777
//!     power off - 0x5555
//!     fial      - 0x3333  (qemu should quit with none-zero return value.)
//!

use core::arch::global_asm;
global_asm!(include_str!("power.s"));



// extern naked functions
extern "C" {
    fn _machine_reset();
    fn _machine_shutdown();
    fn _machine_fail();
}

pub fn shutdown() -> ! {
    unsafe {
        _machine_shutdown();
    }
    panic!()
}

pub fn reset() -> ! {
    unsafe {
        _machine_reset();
    }
    panic!()
}

pub fn failure() -> ! {
    unsafe {
        _machine_fail();
    }
    panic!()
}


