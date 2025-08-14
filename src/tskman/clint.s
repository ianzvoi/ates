    .section .text

    .global _get_mtime
_get_mtime:
    la a0, 0x200BFF8
    ret

    .global _get_mtimecmpr
_get_mtimecmpr:
    la a0, 0x2004000
    ret
   