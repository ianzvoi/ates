//! # The Naive Lock (spin lock)
//! 
//! Usage:
//! ```
//!  naive::naive!(MY_LOCK);
//! 
//!  fn sensitive_function(){
//!     naive::naive_lock!(MY_LOCK);
//! 
//!     /* access memory protected by the lock */
//! 
//!     naive::naive_unlock!(MY_LOCK);
//! }
//! ```
//! 
//! Basicaly The Naive Lock Works Likes this:
//! ```rust
//! fn lock(lc : mut ref):
//!     while(lc is LOCKED):
//!         W.F.I
//!     lc <- LOCKED
//!     
//!
//! fn unlock(lc : mut ref):
//!     lc <- LOCKED
//!
//! ``` 
//! but in atomic form.
//!

pub type NaiveLock = u8;


/// ## try to get the lock
/// If `lock` is LOCKED, it will fail in a loop
/// that check if the lock is UNLOCKED,
/// - if so, break the loop
/// - if not, **it will freeze CPU** untill next interruption.  
pub macro naive_lock ($lock:tt) {
    unsafe {
        use core::arch::asm;
        asm!(
            "1:",
            "mv t0, {lk}",
            "li t1, 1",
            "amoswap.w t1, t1, (t0)",
            "beqz t1, 2f",
            "wfi",
            "J 1b",
            "2:",
            lk = in(reg) &raw mut $lock,
        )
    }
}


pub macro naive_lock_yield ($lock:tt) {
    unsafe {
        use core::arch::asm;
        asm!(
            "1:",
            "mv t2, {lk}",
            "li t1, 1",
            "amoswap.w t1, t1, (t2)",
            "beqz t1, 2f",
            
            "mv t0, zero",
            "ecall",
            
            "J 1b",
            "2:",
            lk = in(reg) &raw mut $lock,
        )
    }
}

/// same as `naive_lock`
/// 
/// but this one won't freaze cpu.
pub macro naive_lock_freaze_not ($lock:tt) {
    unsafe {
        use core::arch::asm;
        asm!(
            "1:",
            "mv t0, {lk}",
            "li t1, 1",
            "amoswap.w t1, t1, (t0)",
            "beqz t1, 2f",
            "J 1b",
            "2:",
            lk = in(reg) &raw mut $lock,
        )
    }
}

/// ## unlock
pub macro naive_unlock ($lock:tt) {
    unsafe {
        $lock = 0;
    }
}

//TODO: make the lock dynamicaly acllocatble 
/// ## create lock globaly
pub macro naive ($name:ident) {
    static mut $name: NaiveLock = 0;
}
