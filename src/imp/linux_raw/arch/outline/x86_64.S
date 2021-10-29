#if !defined(__LP64__)
#error "x32 is not yet supported"
#endif

// x86-64 syscall argument register ordering is the same as the x86-64
// userspace argument register ordering except that a3 is passed in r10
// instead of rcx, and the syscall number (nr) is passed in eax.
//
// outline.rs takes care of reordering the nr argument to the end for us,
// so we only need to move nr into eax and move rcx into r10 as needed.

    .file       "x86_64.S"
    .intel_syntax noprefix

    .section    .text.rsix_syscall0_nr_last,"ax",@progbits
    .p2align    4
    .globl      rsix_syscall0_nr_last
    .weak       rsix_syscall0_nr_last
    .hidden     rsix_syscall0_nr_last
    .type       rsix_syscall0_nr_last, @function
rsix_syscall0_nr_last:
    .cfi_startproc
    mov    eax,edi
    syscall
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
    mov    eax,esi
    syscall
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
    mov    eax,esi
    syscall
    ud2
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
    mov    eax,edx
    syscall
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
    mov    eax,ecx
    syscall
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
    mov    eax,r8d
    mov    r10,rcx
    syscall
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
    mov    eax,r9d
    mov    r10,rcx
    syscall
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
    mov    eax,DWORD PTR [rsp+0x8]
    mov    r10,rcx
    syscall
    ret
    .cfi_endproc
    .size rsix_syscall6_nr_last, .-rsix_syscall6_nr_last

    .section .note.GNU-stack,"",@progbits
