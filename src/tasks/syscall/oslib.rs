use core::arch::global_asm;

global_asm!(include_str!("oslib.s"));

extern "C" {
    pub fn getinst() -> u32;
}
