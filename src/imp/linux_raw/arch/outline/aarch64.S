// aarch64 syscall argument register ordering is the same as the aarch64
// userspace argument register ordering except that the syscall number
// (nr) is passed in w8.
//
// outline.rs takes care of reordering the nr argument to the end for us,
// so we only need to move nr into w8.

    .file       "aarch64.S"

    .section    .text.rsix_reordered_syscall0,"ax",@progbits
    .p2align    2
    .globl      rsix_reordered_syscall0
    .hidden     rsix_reordered_syscall0
    .type       rsix_reordered_syscall0, @function
rsix_reordered_syscall0:
    .cfi_startproc
    mov    w8, w0
    svc    #0
    ret
    .cfi_endproc
    .size rsix_reordered_syscall0, .-rsix_reordered_syscall0

    .section    .text.rsix_reordered_syscall1,"ax",@progbits
    .p2align    2
    .globl      rsix_reordered_syscall1
    .hidden     rsix_reordered_syscall1
    .type       rsix_reordered_syscall1, @function
rsix_reordered_syscall1:
    .cfi_startproc
    mov    w8, w1
    svc    #0
    ret
    .cfi_endproc
    .size rsix_reordered_syscall1, .-rsix_reordered_syscall1

    .section    .text.rsix_reordered_syscall1_noreturn,"ax",@progbits
    .p2align    2
    .globl      rsix_reordered_syscall1_noreturn
    .hidden     rsix_reordered_syscall1_noreturn
    .type       rsix_reordered_syscall1_noreturn, @function
rsix_reordered_syscall1_noreturn:
    .cfi_startproc
    mov    w8, w1
    svc    #0
    brk    #0x1
    .cfi_endproc
    .size rsix_reordered_syscall1_noreturn, .-rsix_reordered_syscall1_noreturn

    .section    .text.rsix_reordered_syscall2,"ax",@progbits
    .p2align    2
    .globl      rsix_reordered_syscall2
    .hidden     rsix_reordered_syscall2
    .type       rsix_reordered_syscall2, @function
rsix_reordered_syscall2:
    .cfi_startproc
    mov    w8, w2
    svc    #0
    ret
    .cfi_endproc
    .size rsix_reordered_syscall2, .-rsix_reordered_syscall2

    .section    .text.rsix_reordered_syscall3,"ax",@progbits
    .p2align    2
    .globl      rsix_reordered_syscall3
    .hidden     rsix_reordered_syscall3
    .type       rsix_reordered_syscall3, @function
rsix_reordered_syscall3:
    .cfi_startproc
    mov    w8, w3
    svc    #0
    ret
    .cfi_endproc
    .size rsix_reordered_syscall3, .-rsix_reordered_syscall3

    .section    .text.rsix_reordered_syscall4,"ax",@progbits
    .p2align    2
    .globl      rsix_reordered_syscall4
    .hidden     rsix_reordered_syscall4
    .type       rsix_reordered_syscall4, @function
rsix_reordered_syscall4:
    .cfi_startproc
    mov    w8, w4
    svc    #0
    ret
    .cfi_endproc
    .size rsix_reordered_syscall4, .-rsix_reordered_syscall4

    .section    .text.rsix_reordered_syscall5,"ax",@progbits
    .p2align    2
    .globl      rsix_reordered_syscall5
    .hidden     rsix_reordered_syscall5
    .type       rsix_reordered_syscall5, @function
rsix_reordered_syscall5:
    .cfi_startproc
    mov    w8, w5
    svc    #0
    ret
    .cfi_endproc
    .size rsix_reordered_syscall5, .-rsix_reordered_syscall5

    .section    .text.rsix_reordered_syscall6,"ax",@progbits
    .p2align    2
    .globl      rsix_reordered_syscall6
    .hidden     rsix_reordered_syscall6
    .type       rsix_reordered_syscall6, @function
rsix_reordered_syscall6:
    .cfi_startproc
    mov    w8, w6
    svc    #0
    ret
    .cfi_endproc
    .size rsix_reordered_syscall6, .-rsix_reordered_syscall6

    .section .note.GNU-stack,"",@progbits
