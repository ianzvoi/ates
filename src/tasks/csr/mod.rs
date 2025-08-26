
pub mod mie {
    pub const MIE_EXT_INT: u32 = 0b100000000000;
    pub const MIE_TIMER_INT: u32 = 0b10000000;
    pub const MIE_SOFT_INT: u32 = 0b1000;


    pub macro csr_mie_set($intr:expr){
        unsafe {
            core::arch::asm!(
            "csrs mie, {}",
            in(reg) ($intr)
            );
        }
    }



    pub macro csr_mie_clear($intr:expr) {
        unsafe {
            core::arch::asm!(
            "csrc mie, {}",
            in(reg) ($intr)
            );
        }
    }


    pub macro csr_mie_write ($intr:expr) {
        unsafe {
            core::arch::asm!(
            "csrw mie, {}",
            in(reg) ($intr)
            );
        }
    }
}