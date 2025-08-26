# Riscv Rust Rtos

### 介绍
在**R**iscv架构的`qemu-virt32`虚拟机上运行的,
使用**R**ust编写的**R**TOS.



#### 任务
##### Boot Loader:
* 目前, 此程序作为qemu的bios固件启动.(搬到物理机上则需要和具体的bootloader打交道)

##### 内存管理：
* **TODO**:(目前暂时借用`linked_list_allocator`第三方Crate的实现.)


##### 多线程：
* 系统的定时器等时间间隔中断, 抢占线程调度.
* 协作进程调度(**TODO**:协作调度的调度器.)


##### 并发安全：
* 一个 Naive Lock (回旋锁)
* **TODO**:(取代`linked_list_allocator`第三方Crate内置回旋锁.)





#### More **TODO**s

[//]: # (期望目标)
- 让Task可以拥有单独的堆.
- 实现信号量.
- 让 `naive_lock` 可被动态分配.
- 移植到 xuantieC910 物理机上. 

[//]: # (额外目标)
- 移植到 ESP32-C3 物理机上(由于硬件差异很大，可能不会在这个仓库里)
- 虚拟内存与内存分页 (RTOS不一定需要这个，但是可以尝试实现.)
- 试着复现 XuantieC910 CPU 的 Ghostwrite 漏洞(~~如果到这一天时还可以买到带有这样严重漏洞的芯片~~).
- 文件系统.





#### 版本 & 依赖
- Rust 版本:
`nightly`

- Rust 构建工具: 
`riscv32imac-unknown-none-elf` 
