    .p2align    2
syscall0:
    .cfi_startproc
    mov    w8, w0
    svc    #0
    ret
    .cfi_endproc
    .size syscall0, .-syscall0

    .p2align    2
syscall1:
    .cfi_startproc
    mov    w8, w0
    mov    x0, x1
    svc    #0
    ret
    .cfi_endproc
    .size syscall1, .-syscall1

    .p2align    2
syscall1_noreturn:
    .cfi_startproc
    mov    w8, w0
    mov    x0, x1
    svc    #0
    brk    #0x1
    .cfi_endproc
    .size syscall1_noreturn, .-syscall1_noreturn

    .p2align    2
syscall2:
    .cfi_startproc
    mov    w8, w0
    mov    x0, x1
    mov    x1, x2
    svc    #0
    ret
    .cfi_endproc
    .size syscall2, .-syscall2

    .p2align    2
syscall3:
    .cfi_startproc
    mov    w8, w0
    mov    x0, x1
    mov    x1, x2
    mov    x2, x3
    svc    #0
    ret
    .cfi_endproc
    .size syscall3, .-syscall3

    .p2align    2
syscall4:
    .cfi_startproc
    mov    w8, w0
    mov    x0, x1
    mov    x1, x2
    mov    x2, x3
    mov    x3, x4
    svc    #0
    ret
    .cfi_endproc
    .size syscall4, .-syscall4

    .p2align    2
syscall5:
    .cfi_startproc
    mov    w8, w0
    mov    x0, x1
    mov    x1, x2
    mov    x2, x3
    mov    x3, x4
    mov    x4, x5
    svc    #0
    ret
    .cfi_endproc
    .size syscall5, .-syscall5

    .p2align    2
syscall6:
    .cfi_startproc
    mov    w8, w0
    mov    x0, x1
    mov    x1, x2
    mov    x2, x3
    mov    x3, x4
    mov    x4, x5
    mov    x5, x6
    svc    #0
    ret
    .cfi_endproc
    .size syscall6, .-syscall6

    .globl   syscall0
    .globl   syscall0
    .globl   syscall1
    .globl   syscall1_noreturn
    .globl   syscall2
    .globl   syscall3
    .globl   syscall4
    .globl   syscall5
    .globl   syscall6

    .hidden  syscall0
    .hidden  syscall0
    .hidden  syscall1
    .hidden  syscall1_noreturn
    .hidden  syscall2
    .hidden  syscall3
    .hidden  syscall4
    .hidden  syscall5
    .hidden  syscall6

    .type    syscall0, @function
    .type    syscall0, @function
    .type    syscall1, @function
    .type    syscall1_noreturn, @function
    .type    syscall2, @function
    .type    syscall3, @function
    .type    syscall4, @function
    .type    syscall5, @function
    .type    syscall6, @function

    # We can't describe calls to external asm as readonly, so just alias
    # these to their non-readonly counterparts.
    .set     syscall0_readonly, syscall0
    .set     syscall1_readonly, syscall1
    .set     syscall2_readonly, syscall2
    .set     syscall3_readonly, syscall3
    .set     syscall4_readonly, syscall4
    .set     syscall5_readonly, syscall5
    .set     syscall6_readonly, syscall6

    .globl   syscall0_readonly
    .globl   syscall0_readonly
    .globl   syscall1_readonly
    .globl   syscall2_readonly
    .globl   syscall3_readonly
    .globl   syscall4_readonly
    .globl   syscall5_readonly
    .globl   syscall6_readonly

    .hidden  syscall0_readonly
    .hidden  syscall0_readonly
    .hidden  syscall1_readonly
    .hidden  syscall2_readonly
    .hidden  syscall3_readonly
    .hidden  syscall4_readonly
    .hidden  syscall5_readonly
    .hidden  syscall6_readonly

    .section .note.GNU-stack,"",@progbits
