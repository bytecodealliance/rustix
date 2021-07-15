// riscv64 syscall argument register ordering is the same as the riscv64
// userspace argument register ordering except that the syscall number
// (nr) is passed in a7.
//
// outline.rs takes care of reordering the nr argument to the end for us,
// so we only need to move nr into eax and move rcx into r10 as needed.

    .section    .text.reordered_syscall,"ax",@progbits
    .p2align    4
reordered_syscall:
reordered_syscall0:
reordered_syscall1:
reordered_syscall2:
reordered_syscall3:
reordered_syscall4:
reordered_syscall5:
reordered_syscall6:
    .cfi_startproc
    ecall
    ret
    .cfi_endproc
    .size reordered_syscall, .-reordered_syscall

    .section    .text.reordered_syscall1_noreturn,"ax",@progbits
    .p2align    4
reordered_syscall1_noreturn:
    .cfi_startproc
    ecall
    unimp
    .cfi_endproc
    .size reordered_syscall_noreturn, .-reordered_syscall_noreturn

    .globl   reordered_syscall0
    .globl   reordered_syscall1
    .globl   reordered_syscall1_noreturn
    .globl   reordered_syscall2
    .globl   reordered_syscall3
    .globl   reordered_syscall4
    .globl   reordered_syscall5
    .globl   reordered_syscall6

    .hidden  reordered_syscall0
    .hidden  reordered_syscall1
    .hidden  reordered_syscall1_noreturn
    .hidden  reordered_syscall2
    .hidden  reordered_syscall3
    .hidden  reordered_syscall4
    .hidden  reordered_syscall5
    .hidden  reordered_syscall6

    .type    reordered_syscall0, @function
    .type    reordered_syscall1, @function
    .type    reordered_syscall1_noreturn, @function
    .type    reordered_syscall2, @function
    .type    reordered_syscall3, @function
    .type    reordered_syscall4, @function
    .type    reordered_syscall5, @function
    .type    reordered_syscall6, @function

    // We can't describe calls to external asm as readonly, so just alias
    // these to their non-readonly counterparts.
    .set     reordered_syscall0_readonly, reordered_syscall0
    .set     reordered_syscall1_readonly, reordered_syscall1
    .set     reordered_syscall2_readonly, reordered_syscall2
    .set     reordered_syscall3_readonly, reordered_syscall3
    .set     reordered_syscall4_readonly, reordered_syscall4
    .set     reordered_syscall5_readonly, reordered_syscall5
    .set     reordered_syscall6_readonly, reordered_syscall6

    .globl   reordered_syscall0_readonly
    .globl   reordered_syscall1_readonly
    .globl   reordered_syscall2_readonly
    .globl   reordered_syscall3_readonly
    .globl   reordered_syscall4_readonly
    .globl   reordered_syscall5_readonly
    .globl   reordered_syscall6_readonly

    .hidden  reordered_syscall0_readonly
    .hidden  reordered_syscall1_readonly
    .hidden  reordered_syscall2_readonly
    .hidden  reordered_syscall3_readonly
    .hidden  reordered_syscall4_readonly
    .hidden  reordered_syscall5_readonly
    .hidden  reordered_syscall6_readonly

    .section .note.GNU-stack,"",@progbits
