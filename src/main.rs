#![feature(ascii_char)]
#![no_std]
#![no_main]

// #[unsafe(link_section = ".magic")]
// static _MAGIC: [u8; 4] = [0xae,0xdb,0x04,0x1d];

mod uart;
mod pm;
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
#[unsafe(naked)]
#[no_mangle]
#[link_section = ".text.init"]
unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "la gp, _global_pointer",
        "la sp, _init_stack_top",
        "J {entry}",
        entry = sym entry, // {entry} refers to the function [entry] below
    )
}



unsafe fn bad_function(x: &mut uart::Tty) {
    static APC: [char; 8] = ['a','b','c','d','e','f','g','h'];
    static mut BUF: usize = 0;

    x.put(APC[BUF] as u8);
    BUF = (BUF + 1) % APC.len();

}
fn entry() -> ! {
    let mut chto = uart::Tty::new(0x1000_0000);
    for a in "Hello, world!\r\n".as_bytes() {
        chto.put(*a);
    }

    for i in 0..128 {
        unsafe { bad_function(&mut chto) };
    }
    
    pm::PM::off();
    loop {}
}
