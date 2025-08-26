//! # The entry point
//!
//! `_start` :
//! init gp, init sp call _start_utils
//!
//! `_start_utils` :
//! init serial port,
//!    init allocator,
//!   init task manager,

use alloc::format;
use alloc::string::String;
use core::arch::{asm, global_asm};
use crate::{dev::power, dev::uart, tasks, mem, dev};


global_asm!(include_str!("boot.s"));



//TODO global ?

#[no_mangle]
extern "C" fn _start_utils() -> !{
    uart::init();
    // tmper_log("Serial IO initiated.");

    mem::init();
    // tmper_log("Allocator initiated.");

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

    tasks::create_task(say_hello_tsk,stack1);
    tasks::create_task(say_hello_tsk,stack2);
    tasks::create_task(say_hello_tsk,stack4);
    tasks::create_task(say_hello_tsk,stack3);

    tasks::start_routing();

    power::shutdown();
}


use crate::tasks::locks::naive::{NaiveLock};



crate::tasks::locks::naive::naive!(LOG_LOCK);
fn tmper_log(p : &String){
    tasks::locks::naive::naive_lock!(LOG_LOCK);

    uart::Uart::get().write(b"[info] ", 7);
    uart::Uart::get().write(p.as_bytes(), p.len());
    uart::Uart::get().writec('\n' as u8);

    tasks::locks::naive::naive_unlock!(LOG_LOCK);
}

fn say_hello(recursion : u32) {
    if recursion <= 0 { return; };
    let p = format!("Recusion {}", recursion as usize);
    // for i in 0..1000000 {}
    tmper_log(&p);
    say_hello(recursion - 1);
}

fn say_hello_tsk() {
    say_hello(5);
    
    tmper_log(&format!("time: {}",unsafe{*dev::clint::_get_mtime()}));
    tmper_log(&String::from("Terminated."));
    // power::shutdown();
}