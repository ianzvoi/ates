    .section .text
    .global _timer_start
_timer_start:
    la t0, 0x2000000
    sb t1, (t0)
    ret