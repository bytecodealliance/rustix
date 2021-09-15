// riscv64 syscall argument register ordering is the same as the riscv64
// userspace argument register ordering except that the syscall number
// (nr) is passed in a7.
//
// nr_last.rs takes care of reordering the nr argument to the end for us,
// so we only need to move nr into a7.

    .section    .text.rsix_reordered_syscall0,"ax",@progbits
    .p2align    4
rsix_reordered_syscall0:
    .cfi_startproc
    mv      a7, a0
    ecall
    ret
    .cfi_endproc
    .size rsix_reordered_syscall0, .-rsix_reordered_syscall0

    .section    .text.rsix_reordered_syscall1,"ax",@progbits
    .p2align    4
rsix_reordered_syscall1:
    .cfi_startproc
    mv      a7, a1
    ecall
    ret
    .cfi_endproc
    .size rsix_reordered_syscall1, .-rsix_reordered_syscall1

    .section    .text.rsix_reordered_syscall1_noreturn,"ax",@progbits
    .p2align    4
rsix_reordered_syscall1_noreturn:
    .cfi_startproc
    mv      a7, a1
    ecall
    unimp
    .cfi_endproc
    .size rsix_reordered_syscall1_noreturn, .-rsix_reordered_syscall1_noreturn

    .section    .text.rsix_reordered_syscall2,"ax",@progbits
    .p2align    4
rsix_reordered_syscall2:
    .cfi_startproc
    mv      a7, a2
    ecall
    ret
    .cfi_endproc
    .size rsix_reordered_syscall2, .-rsix_reordered_syscall2

    .section    .text.rsix_reordered_syscall3,"ax",@progbits
    .p2align    4
rsix_reordered_syscall3:
    .cfi_startproc
    mv      a7, a3
    ecall
    ret
    .cfi_endproc
    .size rsix_reordered_syscall3, .-rsix_reordered_syscall3

    .section    .text.rsix_reordered_syscall4,"ax",@progbits
    .p2align    4
rsix_reordered_syscall4:
    .cfi_startproc
    mv      a7, a4
    ecall
    ret
    .cfi_endproc
    .size rsix_reordered_syscall4, .-rsix_reordered_syscall4

    .section    .text.rsix_reordered_syscall5,"ax",@progbits
    .p2align    4
rsix_reordered_syscall5:
    .cfi_startproc
    mv      a7, a5
    ecall
    ret
    .cfi_endproc
    .size rsix_reordered_syscall5, .-rsix_reordered_syscall5

    .section    .text.rsix_reordered_syscall6,"ax",@progbits
    .p2align    4
rsix_reordered_syscall6:
    .cfi_startproc
    mv      a7, a6
    ecall
    ret
    .cfi_endproc
    .size rsix_reordered_syscall6, .-rsix_reordered_syscall6

    .globl   rsix_reordered_syscall0
    .globl   rsix_reordered_syscall1
    .globl   rsix_reordered_syscall1_noreturn
    .globl   rsix_reordered_syscall2
    .globl   rsix_reordered_syscall3
    .globl   rsix_reordered_syscall4
    .globl   rsix_reordered_syscall5
    .globl   rsix_reordered_syscall6

    .hidden  rsix_reordered_syscall0
    .hidden  rsix_reordered_syscall1
    .hidden  rsix_reordered_syscall1_noreturn
    .hidden  rsix_reordered_syscall2
    .hidden  rsix_reordered_syscall3
    .hidden  rsix_reordered_syscall4
    .hidden  rsix_reordered_syscall5
    .hidden  rsix_reordered_syscall6

    .type    rsix_reordered_syscall0, @function
    .type    rsix_reordered_syscall1, @function
    .type    rsix_reordered_syscall1_noreturn, @function
    .type    rsix_reordered_syscall2, @function
    .type    rsix_reordered_syscall3, @function
    .type    rsix_reordered_syscall4, @function
    .type    rsix_reordered_syscall5, @function
    .type    rsix_reordered_syscall6, @function

    // We can't describe calls to external asm as readonly, so just alias
    // these to their non-readonly counterparts.
    .set     rsix_reordered_syscall0_readonly, rsix_reordered_syscall0
    .set     rsix_reordered_syscall1_readonly, rsix_reordered_syscall1
    .set     rsix_reordered_syscall2_readonly, rsix_reordered_syscall2
    .set     rsix_reordered_syscall3_readonly, rsix_reordered_syscall3
    .set     rsix_reordered_syscall4_readonly, rsix_reordered_syscall4
    .set     rsix_reordered_syscall5_readonly, rsix_reordered_syscall5
    .set     rsix_reordered_syscall6_readonly, rsix_reordered_syscall6

    .globl   rsix_reordered_syscall0_readonly
    .globl   rsix_reordered_syscall1_readonly
    .globl   rsix_reordered_syscall2_readonly
    .globl   rsix_reordered_syscall3_readonly
    .globl   rsix_reordered_syscall4_readonly
    .globl   rsix_reordered_syscall5_readonly
    .globl   rsix_reordered_syscall6_readonly

    .hidden  rsix_reordered_syscall0_readonly
    .hidden  rsix_reordered_syscall1_readonly
    .hidden  rsix_reordered_syscall2_readonly
    .hidden  rsix_reordered_syscall3_readonly
    .hidden  rsix_reordered_syscall4_readonly
    .hidden  rsix_reordered_syscall5_readonly
    .hidden  rsix_reordered_syscall6_readonly

    .section .note.GNU-stack,"",@progbits
