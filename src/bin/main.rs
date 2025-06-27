#![no_std]
#![no_main]
use core::panic::PanicInfo;


#[unsafe(link_section = ".text.magic")]
static VARI: [u8; 4] = [0xae,0xdb,0x04,0x1d];

#[no_mangle]
fn entry() -> u8{
    VARI[1]
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}