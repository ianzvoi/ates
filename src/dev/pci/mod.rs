use alloc::fmt::format;
use alloc::format;
use core::ptr::{read, read_volatile, write_volatile};
use crate::dev::uart::Uart;

const PCI_ECAM : usize = 0x30000000usize;
const PCI_PIO : usize = 0x3000000usize;
fn report(w : u32, ad : u32){
    let s = format!("PCI ID : {:8x} @ {:X}.\n", w, ad);
    Uart::get().write(s.as_bytes(),s.len());
}

pub fn scan_pci_devices() {

    let ecam_base: u32 = PCI_ECAM as u32;;
    let bf : u32 = 0x40000000u32;
    for dev in 0..32{
        unsafe {
            let ad = ((dev << 11) + ecam_base) as *mut u32;
            let q = read_volatile(ad);
            if (q == 0xffffffffu32) { continue;}
            if (q == 0x11111234u32) {
                write_volatile(ad.add(1),0x7u32);
                
                // write_volatile(ad.add(4 + 0),0xffffffffu32);
                write_volatile(ad.add(4 + 0),0x70000000u32);
                
                // write_volatile(ad.add(4 + 2),0xffffffffu32);
                write_volatile(ad.add(4 + 2),0x7E000000u32);
                
            }
            report(q, ((dev << 11) + ecam_base));
        }
    }
}


pub unsafe fn pio_w16(addr : usize, val : u16) {
    write_volatile((PCI_PIO+addr) as *mut u16, val as u16);
}

pub unsafe fn pio_r16(addr : usize) -> u16{
    read_volatile(addr as *mut u16)
}