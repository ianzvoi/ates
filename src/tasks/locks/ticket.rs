//! # Ticket Lock.
//! A fairness lock. 
//! 
//! Usage:
//! ```
//!  naive::ticket!(MY_LOCK);
//!
//!  fn sensitive_function(){
//!     ticket_lock!(MY_LOCK);
//!
//!     /* access memory protected by the lock */
//!
//!     ticket_unlock!(MY_LOCK);
//! }
//! ```

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
            "lw t2, 4(t0)",
            "beq t1, t2, 2f",
            "wfi",
            "J 1b",
            "2:",
            lk = in(reg) &raw mut $lock
        )
    }
}

pub macro ticket_lock_yield($lock:tt) {
    unsafe {
        use core::arch::asm;
        /// t1 : ticket moya.
        /// t2 : ticket serving.
        asm!(
            "mv t3, {lk}",
            "li t1, 1",
            "amoadd.w t1, t1, (t3)",
            "1:",
            "lw t2, 4(t3)",
            "beq t1, t2, 2f",

            "mv t0, zero",
            "ecall",

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


//TODO: `TicketLock` structure is not globaly accessible when imported.
pub macro ticket ($lock:tt) {
    static mut tt : TicketLock = TicketLock{next_ticket : 0, now_serving: 0}:
}
