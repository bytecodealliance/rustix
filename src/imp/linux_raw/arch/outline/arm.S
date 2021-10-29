// arm syscall argument register ordering is the similar to the arm
// userspace argument register ordering except that the syscall number
// (nr) is passed in r7.
//
// nr_last.rs takes care of reordering the nr argument to the end for us,
// so we only need to move nr into r7 and take care of r4 and r5 if needed.

    .file       "arm.S"
    .arch       armv5t

    .section    .text.rsix_syscall0_nr_last,"ax",%progbits
    .p2align    4
    .globl      rsix_syscall0_nr_last
    .weak       rsix_syscall0_nr_last
    .hidden     rsix_syscall0_nr_last
    .type       rsix_syscall0_nr_last, %function
rsix_syscall0_nr_last:
    .fnstart
    .cantunwind
    push    {r7, lr}
    mov     r7, r0
    svc     #0
    pop     {r7, pc}
    .fnend
    .size rsix_syscall0_nr_last, .-rsix_syscall0_nr_last

    .section    .text.rsix_syscall1_nr_last,"ax",%progbits
    .p2align 4
    .globl      rsix_syscall1_nr_last
    .weak       rsix_syscall1_nr_last
    .hidden     rsix_syscall1_nr_last
    .type       rsix_syscall1_nr_last, %function
rsix_syscall1_nr_last:
    .fnstart
    .cantunwind
    push    {r7, lr}
    mov     r7, r1
    svc     #0
    pop     {r7, pc}
    .fnend
    .size rsix_syscall1_nr_last, .-rsix_syscall1_nr_last
    
    .section    .text.rsix_syscall1_noreturn_nr_last,"ax",%progbits
    .p2align    4
    .globl      rsix_syscall1_noreturn_nr_last
    .weak       rsix_syscall1_noreturn_nr_last
    .hidden     rsix_syscall1_noreturn_nr_last
    .type       rsix_syscall1_noreturn_nr_last, %function
rsix_syscall1_noreturn_nr_last:
    .fnstart
    .cantunwind
    // Don't save r7 and lr; this is noreturn, so we'll never restore them.
    mov     r7, r1
    svc     #0
    udf     #16 // Trap instruction
    .fnend
    .size rsix_syscall1_noreturn_nr_last, .-rsix_syscall1_noreturn_nr_last

    .section    .text.rsix_syscall2_nr_last,"ax",%progbits
    .p2align 4
    .globl      rsix_syscall2_nr_last
    .weak       rsix_syscall2_nr_last
    .hidden     rsix_syscall2_nr_last
    .type       rsix_syscall2_nr_last, %function
rsix_syscall2_nr_last:
    .fnstart
    .cantunwind
    push    {r7, lr}
    mov     r7, r2
    svc     #0
    pop     {r7, pc}
    .fnend
    .size rsix_syscall2_nr_last, .-rsix_syscall2_nr_last

    .section    .text.rsix_syscall3_nr_last,"ax",%progbits
    .p2align 4
    .globl      rsix_syscall3_nr_last
    .weak       rsix_syscall3_nr_last
    .hidden     rsix_syscall3_nr_last
    .type       rsix_syscall3_nr_last, %function
rsix_syscall3_nr_last:
    .fnstart
    .cantunwind
    push    {r7, lr}
    mov     r7, r3
    svc     #0
    pop     {r7, pc}
    .fnend
    .size rsix_syscall3_nr_last, .-rsix_syscall3_nr_last

    .section    .text.rsix_syscall4_nr_last,"ax",%progbits
    .p2align 4
    .globl      rsix_syscall4_nr_last
    .weak       rsix_syscall4_nr_last
    .hidden     rsix_syscall4_nr_last
    .type       rsix_syscall4_nr_last, %function
rsix_syscall4_nr_last:
    .fnstart
    .cantunwind
    push    {r7, lr}
    ldr     r7, [sp, #8]
    svc     #0
    pop     {r7, pc}
    .fnend
    .size rsix_syscall4_nr_last, .-rsix_syscall4_nr_last

    .section    .text.rsix_syscall5_nr_last,"ax",%progbits
    .p2align 4
    .globl      rsix_syscall5_nr_last
    .weak       rsix_syscall5_nr_last
    .hidden     rsix_syscall5_nr_last
    .type       rsix_syscall5_nr_last, %function
rsix_syscall5_nr_last:
    .fnstart
    .cantunwind
    push    {r4, r7, r11, lr}
    ldr     r7, [sp, #20]
    ldr     r4, [sp, #16]
    svc     #0
    pop     {r4, r7, r11, pc}
    .fnend
    .size rsix_syscall5_nr_last, .-rsix_syscall5_nr_last

    .section    .text.rsix_syscall6_nr_last,"ax",%progbits
    .p2align 4
    .globl      rsix_syscall6_nr_last
    .weak       rsix_syscall6_nr_last
    .hidden     rsix_syscall6_nr_last
    .type       rsix_syscall6_nr_last, %function
rsix_syscall6_nr_last:
    .fnstart
    .cantunwind
    push    {r4, r5, r7, lr}
    add     r7, sp, #16
    ldm     r7, {r4, r5, r7}
    svc     #0
    pop     {r4, r5, r7, pc}
    .fnend
    .size rsix_syscall6_nr_last, .-rsix_syscall6_nr_last

    .section .note.GNU-stack,"",%progbits
