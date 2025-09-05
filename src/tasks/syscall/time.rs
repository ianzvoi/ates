use core::arch::asm;


/// # SPIN TIMER
/// 
/// trapped in a yield loop till timer is greater than target.
/// 
/// Timer Overflow is handled. if sampling <= timer loop period.
/// ```psudo
/// 
/// target = get_time() + delta 
/// 
/// while (get_time() - target < delta):
///     yield
/// 
/// ```
pub fn spin_timer(delta : usize) {
    unsafe {
        asm!(
        "lw t1 , _clint_mtime",
        "mv t3 , {delta}",
        "2: lw t4, _clint_mtime",
        "sub t2, t4, t1",
        "bltu t2, t3, 1f",
        "J 3f",
        "1:",
        "mv t0, zero",
        "ecall",
        "J 2b",
        "3:",
        delta = in(reg) delta,
        );
    }
}
