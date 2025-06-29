
use core::arch::naked_asm;
#[allow(dead_code)]
#[unsafe(naked)]
pub extern "C" fn rst() {
    naked_asm!(
        "li t0, 0x100000",
        "li t1, 0x7777",
        "sw t1, 0(t0)"
    )
}
#[allow(dead_code)]
#[unsafe(naked)]
pub extern "C" fn off() {
    naked_asm!(
        "li t0, 0x100000",
        "li t1, 0x5555",
        "sw t1, 0(t0)"
    )
}

#[allow(dead_code)]
#[unsafe(naked)]
pub extern "C" fn fail() {
    naked_asm!(
        "li t0, 0x100000",
        "li t1, 0x3333",
        "sw t1, 0(t0)"
    )
}