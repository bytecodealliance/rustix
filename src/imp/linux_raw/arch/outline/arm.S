// arm syscall argument register ordering is the similar to the arm
// userspace argument register ordering except that the syscall number
// (nr) is passed in r7.
//
// nr_last.rs takes care of reordering the nr argument to the end for us,
// so we only need to move nr into r7 and take care of r4 and r5 if needed.

    .file       "arm.S"

    .section    .text.rsix_reordered_syscall0,"ax",%progbits
    .p2align    4
    .globl      rsix_reordered_syscall0
    .hidden     rsix_reordered_syscall0
    .type       rsix_reordered_syscall0, %function
rsix_reordered_syscall0:
    .fnstart
    .cantunwind
    push    {r7, lr}
    mov     r7, r0
    svc     #0
    pop     {r7, pc}
    .fnend
    .size rsix_reordered_syscall0, .-rsix_reordered_syscall0

    .section    .text.rsix_reordered_syscall1,"ax",%progbits
    .p2align 4
    .globl      rsix_reordered_syscall1
    .hidden     rsix_reordered_syscall1
    .type       rsix_reordered_syscall1, %function
rsix_reordered_syscall1:
    .fnstart
    .cantunwind
    push    {r7, lr}
    mov     r7, r1
    svc     #0
    pop     {r7, pc}
    .fnend
    .size rsix_reordered_syscall1, .-rsix_reordered_syscall1
    
    .section    .text.rsix_reordered_syscall1_noreturn,"ax",%progbits
    .p2align    4
    .globl      rsix_reordered_syscall1_noreturn
    .hidden     rsix_reordered_syscall1_noreturn
    .type       rsix_reordered_syscall1_noreturn, %function
rsix_reordered_syscall1_noreturn:
    .fnstart
    .cantunwind
    // Don't save r7 and lr; this is noreturn, so we'll never restore them.
    mov     r7, r1
    svc     #0
    udf     #16 // Trap instruction
    .fnend
    .size rsix_reordered_syscall1_noreturn, .-rsix_reordered_syscall1_noreturn

    .section    .text.rsix_reordered_syscall2,"ax",%progbits
    .p2align 4
    .globl      rsix_reordered_syscall2
    .hidden     rsix_reordered_syscall2
    .type       rsix_reordered_syscall2, %function
rsix_reordered_syscall2:
    .fnstart
    .cantunwind
    push    {r7, lr}
    mov     r7, r2
    svc     #0
    pop     {r7, pc}
    .fnend
    .size rsix_reordered_syscall2, .-rsix_reordered_syscall2

    .section    .text.rsix_reordered_syscall3,"ax",%progbits
    .p2align 4
    .globl      rsix_reordered_syscall3
    .hidden     rsix_reordered_syscall3
    .type       rsix_reordered_syscall3, %function
rsix_reordered_syscall3:
    .fnstart
    .cantunwind
    push    {r7, lr}
    mov     r7, r3
    svc     #0
    pop     {r7, pc}
    .fnend
    .size rsix_reordered_syscall3, .-rsix_reordered_syscall3

    .section    .text.rsix_reordered_syscall4,"ax",%progbits
    .p2align 4
    .globl      rsix_reordered_syscall4
    .hidden     rsix_reordered_syscall4
    .type       rsix_reordered_syscall4, %function
rsix_reordered_syscall4:
    .fnstart
    .cantunwind
    push    {r7, lr}
    ldr     r7, [sp, #8]
    svc     #0
    pop     {r7, pc}
    .fnend
    .size rsix_reordered_syscall4, .-rsix_reordered_syscall4

    .section    .text.rsix_reordered_syscall5,"ax",%progbits
    .p2align 4
    .globl      rsix_reordered_syscall5
    .hidden     rsix_reordered_syscall5
    .type       rsix_reordered_syscall5, %function
rsix_reordered_syscall5:
    .fnstart
    .cantunwind
    push    {r4, r7, r11, lr}
    ldr     r7, [sp, #20]
    ldr     r4, [sp, #16]
    svc     #0
    pop     {r4, r7, r11, pc}
    .fnend
    .size rsix_reordered_syscall5, .-rsix_reordered_syscall5

    .section    .text.rsix_reordered_syscall6,"ax",%progbits
    .p2align 4
    .globl      rsix_reordered_syscall6
    .hidden     rsix_reordered_syscall6
    .type       rsix_reordered_syscall6, %function
rsix_reordered_syscall6:
    .fnstart
    .cantunwind
    push    {r4, r5, r7, lr}
    add     r7, sp, #16
    ldm     r7, {r4, r5, r7}
    svc     #0
    pop     {r4, r5, r7, pc}
    .fnend
    .size rsix_reordered_syscall6, .-rsix_reordered_syscall6

    .section .note.GNU-stack,"",%progbits
