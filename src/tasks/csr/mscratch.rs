pub macro csr_mscratch_write($ptTCB:expr) {
    core::arch::asm!(
        "csrw mscratch, {}",
        in(reg) ($ptTCB)
    );
}
