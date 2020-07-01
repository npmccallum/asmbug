#![feature(naked_functions)]
#![feature(asm)]

#[naked]
#[must_use]
#[inline(never)]
pub unsafe extern "C" fn save(ctx: &mut [usize; 8]) -> bool {
    asm!(
        "pop    rax",
        "mov    [rdi + 0x00], r12",
        "mov    [rdi + 0x08], r13",
        "mov    [rdi + 0x10], r14",
        "mov    [rdi + 0x18], r15",
        "mov    [rdi + 0x20], rbx",
        "mov    [rdi + 0x28], rbp",
        "mov    [rdi + 0x30], rsp",
        "mov    [rdi + 0x38], rax",
        "jmp    rax",
        in("rdi") ctx,
        options(noreturn)
    )
}
