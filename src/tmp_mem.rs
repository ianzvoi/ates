use linked_list_allocator::LockedHeap;

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[export_name = "__init_heap"]
extern "C" fn init() {
    unsafe {
        let bottom;
        let top;
        core::arch::asm!(
            "la {b}, _init_heap_bottom",
            "la {t}, _init_heap_top",
            b = out(reg) bottom,
            t = out(reg) top,
            options(nomem),
        );
        ALLOCATOR.lock().init(bottom, top)
    }
}