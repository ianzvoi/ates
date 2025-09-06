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

    tasks::create_task(command_getter, stack1);
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




type PoType = u32;
static mut MW : TicketLock = TicketLock{next_ticket : 0, now_serving: 0};
static mut PO : PoType = 12;

fn PO_w(w : PoType){
    ticket_lock!(MW);
    unsafe { PO = w; }
    ticket_unlock!(MW);
}

fn PO_r() -> PoType {
    let w : PoType;
    ticket_lock!(MW);
    unsafe { w = PO; }
    ticket_unlock!(MW);
    w
}

fn hjk_task() {
    loop {
        let w = Uart::get().readc();
        if w == 'q' as u8 {
            shutdown();
        }
    }
}


fn command_getter() {
    loop{
        spin_timer(720000);
        PO_w(unsafe {getinst()});
    }
}

fn vga_screen() {
    const MOTO: &'static str = "Blinky Texts: ";
    for i in 0..MOTO.len() {
        VGAScreen::get().write(i, 0x0600 | ((MOTO.as_bytes()[i] & 0xff) as u8) as u16);
    }

    loop {
        let eo = format!(" Curri $PC Moya: {:8x} ",PO_r());

        let text_msk = 0x6f00u16;
        let text_msk_highlight = 0x6800u16;
        let text_bias = 205;
        // VGAScreen::get().setcursor((eo.len() + 300) as u16);
        for i in 0..eo.len() {
            VGAScreen::get().write(i + text_bias + 100 + 1,
                                   text_msk_highlight| (eo.as_bytes()[i]  & 0xff) as u16)
        }
        static mut FIRST : bool = true;
        unsafe {
            if FIRST {

                VGAScreen::get().write(text_bias,
                                       text_msk | 0xc9 );
                for i in 0..eo.len() {
                    VGAScreen::get().write(text_bias + i + 1,
                                           text_msk | 0xcd );
                }
                VGAScreen::get().write(0 + text_bias + eo.len() + 1,
                                       text_msk | 0xbb);


                VGAScreen::get().write(text_bias + 100,
                                       text_msk | 0xba );
                VGAScreen::get().write(text_bias + 100,
                                       text_msk | 0xba );
                VGAScreen::get().write(text_bias + 100 + eo.len() + 1,
                                       text_msk | 0xba );


                VGAScreen::get().write(0 + text_bias + 200,
                                       text_msk | 0xc8 );
                for i in 0..eo.len() {
                    VGAScreen::get().write(text_bias + 200 + i + 1,
                                           text_msk | 0xcd );
                }
                VGAScreen::get().write(0 + text_bias + 200 + eo.len() + 1,
                                       text_msk | 0xbc );


                FIRST = false;
            };
        }
    }
}