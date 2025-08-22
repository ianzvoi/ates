#[repr(C)]
pub struct TaskContext {
    reg : [u32; 31],
}


impl TaskContext {
    pub const fn new() -> TaskContext {
        TaskContext {reg: [0; 31]}
    }
}



