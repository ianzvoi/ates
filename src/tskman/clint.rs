/** documents of clint
* https://chromitem-soc.readthedocs.io/en/latest/clint.html
*/

use core::arch::global_asm;
global_asm!(include_str!("clint.s"));


pub fn timer_start() {
    unsafe {
        // the write instruction of 0x2000000 will start ticking.
        core::ptr::write_volatile(0x2000000 as *mut u8, 0);
    }
}
pub fn timer_set(mtimecmp: u64) {
    unsafe { // Never Fail
        core::ptr::write_volatile(0x2004000 as *mut u64, mtimecmp);
    }
}
