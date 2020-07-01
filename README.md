I thought I'd do some exploration of the new Rust inline assembly support. Unfortunately, I think I've hit a bug.

When compiling (`cargo rustc -- --emit asm`), I get the following output:

```asm
        .section        .text._ZN6asmbug4save17h08c3eb7a61e00fbaE,"ax",@progbits
        .globl  _ZN6asmbug4save17h08c3eb7a61e00fbaE
        .p2align        4, 0x90
        .type   _ZN6asmbug4save17h08c3eb7a61e00fbaE,@function
_ZN6asmbug4save17h08c3eb7a61e00fbaE:
.Lfunc_begin0:
        .file   1 "asmbug/src/lib.rs"
        .loc    1 7 0
        .cfi_startproc
        movq    %rdi, (%rsp)		# This is definitely not correct...
.Ltmp0:
        .loc    1 8 5 prologue_end
        #APP

        popq    %rax
        movq    %r12, (%rdi)
        movq    %r13, 8(%rdi)
        movq    %r14, 16(%rdi)
        movq    %r15, 24(%rdi)
        movq    %rbx, 32(%rdi)
        movq    %rbp, 40(%rdi)
        movq    %rsp, 48(%rdi)
        movq    %rax, 56(%rdi)
        jmpq    *%rax

        #NO_APP
        ud2
.Ltmp1:
.Lfunc_end0:
        .size   _ZN6asmbug4save17h08c3eb7a61e00fbaE, .Lfunc_end0-_ZN6asmbug4save17h08c3eb7a61e00fbaE
        .cfi_endproc
```

However, if I compile a different way (`rustc src/lib.rs --crate-type lib --emit asm`), I don't get the offending line:

```asm
        .section        .text._ZN3lib4save17h41fda79ed270926dE,"ax",@progbits
        .globl  _ZN3lib4save17h41fda79ed270926dE
        .p2align        4, 0x90
        .type   _ZN3lib4save17h41fda79ed270926dE,@function
_ZN3lib4save17h41fda79ed270926dE:
        .cfi_startproc
        #APP

        popq    %rax
        movq    %r12, (%rdi)
        movq    %r13, 8(%rdi)
        movq    %r14, 16(%rdi)
        movq    %r15, 24(%rdi)
        movq    %rbx, 32(%rdi)
        movq    %rbp, 40(%rdi)
        movq    %rsp, 48(%rdi)
        movq    %rax, 56(%rdi)
        jmpq    *%rax

        #NO_APP
        ud2
.Lfunc_end0:
        .size   _ZN3lib4save17h41fda79ed270926dE, .Lfunc_end0-_ZN3lib4save17h41fda79ed270926dE
        .cfi_endproc
```

Likewise, if I compile for release (`cargo rustc --release -- --emit asm`), the offending line is missing:

```asm
        .section        .text._ZN6asmbug4save17h151c2e3546492694E,"ax",@progbits
        .globl  _ZN6asmbug4save17h151c2e3546492694E
        .p2align        4, 0x90
        .type   _ZN6asmbug4save17h151c2e3546492694E,@function
_ZN6asmbug4save17h151c2e3546492694E:
        .cfi_startproc
        #APP

        popq    %rax
        movq    %r12, (%rdi)
        movq    %r13, 8(%rdi)
        movq    %r14, 16(%rdi)
        movq    %r15, 24(%rdi)
        movq    %rbx, 32(%rdi)
        movq    %rbp, 40(%rdi)
        movq    %rsp, 48(%rdi)
        movq    %rax, 56(%rdi)
        jmpq    *%rax

        #NO_APP
        ud2
.Lfunc_end0:
        .size   _ZN6asmbug4save17h151c2e3546492694E, .Lfunc_end0-_ZN6asmbug4save17h151c2e3546492694E
        .cfi_endproc
```
