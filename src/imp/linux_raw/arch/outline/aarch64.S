// aarch64 syscall argument register ordering is the same as the aarch64
// userspace argument register ordering except that the syscall number
// (nr) is passed in w8.
//
// outline.rs takes care of reordering the nr argument to the end for us,
// so we only need to move nr into w8.

    .section    .text.posish_reordered_syscall0,"ax",@progbits
    .p2align    2
posish_reordered_syscall0:
    .cfi_startproc
    mov    w8, w0
    svc    #0
    ret
    .cfi_endproc
    .size posish_reordered_syscall0, .-posish_reordered_syscall0

    .section    .text.posish_reordered_syscall1,"ax",@progbits
    .p2align    2
posish_reordered_syscall1:
    .cfi_startproc
    mov    w8, w1
    svc    #0
    ret
    .cfi_endproc
    .size posish_reordered_syscall1, .-posish_reordered_syscall1

    .section    .text.posish_reordered_syscall1_noreturn,"ax",@progbits
    .p2align    2
posish_reordered_syscall1_noreturn:
    .cfi_startproc
    mov    w8, w1
    svc    #0
    brk    #0x1
    .cfi_endproc
    .size posish_reordered_syscall1_noreturn, .-posish_reordered_syscall1_noreturn

    .section    .text.posish_reordered_syscall2,"ax",@progbits
    .p2align    2
posish_reordered_syscall2:
    .cfi_startproc
    mov    w8, w2
    svc    #0
    ret
    .cfi_endproc
    .size posish_reordered_syscall2, .-posish_reordered_syscall2

    .section    .text.posish_reordered_syscall3,"ax",@progbits
    .p2align    2
posish_reordered_syscall3:
    .cfi_startproc
    mov    w8, w3
    svc    #0
    ret
    .cfi_endproc
    .size posish_reordered_syscall3, .-posish_reordered_syscall3

    .section    .text.posish_reordered_syscall4,"ax",@progbits
    .p2align    2
posish_reordered_syscall4:
    .cfi_startproc
    mov    w8, w4
    svc    #0
    ret
    .cfi_endproc
    .size posish_reordered_syscall4, .-posish_reordered_syscall4

    .section    .text.posish_reordered_syscall5,"ax",@progbits
    .p2align    2
posish_reordered_syscall5:
    .cfi_startproc
    mov    w8, w5
    svc    #0
    ret
    .cfi_endproc
    .size posish_reordered_syscall5, .-posish_reordered_syscall5

    .section    .text.posish_reordered_syscall6,"ax",@progbits
    .p2align    2
posish_reordered_syscall6:
    .cfi_startproc
    mov    w8, w6
    svc    #0
    ret
    .cfi_endproc
    .size posish_reordered_syscall6, .-posish_reordered_syscall6

    .globl   posish_reordered_syscall0
    .globl   posish_reordered_syscall1
    .globl   posish_reordered_syscall1_noreturn
    .globl   posish_reordered_syscall2
    .globl   posish_reordered_syscall3
    .globl   posish_reordered_syscall4
    .globl   posish_reordered_syscall5
    .globl   posish_reordered_syscall6

    .hidden  posish_reordered_syscall0
    .hidden  posish_reordered_syscall1
    .hidden  posish_reordered_syscall1_noreturn
    .hidden  posish_reordered_syscall2
    .hidden  posish_reordered_syscall3
    .hidden  posish_reordered_syscall4
    .hidden  posish_reordered_syscall5
    .hidden  posish_reordered_syscall6

    .type    posish_reordered_syscall0, @function
    .type    posish_reordered_syscall1, @function
    .type    posish_reordered_syscall1_noreturn, @function
    .type    posish_reordered_syscall2, @function
    .type    posish_reordered_syscall3, @function
    .type    posish_reordered_syscall4, @function
    .type    posish_reordered_syscall5, @function
    .type    posish_reordered_syscall6, @function

    // We can't describe calls to external asm as readonly, so just alias
    // these to their non-readonly counterparts.
    .set     posish_reordered_syscall0_readonly, posish_reordered_syscall0
    .set     posish_reordered_syscall1_readonly, posish_reordered_syscall1
    .set     posish_reordered_syscall2_readonly, posish_reordered_syscall2
    .set     posish_reordered_syscall3_readonly, posish_reordered_syscall3
    .set     posish_reordered_syscall4_readonly, posish_reordered_syscall4
    .set     posish_reordered_syscall5_readonly, posish_reordered_syscall5
    .set     posish_reordered_syscall6_readonly, posish_reordered_syscall6

    .globl   posish_reordered_syscall0_readonly
    .globl   posish_reordered_syscall1_readonly
    .globl   posish_reordered_syscall2_readonly
    .globl   posish_reordered_syscall3_readonly
    .globl   posish_reordered_syscall4_readonly
    .globl   posish_reordered_syscall5_readonly
    .globl   posish_reordered_syscall6_readonly

    .hidden  posish_reordered_syscall0_readonly
    .hidden  posish_reordered_syscall1_readonly
    .hidden  posish_reordered_syscall2_readonly
    .hidden  posish_reordered_syscall3_readonly
    .hidden  posish_reordered_syscall4_readonly
    .hidden  posish_reordered_syscall5_readonly
    .hidden  posish_reordered_syscall6_readonly

    .section .note.GNU-stack,"",@progbits
