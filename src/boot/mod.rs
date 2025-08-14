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



use core::arch::{asm, global_asm};
use core::fmt::Display;
use crate::{
    power,
    tskman,
    mem,
    uart,
};


global_asm!(include_str!("boot.s"));



//TODO global ?

#[no_mangle]
extern "C" fn _start_utils() -> !{
    uart::init();
    tmper_log("Serial IO initiated.");

    mem::init();
    tmper_log("Allocator initiated.");

    // tmper_log("TaskManager initiated.");

    let stack1 : u32;
    let stack2 : u32;
    let stack3 : u32;
    let stack4 : u32;

    unsafe {
        asm!(
            "la {}, _task1_debug_stack_top",
            "la {}, _task2_debug_stack_top",
            "la {}, _task3_debug_stack_top",
            "la {}, _task4_debug_stack_top",

            out(reg) stack1,
            out(reg) stack2,
            out(reg) stack3,
            out(reg) stack4,
        )
    }

    tskman::create_task(say_hello_tsk,stack1,12);
    tskman::create_task(say_hello_tsk,stack2,12);
    tskman::create_task(say_hello_tsk,stack4,12);
    tskman::create_task(say_hello_tsk,stack3,12);

    tskman::start_routing();
    
    // for i in 0..1000 {
    //     say_hello(12);
    // }
    // tskman::run(0);


    power::shutdown();
}




fn tmper_log(p : &str){
    uart::Uart::get().write(b"[info] ",7);
    uart::Uart::get().write(p.as_bytes(), p.len());
    uart::Uart::get().writec('\n' as u8);
}

fn say_hello(recursion : u32) {
    if recursion <= 0 { return; };
    let p = format!("Recusion {}", recursion as usize);
    // for i in 0..1000000 {}
    tmper_log(p.as_str());
    say_hello(recursion - 1);
}

fn say_hello_tsk() {
    say_hello(5);
    
    tmper_log(format!("time: {}",unsafe{*tskman::clint::_get_mtime()}).as_str());
    tmper_log("Terminated.");
    // power::shutdown();
}