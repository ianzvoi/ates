/*GLOBALCONFIGS*/
#![no_main]
#![no_std]
#![test_runner(test)]
#![feature(custom_test_frameworks)]
#![feature(decl_macro)]
#![feature(exclusive_wrapper)]
#![feature(generic_atomic)]
extern crate alloc;

use alloc::string::ToString;


mod mem;
mod boot;
mod tasks;
mod dev;
// mod mem;


#[allow(dead_code)]
/// dummy function, prevent ide error.
fn test(_test: &[&i32]) {
    loop{}
}

fn tmper_log(p : &str){
    dev::uart::Uart::get().write(b"[sync] ",7);
    dev::uart::Uart::get().write(p.as_bytes(), p.len());
    dev::uart::Uart::get().writec('\n' as u8);
}
#[panic_handler]
/// hit when panic happens
fn panic_handler(useful: &core::panic::PanicInfo) -> ! {
    tmper_log(useful.to_string().as_str());
    loop {}
}
