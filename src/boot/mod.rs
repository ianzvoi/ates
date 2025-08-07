use alloc::string::String;
/** 
* The entry point 
* _start & _start_utils
* are declared here:
*
* _start : 
*    init gp, init sp call _start_utils
*
* _start_utils :
*    init serial port,
*    init allocator,
*    init task manager,
*/



use core::arch::global_asm;
use core::fmt::Write;
use crate::{
    power,
    tskman,
    mem,
    uart,
};
use crate::uart::Uart;

global_asm!(include_str!("boot.s"));

#[no_mangle]
extern "C" fn _start_utils() {

    uart::init();
    let mut dbgr = uart::Uart::get();
    dbgr.write("Serial IO initiated.\n".as_bytes(), 21);

    mem::init();
    dbgr.write("Allocator initiated.\n".as_bytes(), 21);

    demo(&mut dbgr);
    
    tskman::init();
    dbgr.write("TaskManager initiated\n".as_bytes(), 22);
    
    
    power::shutdown();
}

fn demo(port : &mut Uart) {
    use alloc::string;
    let q = string::String::from("----------\nAllocator Test\n----------\n");
    for q in q.chars() {
        port.writec(q as u8);
    }
}

