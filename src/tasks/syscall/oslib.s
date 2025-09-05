    .global getinst
    .section .text
getinst:
    li t0, 0xBEEF
    ecall
    ret