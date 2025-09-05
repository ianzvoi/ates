//! # The entry point
//!
//! `_start` :
//! init gp, init sp call _start_utils
//!
//! `_start_utils` :
//! init serial port,
//!    init allocator,
//!   init task manager,

use alloc::fmt::format;
use alloc::format;
use alloc::string::{String};
use core::arch::{asm, global_asm};
use core::ptr::write;
use crate::{dev::power, dev::uart, tasks, mem, dev};
use crate::dev::pci::scan_pci_devices;
use crate::dev::power::shutdown;
use crate::dev::uart::Uart;
use crate::dev::vga::VGAScreen;

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

    scan_pci_devices();
    VGAScreen::get().init();

    
    
    
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
    tasks::create_task(vga_screen,stack2);
    tasks::create_task(hjk_task,stack3);

    tasks::start_routing();

    power::shutdown();
}





fn tmper_log(p : &String){

    uart::Uart::get().write(b"[info] ", 7);
    uart::Uart::get().write(p.as_bytes(), p.len());
    uart::Uart::get().writec('\n' as u8);

}


use crate::tasks::locks::ticket::{ticket_lock, ticket_unlock, TicketLock};
use crate::tasks::syscall::oslib::getinst;
use crate::tasks::syscall::time::spin_timer;


fn renderer() {
    
    loop{
        spin_timer(1090025);
        unsafe {
            let time : u32;
            asm!(
                "lw {}, _clint_mtimecmpr",
                out(reg) time
            );
            let fmt_word = format!("{}[?25lThe instruction of next task: {:x}\r",0o33 as char,getinst());
            Uart::get().write(fmt_word.as_bytes(),fmt_word.len());
        }
    }
}



static mut MW : TicketLock = TicketLock{next_ticket : 0, now_serving: 0};
static mut PO : i32 = 12;

fn PO_w(w : i32){
    ticket_lock!(MW);
    unsafe { PO = w; }
    ticket_unlock!(MW);
}

fn PO_r() -> i32 {
    let w : i32;
    ticket_lock!(MW);
    unsafe { w = PO; }
    ticket_unlock!(MW);
    w
}

fn hjk_task() {
    loop {
        let w = Uart::get().readc();
        if w == 'd' as u8 {
            PO_w(PO_r() + 1);
        } else if w == 'a' as u8 {
            PO_w(PO_r() - 1);
        } else if w == 'q' as u8 {
            shutdown();
        }
    }
}

fn vga_screen() {
    const MOTO: &'static str = "Blinky Texts: ";
    for i in 0..MOTO.len() {
        VGAScreen::get().write(i, 0x0600 | ((MOTO.as_bytes()[i] & 0xff) as u8) as u16);
    }
    loop {
        let eo = format!("{}", PO_r());
        // VGAScreen::get().setcursor((eo.len() + 300) as u16);
        for i in 0..eo.len() {
            VGAScreen::get().write(i + 300, 
                                   0x0600 | (eo.as_bytes()[i]  & 0xff) as u16)
        }
    }
}