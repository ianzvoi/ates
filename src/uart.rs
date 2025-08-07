/** Serial Utilities
* for Qemu-virt, specifically, there is a NS16550A mapped near 0x10000000.
* https://uart16550.readthedocs.io/_/downloads/en/latest/pdf/
*/

const UART_BASE: *mut u8 = 0x10000000 as *mut u8;

pub struct Uart {
    fifo : *mut u8
}

impl Uart {
    pub fn get() -> Uart {
        Uart{fifo : UART_BASE}
    }
    pub fn writec(&mut self, byte: u8) {
        unsafe { core::ptr::write_volatile(self.fifo, byte); }
    }

    pub fn write(&mut self, bytes: &[u8], len : usize) {
        for i in 0..len {
            unsafe { core::ptr::write_volatile(self.fifo, bytes[i]); }
        }
    }

    pub fn read(& self, dest : *mut u8, len : usize) -> u8 {
        0
        // not implemented.
    }

    pub fn readc(& self) -> (u8, u8){
        ('a' as u8, 0)
        // not implemented.
    }
}

pub fn init(){

    unsafe {
        core::ptr::write_volatile(UART_BASE.offset(1), 0b001);
        //IER - Receive interrupt only.

        core::ptr::write_volatile(UART_BASE.offset(2), 0b001);
        //IIR - TransmitterHolding Register empty.

        core::ptr::write_volatile(UART_BASE.offset(3), 0b11);
        //FCR - 8 byte mode & clear recieve register.
    }
}


