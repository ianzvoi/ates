use core::arch::{global_asm, naked_asm};
use core::ptr::write;


global_asm!(include_str!("./it_handler.s"));
// #[unsafe(naked)]
#[link_section = ".text.it"]
#[no_mangle]
pub unsafe extern "C" fn it_handler() {
    crate::uart::Uart::get().write(
        "\nRouting out ;) .\nNow we're going back\n".as_bytes(), 
        39 
    );
    // loop {}
}