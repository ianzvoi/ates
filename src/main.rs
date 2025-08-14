/*GLOBALCONFIGS*/
#![no_main]
#![no_std]
#![test_runner(test)]
#![feature(custom_test_frameworks)]



extern crate alloc;

use alloc::string::ToString;
use crate::uart::Uart;

mod uart;
mod mem;
mod boot;
mod power;
mod tskman;
// mod mem;


#[allow(dead_code)]
/// dummy function, prevent ide error.
fn test(_test: &[&i32]) {
    loop{}
}

fn tmper_log(p : &str){
    uart::Uart::get().write(b"[sync] ",7);
    uart::Uart::get().write(p.as_bytes(), p.len());
    uart::Uart::get().writec('\n' as u8);
}
#[panic_handler]
/// hit when panic happens
fn panic_handler(useful: &core::panic::PanicInfo) -> ! {
    tmper_log(useful.to_string().as_str());
    loop {}
}


fn main() {
    let q  = "Boot";
}
