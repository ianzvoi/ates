/*GLOBALCONFIGS*/
#![no_main]
#![no_std]
#![test_runner(test)]
#![feature(custom_test_frameworks)]


/*MODS*/
extern crate alloc;

use alloc::string::String;

mod uart;
mod pm;
mod tmp_mem;
// mod mem;


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
        entry = sym entry,
    )
}





fn bad_function(x: &mut uart::Tty) {
    static APC: [&str; 7] = [
        "Hi I am a progreemu. ",
        "this is my frieend, qemu.",
        "i am ","a ","messy ","sentence. ",
        "Isn't it? "];
    
    
    
    unsafe {
        static mut BUF: usize = 0;
        for words in APC[BUF % APC.len()].as_bytes(){
            x.put(*words);
        }
        BUF = (5*BUF + 3) % 12332312;
    }


    let a = String::from("word in dynamic space");

    for i in a.as_bytes(){
        x.put(*i);
    }
}



#[allow(dead_code)]
fn entry() -> ! {
    tmp_mem::init();
    
    let mut chto = uart::Tty::new(0x1000_0000);
    for a in "Hello, world!\r\n".as_bytes() {
        chto.put(*a);
    }

    for _i in 0..14 {
        bad_function(&mut chto);
    }
    
    pm::off();
    loop {}
}
