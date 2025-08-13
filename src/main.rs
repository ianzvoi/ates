/*GLOBALCONFIGS*/
#![no_main]
#![no_std]
#![test_runner(test)]
#![feature(custom_test_frameworks)]



extern crate alloc;

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


#[panic_handler]
/// hit when panic happens
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}


fn main() {
    let q  = "Boot";
}
