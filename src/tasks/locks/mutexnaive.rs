//! # The Mutex Naive Lock (half spin lock, half mutex lock)
//! 
//! Usage:
//! ```
//!  naive::mutexnaive!(MY_LOCK);
//! 
//!  fn sensitive_function(){
//!     naive::mutexnaive_lock!(MY_LOCK);
//! 
//!     /* access memory protected by the lock */
//! 
//!     naive::mutexnaive_unlock!(MY_LOCK);
//! }
//! ```
//! 
//! Basicaly The Mutex Naive Lock Works Likes this:
//! ```rust
//! fn lock(lc : mut ref):
//!     while(lc is LOCKED):
//!         Switch to Next task.
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
/// - if not, trigger a soft exception, forcing tskman to switch to another task.  
pub macro mutexnaive_lock ($lock:tt) {
    unsafe {
        use core::arch::asm;
        asm!(
            "1:",
            "mv t0, {lk}",
            "li t1, 1",
            "amoswap.w t1, t1, (t0)",
            "beqz t1, 2f",
            // mark some register.
            // or call _switch function to reduce cost of task switching.
            "ecall",
            "J 1b",
            "2:",
            lk = in(reg) &raw mut $lock,
        )
    }
}

/// ## unlock
pub macro mutexnaive_unlock ($lock:tt) {
    unsafe {
        $lock = 0;
    }
}

//TODO: make the lock dynamicaly acllocatble 
/// ## create lock globaly
pub macro mutexnaive ($name:ident) {
    static mut $name: NaiveLock = 0;
}
