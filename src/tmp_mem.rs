use linked_list_allocator::LockedHeap;

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init() {
    let bottom;
    let top : *mut u8;
    
    unsafe {
        core::arch::asm!(
            "la {b}, _init_heap_bottom",
            "la {t}, _heap_size",
            b = out(reg) bottom,
            t = out(reg) top,
            options(nomem),
        );
    }
    
    unsafe {
        ALLOCATOR.lock().init(bottom, (top) as usize);
    }
}