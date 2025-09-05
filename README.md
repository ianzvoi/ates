# Riscv Rust Rtos



---

### 介绍
在**R**iscv架构的`qemu-virt32`虚拟机上运行的,
使用**R**ust编写的**R**TOS.

作为简易RTOS, 所有指令均在M模式下执行.


---



### 主要内容
##### Boot Loader:
* TODO

(目前, 此程序作为qemu的bios固件启动. (搬到物理机上则需要和具体的bootloader打交道))

##### 内存管理：
* **TODO**:
 
(目前暂时借用`linked_list_allocator`第三方Crate的实现.)


##### 多线程：
* 定时器产生等时间间隔中断, 抢占线程调度.
* 协作进程调度(**TODO**:协作调度的调度器.)


##### 并发安全：

(回旋锁)
* **TODO**:(取代`linked_list_allocator`第三方Crate内置回旋锁.)
* Naive Lock
* Ticket Lock

(互斥锁)
* Naive Mutex Lock

##### 外设
- CLINT (这或许不算是外设了)
- 一个勉强能用的PCI设备扫描
- qemu的standard-VGA显卡文字模式驱动 (这或许是bios的职责)
- UART (NS16550A)
- *TODO*: GPIO 
- *TODO*: SPI
- *TODO*: I2C

---


### More **TODO**s

[//]: # (期望目标)
- 让Task可以拥有单独的堆.
- 把内存安全措施和RUST的内存安全措施整合.
- 实现软时钟，和 `scheduled_sleep` 功能.
- 实现 `mutex_lock`.
- 让 `naive_lock` 可被动态分配.
- 移植到 xuantie-C910 物理机上. 

[//]: # (额外目标)
- 移植到 ESP32-C3 物理机上(由于硬件差异很大，可能不会在这个仓库里)

[//]: # (- 试着复现 XuantieC910 CPU 的 Ghostwrite 漏洞.)
- 虚拟内存与内存分页 (RTOS不一定需要这个，但是可以尝试实现.)

- 文件系统.


---

### 版本 & 依赖
- Rust 版本:
`nightly`

- Rust 构建工具: 
`riscv32imac-unknown-none-elf` 
