use core::arch::asm;

pub mod it_types {
    const M_EXT_INT: u32 = 0b100000000000;
    const M_TIMER_INT: u32 = 0b10000000;
    const M_SOFT_INT: u32 = 0b01000;
}

pub fn set_mie(its : u32) {
    unsafe {
        asm!(
        "csrs mie, {}",
        in(reg) its
        );
    }   
}

pub fn clear_mie(its : u32) {
    unsafe {
        asm!(
        "csrc mie, {}",
        in(reg) its
        );
    }
}