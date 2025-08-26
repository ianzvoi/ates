pub type NaiveLock = u8;

#[macro_export] macro_rules! naive_lock {
    ($lock:tt) => {
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
    };
}


#[macro_export] macro_rules! naive_unlock {
    ($lock:tt) => {
        unsafe {
            $lock = 0;
        }
    };
}

#[macro_export] macro_rules! naive {
    ($name:ident) => {
        static mut $name: NaiveLock = 0;
    }
}
