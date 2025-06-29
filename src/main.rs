#![no_main]
#![no_std]
#![test_runner(test)]
#![feature(custom_test_frameworks)]

mod uart;
mod pm;


#[allow(dead_code)]
/// dummy function, prevent ide error.
fn test(_test: &[&i32]) {
    loop{}
}



#[panic_handler]
/// hit when panic happens
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}




#[unsafe(naked)]
#[no_mangle]
#[link_section = ".text.init"]
/// startup function, .text.init segment.
extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "la gp, _global_pointer",
        "la sp, _init_stack_top",
        "J {entry}",
        entry = sym entry
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

    for _i in 0..128 {
        unsafe { bad_function(&mut chto) };
    }
    
    pm::off();
    loop {}
}
