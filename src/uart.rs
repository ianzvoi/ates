
pub struct Tty {
    base: usize
}

impl Tty {
    /// Create a new UART device.
    /// # Safety
    /// `base` must be the base address of a UART device.
    pub fn new(base: usize) -> Self {
        let addr= base as *mut u8;
        // Set data size to 8 bits.
        unsafe {
            core::ptr::write_volatile(addr.offset(1), 0b1);
            core::ptr::write_volatile(addr.offset(2), 0b1);
            core::ptr::write_volatile(addr.offset(3), 0b11);
        } 
        Tty { base }
    }

    pub fn put(&mut self, character: u8) {
        let ptr = self.base as *mut u8;
        // UNSAFE: fine as long as self.base is valid
        unsafe { core::ptr::write_volatile(ptr, character); }
    }
}

