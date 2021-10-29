// aarch64 syscall argument register ordering is the same as the aarch64
// userspace argument register ordering except that the syscall number
// (nr) is passed in w8.
//
// outline.rs takes care of reordering the nr argument to the end for us,
// so we only need to move nr into w8.

    .file       "aarch64.S"
    .arch       armv8-a

    .section    .text.rsix_syscall0_nr_last,"ax",@progbits
    .p2align    2
    .globl      rsix_syscall0_nr_last
    .weak       rsix_syscall0_nr_last
    .hidden     rsix_syscall0_nr_last
    .type       rsix_syscall0_nr_last, @function
rsix_syscall0_nr_last:
    .cfi_startproc
    mov    w8, w0
    svc    #0
    ret
    .cfi_endproc
    .size rsix_syscall0_nr_last, .-rsix_syscall0_nr_last

    .section    .text.rsix_syscall1_nr_last,"ax",@progbits
    .p2align    2
    .globl      rsix_syscall1_nr_last
    .weak       rsix_syscall1_nr_last
    .hidden     rsix_syscall1_nr_last
    .type       rsix_syscall1_nr_last, @function
rsix_syscall1_nr_last:
    .cfi_startproc
    mov    w8, w1
    svc    #0
    ret
    .cfi_endproc
    .size rsix_syscall1_nr_last, .-rsix_syscall1_nr_last

    .section    .text.rsix_syscall1_noreturn_nr_last,"ax",@progbits
    .p2align    2
    .globl      rsix_syscall1_noreturn_nr_last
    .weak       rsix_syscall1_noreturn_nr_last
    .hidden     rsix_syscall1_noreturn_nr_last
    .type       rsix_syscall1_noreturn_nr_last, @function
rsix_syscall1_noreturn_nr_last:
    .cfi_startproc
    mov    w8, w1
    svc    #0
    brk    #0x1
    .cfi_endproc
    .size rsix_syscall1_noreturn_nr_last, .-rsix_syscall1_noreturn_nr_last

    .section    .text.rsix_syscall2_nr_last,"ax",@progbits
    .p2align    2
    .globl      rsix_syscall2_nr_last
    .weak       rsix_syscall2_nr_last
    .hidden     rsix_syscall2_nr_last
    .type       rsix_syscall2_nr_last, @function
rsix_syscall2_nr_last:
    .cfi_startproc
    mov    w8, w2
    svc    #0
    ret
    .cfi_endproc
    .size rsix_syscall2_nr_last, .-rsix_syscall2_nr_last

    .section    .text.rsix_syscall3_nr_last,"ax",@progbits
    .p2align    2
    .globl      rsix_syscall3_nr_last
    .weak       rsix_syscall3_nr_last
    .hidden     rsix_syscall3_nr_last
    .type       rsix_syscall3_nr_last, @function
rsix_syscall3_nr_last:
    .cfi_startproc
    mov    w8, w3
    svc    #0
    ret
    .cfi_endproc
    .size rsix_syscall3_nr_last, .-rsix_syscall3_nr_last

    .section    .text.rsix_syscall4_nr_last,"ax",@progbits
    .p2align    2
    .globl      rsix_syscall4_nr_last
    .weak       rsix_syscall4_nr_last
    .hidden     rsix_syscall4_nr_last
    .type       rsix_syscall4_nr_last, @function
rsix_syscall4_nr_last:
    .cfi_startproc
    mov    w8, w4
    svc    #0
    ret
    .cfi_endproc
    .size rsix_syscall4_nr_last, .-rsix_syscall4_nr_last

    .section    .text.rsix_syscall5_nr_last,"ax",@progbits
    .p2align    2
    .globl      rsix_syscall5_nr_last
    .weak       rsix_syscall5_nr_last
    .hidden     rsix_syscall5_nr_last
    .type       rsix_syscall5_nr_last, @function
rsix_syscall5_nr_last:
    .cfi_startproc
    mov    w8, w5
    svc    #0
    ret
    .cfi_endproc
    .size rsix_syscall5_nr_last, .-rsix_syscall5_nr_last

    .section    .text.rsix_syscall6_nr_last,"ax",@progbits
    .p2align    2
    .globl      rsix_syscall6_nr_last
    .weak       rsix_syscall6_nr_last
    .hidden     rsix_syscall6_nr_last
    .type       rsix_syscall6_nr_last, @function
rsix_syscall6_nr_last:
    .cfi_startproc
    mov    w8, w6
    svc    #0
    ret
    .cfi_endproc
    .size rsix_syscall6_nr_last, .-rsix_syscall6_nr_last

    .section .note.GNU-stack,"",@progbits
