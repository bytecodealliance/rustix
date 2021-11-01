// riscv64 syscall argument register ordering is the same as the riscv64
// userspace argument register ordering except that the syscall number
// (nr) is passed in a7.
//
// nr_last.rs takes care of reordering the nr argument to the end for us,
// so we only need to move nr into a7.

    .file       "riscv64.S"

    .section    .text.rsix_syscall0_nr_last,"ax",@progbits
    .p2align    4
    .globl      rsix_syscall0_nr_last
    .weak       rsix_syscall0_nr_last
    .hidden     rsix_syscall0_nr_last
    .type       rsix_syscall0_nr_last, @function
rsix_syscall0_nr_last:
    .cfi_startproc
    mv      a7, a0
    ecall
    ret
    .cfi_endproc
    .size rsix_syscall0_nr_last, .-rsix_syscall0_nr_last

    .section    .text.rsix_syscall1_nr_last,"ax",@progbits
    .p2align    4
    .globl      rsix_syscall1_nr_last
    .weak       rsix_syscall1_nr_last
    .hidden     rsix_syscall1_nr_last
    .type       rsix_syscall1_nr_last, @function
rsix_syscall1_nr_last:
    .cfi_startproc
    mv      a7, a1
    ecall
    ret
    .cfi_endproc
    .size rsix_syscall1_nr_last, .-rsix_syscall1_nr_last

    .section    .text.rsix_syscall1_noreturn_nr_last,"ax",@progbits
    .p2align    4
    .globl      rsix_syscall1_noreturn_nr_last
    .weak       rsix_syscall1_noreturn_nr_last
    .hidden     rsix_syscall1_noreturn_nr_last
    .type       rsix_syscall1_noreturn_nr_last, @function
rsix_syscall1_noreturn_nr_last:
    .cfi_startproc
    mv      a7, a1
    ecall
    unimp
    .cfi_endproc
    .size rsix_syscall1_noreturn_nr_last, .-rsix_syscall1_noreturn_nr_last

    .section    .text.rsix_syscall2_nr_last,"ax",@progbits
    .p2align    4
    .globl      rsix_syscall2_nr_last
    .weak       rsix_syscall2_nr_last
    .hidden     rsix_syscall2_nr_last
    .type       rsix_syscall2_nr_last, @function
rsix_syscall2_nr_last:
    .cfi_startproc
    mv      a7, a2
    ecall
    ret
    .cfi_endproc
    .size rsix_syscall2_nr_last, .-rsix_syscall2_nr_last

    .section    .text.rsix_syscall3_nr_last,"ax",@progbits
    .p2align    4
    .globl      rsix_syscall3_nr_last
    .weak       rsix_syscall3_nr_last
    .hidden     rsix_syscall3_nr_last
    .type       rsix_syscall3_nr_last, @function
rsix_syscall3_nr_last:
    .cfi_startproc
    mv      a7, a3
    ecall
    ret
    .cfi_endproc
    .size rsix_syscall3_nr_last, .-rsix_syscall3_nr_last

    .section    .text.rsix_syscall4_nr_last,"ax",@progbits
    .p2align    4
    .globl      rsix_syscall4_nr_last
    .weak       rsix_syscall4_nr_last
    .hidden     rsix_syscall4_nr_last
    .type       rsix_syscall4_nr_last, @function
rsix_syscall4_nr_last:
    .cfi_startproc
    mv      a7, a4
    ecall
    ret
    .cfi_endproc
    .size rsix_syscall4_nr_last, .-rsix_syscall4_nr_last

    .section    .text.rsix_syscall5_nr_last,"ax",@progbits
    .p2align    4
    .globl      rsix_syscall5_nr_last
    .weak       rsix_syscall5_nr_last
    .hidden     rsix_syscall5_nr_last
    .type       rsix_syscall5_nr_last, @function
rsix_syscall5_nr_last:
    .cfi_startproc
    mv      a7, a5
    ecall
    ret
    .cfi_endproc
    .size rsix_syscall5_nr_last, .-rsix_syscall5_nr_last

    .section    .text.rsix_syscall6_nr_last,"ax",@progbits
    .p2align    4
    .globl      rsix_syscall6_nr_last
    .weak       rsix_syscall6_nr_last
    .hidden     rsix_syscall6_nr_last
    .type       rsix_syscall6_nr_last, @function
rsix_syscall6_nr_last:
    .cfi_startproc
    mv      a7, a6
    ecall
    ret
    .cfi_endproc
    .size rsix_syscall6_nr_last, .-rsix_syscall6_nr_last

    .section .note.GNU-stack,"",@progbits
