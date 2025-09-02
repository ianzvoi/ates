use core::sync::atomic::AtomicU16;

pub struct TicketLock {
    pub(crate) next_ticket: u32,
    pub(crate) now_serving: u32,
}


pub macro ticket_lock($lock:tt) {
    unsafe {
        use core::arch::asm;
        /// t1 : ticket moya.
        /// t2 : ticket serving.
        asm!(
            "mv t0, {lk}",
            "li t1, 1",
            "amoadd.w t1, t1, (t0)",
            "1:",
            "lhu t2, 4(t0)",
            "beq t1, t2, 2f",
            "wfi",
            "J 1b",
            "2:",
            lk = in(reg) &raw mut $lock
        )
    }
}

pub macro ticket_unlock ($lock:tt) {
    unsafe {
        use core::arch::asm;
        asm!(
            "mv t0, {lk}",
            "c.addi t0, 4",
            "li t1, 1",
            "amoadd.w zero, t1, (t0)",
            lk = in(reg) &raw mut $lock,    
        )
    }
}

pub macro ticket ($lock:tt) {
    static mut tt : TicketLock = TicketLock{next_ticket : 0, now_serving: 0}:
}
