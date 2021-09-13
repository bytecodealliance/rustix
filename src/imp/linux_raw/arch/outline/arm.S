// arm syscall argument register ordering is the similar to the arm
// userspace argument register ordering except that the syscall number
// (nr) is passed in r7.
//
// arm.rs takes care of reordering the nr argument to the end for us,
// so we only need to take care of sp by moving it to r4, r5 and r7.

    .section    .text.rsix_reordered_syscall,"ax",%progbits
    .p2align    4
rsix_reordered_syscall:
rsix_reordered_syscall0:
rsix_reordered_syscall1:
rsix_reordered_syscall2:
rsix_reordered_syscall3:
rsix_reordered_syscall4:
rsix_reordered_syscall5:
rsix_reordered_syscall6:
    .fnstart
    .cantunwind
    push    {r4, r5, r7, lr}
    ldr     r7, [sp, #28]
    ldr     r4, [sp, #16]
    ldr     r5, [sp, #20]
    svc     #0
    pop     {r4, r5, r7, pc}
    .fnend
    .size rsix_reordered_syscall, .-rsix_reordered_syscall

    .section    .text.rsix_reordered_syscall1_noreturn,"ax",%progbits
    .p2align    4
rsix_reordered_syscall1_noreturn:
    .fnstart
    .cantunwind
    push    {r7, lr}
    ldr     r7, [sp, #28]
    svc     #0
    .inst   0xe7f001f0 // Trap instruction
    .fnend
    .size rsix_reordered_syscall1_noreturn, .-rsix_reordered_syscall1_noreturn

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

    .type    rsix_reordered_syscall0, %function
    .type    rsix_reordered_syscall1, %function
    .type    rsix_reordered_syscall1_noreturn, %function
    .type    rsix_reordered_syscall2, %function
    .type    rsix_reordered_syscall3, %function
    .type    rsix_reordered_syscall4, %function
    .type    rsix_reordered_syscall5, %function
    .type    rsix_reordered_syscall6, %function

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

    .section .note.GNU-stack,"",%progbits
