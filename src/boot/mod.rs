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
use alloc::string::{String};
use core::arch::{asm, global_asm};
use crate::{dev::power, dev::uart, tasks, mem, dev};
use crate::dev::power::shutdown;
use crate::dev::uart::Uart;

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

    tasks::create_task(renderer,stack1);
    tasks::create_task(hjk_task,stack3);

    tasks::start_routing();

    power::shutdown();
}


use crate::tasks::locks::naive::{naive_lock, naive_unlock, NaiveLock};



fn tmper_log(p : &String){

    uart::Uart::get().write(b"[info] ", 7);
    uart::Uart::get().write(p.as_bytes(), p.len());
    uart::Uart::get().writec('\n' as u8);

}


use crate::tasks::locks::mutexnaive::{mutexnaive, mutexnaive_lock, mutexnaive_unlock};
use crate::tasks::locks::ticket::{ticket, ticket_lock, ticket_unlock, TicketLock};

static mut MW : TicketLock = TicketLock{next_ticket : 0, now_serving: 0};


static mut PO : i32 = 12;
static MOTS: [&str; 4] = ["howdy","snoop!","Galonbo!","bonjour."];
static mut RENDER_MOT : i32 = 0;
fn renderer() {
    loop{
        unsafe {
            ticket_lock!(MW);
            let cPO = PO;


            let mut s : String = String::from("");
            for i in 0..cPO {
                s.push('.');
            }
            s.push('#');
            for i in cPO..100 {
                s.push('.');
            }

            let wors = format!("{}[?25l[{:3}][{}][{:10}][{:4}]",0o33 as char, cPO,s,MOTS[(RENDER_MOT / 1000) as usize],RENDER_MOT);
            Uart::get().write(wors.as_bytes(), wors.len());

            RENDER_MOT = (RENDER_MOT + 1) % 4000;

            Uart::get().writec('\r' as u8);
            ticket_unlock!(MW);

        }
    }
}

fn hjk_task() {
    loop {
        let w = uart::Uart::get().readc();
        unsafe {
            ticket_lock!(MW);
            
            if (w == 'd' as u8) {

                if (PO < 100) {
                    PO += 1
                }

            } else if( w == 'a' as u8){
                if (PO > 0) {
                    PO -= 1
                }
            } else if( w == 'q' as u8){
                shutdown();
            }
            ticket_unlock!(MW);
        }
    }
}