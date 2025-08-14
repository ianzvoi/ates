/** documents of clint
* https://chromitem-soc.readthedocs.io/en/latest/clint.html
*/

use core::arch::global_asm;
global_asm!(include_str!("clint.s"));

extern "C" {
    pub fn _get_mtime() -> *const u64;
    pub fn _get_mtimecmpr() -> *const u64;
}