use alloc::fmt::format;
use alloc::format;
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



use core::arch::{global_asm};
use core::fmt::Display;
use crate::{
    power,
    tskman,
    mem,
    uart,
};
use crate::tskman::tsk::{TaskContext, _start_tsk, _switch_forced};

global_asm!(include_str!("boot.s"));



//TODO global ?
static  mut next: TaskContext = tskman::tsk::TaskContext::new();
static  mut sys: TaskContext = tskman::tsk::TaskContext::new();

#[no_mangle]
extern "C" fn _start_utils() {
    uart::init();
    tmper_log("Serial IO initiated.");

    mem::init();
    tmper_log("Allocator initiated.");

    // tmper_log("TaskManager initiated.");

    tskman::clint::timer_set(129800);

    tskman::start_routing();
    for a in 0..0xFF {
        tmper_log("Waiting for Routing out.");
    }
    unsafe {
        next = tskman::tsk::create_task(say_hello_tsk);
    }
    
    tmper_log("sys recursion");
    say_hello(10);

    unsafe {
        tmper_log("tsk recursion");
        _start_tsk(&raw mut sys, &raw const next);
    }

    tmper_log("After all those wonderfully weird things, you are back to sys");
    
    
    power::shutdown();
}




fn tmper_log(p : &str){
    uart::Uart::get().write(b"[info] ",7);
    uart::Uart::get().write(p.as_bytes(), p.len());
    uart::Uart::get().writec('\n' as u8);
}

fn say_hello(recursion : u32) {
    if recursion <= 0 {
        
        return;
    }
    let local_var : u32 = 0;
    let p = format!("{:X}", &local_var as *const u32 as usize);
    tmper_log(p.as_str());
    say_hello(recursion - 1);
}

fn say_hello_tsk() {
    say_hello(12);
    unsafe {
        _switch_forced(&raw mut next, &raw const sys);
    }
}