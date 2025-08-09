use core::arch::naked_asm;
use core::ptr::write;

// #[unsafe(naked)]
#[link_section = ".text.it"]
pub unsafe extern "C" fn it_handler() {
   
    crate::uart::Uart::get().write(
        "\nRouting out ;) .\nTrapped noreturn loop in it_handler".as_bytes(), 
        53 
    );
    
    while true {}
    
    // naked_asm!(
    //     "csrrw a0, mscratch, a0",
    //     "sw ra, 0(a0)",
    //     "sw sp, 8(a0)",
    //     "sw gp, 16(a0)",
    //     "sw tp, 24(a0)",
    //     
    //     "sw t0 ,32(a0)",
    //     "sw t1 ,40(a0)",
    //     "sw t2 ,48(a0)",
    //     "sw t3 ,56(a0)",
    //     "sw t4 ,64(a0)",
    //     "sw t5 ,72(a0)",
    //     "sw t6 ,80(a0)",
    // 
    //     "sw s0, 88(a0)",
    //     "sw s1, 96(a0)",
    //     "sw s2, 104(a0)",
    //     "sw s3, 112(a0)",
    //     "sw s4, 120(a0)",
    //     "sw s5, 128(a0)",
    //     "sw s6, 136(a0)",
    //     "sw s7, 144(a0)",
    //     "sw s8, 152(a0)",
    //     "sw s9, 160(a0)",
    //     "sw s10, 168(a0)",
    //     "sw s11, 176(a0)",
    //     
    //     "csrr t0 , mscratch",
    //     "sw t0, 184(a0)",
    //     "sw a1, 192(a0)",
    //     "sw a2, 200(a0)",
    //     "sw a3, 208(a0)",
    //     "sw a4, 216(a0)",
    //     "sw a5, 224(a0)",
    //     "sw a6, 232(a0)",
    //     "sw a7, 240(a0)",
    //     // todo phrase interrupt type
    // )
}