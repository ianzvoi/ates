# Riscv Rust Rtos

### 介绍
在qemu的virt虚拟机上实现
具有内存管理的多线程RTOS

#### 任务
内存管理：

    目前借用了linked_list_allocator的实现


多线程：

    通过系统的定时器中断进行平均线程调度


并发安全：

    TODO
    *(事实上，linked_list_allocator已经内置了一个回旋锁了)


#### 依赖
- rust nightly 
- rust riscv32imac-unknown-none-elf 构建工具