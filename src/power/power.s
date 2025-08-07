    .section .text
    .global _machine_reset
    .global _machine_shutdown
    .global _machine_fail

_machine_reset:
    li t0, 0x100000
    li t1, 0x7777
    sw t1, 0(t0)

_machine_shutdown:
    li t0, 0x100000
    li t1, 0x5555
    sw t1, 0(t0)
_machine_fail:
    li t0, 0x100000
    li t1, 0x3333
    sw t1, 0(t0)



