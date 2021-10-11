// On x86 we use the "fastcall" convention which passes the first two
// arguments in ecx and edx. Outline.rs reorders the arguments to put
// a1 and a2 in those registers so they we don't have to move them to
// set up the kernel convention.
//
// "fastcall" expects callee to pop argument stack space, so we use
// `ret imm` instructions to clean up the stack. We don't need callee
// cleanup per se, it just comes along with using "fastcall".

    .file       "x86.S"
    .intel_syntax noprefix

    .section    .text.rsix_reordered_indirect_syscall0,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_indirect_syscall0
    .hidden     rsix_reordered_indirect_syscall0
    .type       rsix_reordered_indirect_syscall0, @function
rsix_reordered_indirect_syscall0:
    .cfi_startproc
    mov    eax,ecx
    call   edx
    ret
    .cfi_endproc
    .size rsix_reordered_indirect_syscall0, .-rsix_reordered_indirect_syscall0

    .section    .text.rsix_reordered_indirect_syscall1,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_indirect_syscall1
    .hidden     rsix_reordered_indirect_syscall1
    .type       rsix_reordered_indirect_syscall1, @function
rsix_reordered_indirect_syscall1:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    .cfi_offset ebx, -8
    mov    ebx,ecx
    mov    eax,edx
    call   DWORD PTR [esp+0x8]
    pop    ebx
    .cfi_def_cfa_offset 4
    ret    0x4
    .cfi_endproc
    .size rsix_reordered_indirect_syscall1, .-rsix_reordered_indirect_syscall1

    .section    .text.rsix_reordered_indirect_syscall1_noreturn,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_indirect_syscall1_noreturn
    .hidden     rsix_reordered_indirect_syscall1_noreturn
    .type       rsix_reordered_indirect_syscall1_noreturn, @function
rsix_reordered_indirect_syscall1_noreturn:
    .cfi_startproc
    mov    ebx,ecx
    mov    eax,edx
    call   DWORD PTR [esp+0x4]
    ud2
    .cfi_endproc
    .size rsix_reordered_indirect_syscall1_noreturn, .-rsix_reordered_indirect_syscall1_noreturn

    .section    .text.rsix_reordered_indirect_syscall2,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_indirect_syscall2
    .hidden     rsix_reordered_indirect_syscall2
    .type       rsix_reordered_indirect_syscall2, @function
rsix_reordered_indirect_syscall2:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    .cfi_offset ebx, -8
    mov    ebx,edx
    mov    eax,DWORD PTR [esp+0x8]
    call   DWORD PTR [esp+0xc]
    pop    ebx
    .cfi_def_cfa_offset 4
    ret    0x8
    .cfi_endproc
    .size rsix_reordered_indirect_syscall2, .-rsix_reordered_indirect_syscall2

    .section    .text.rsix_reordered_indirect_syscall3,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_indirect_syscall3
    .hidden     rsix_reordered_indirect_syscall3
    .type       rsix_reordered_indirect_syscall3, @function
rsix_reordered_indirect_syscall3:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    .cfi_offset ebx, -8
    mov    ebx,DWORD PTR [esp+0x8]
    mov    eax,DWORD PTR [esp+0xc]
    call   DWORD PTR [esp+0x10]
    pop    ebx
    .cfi_def_cfa_offset 4
    ret    0xc
    .cfi_endproc
    .size rsix_reordered_indirect_syscall3, .-rsix_reordered_indirect_syscall3

    .section    .text.rsix_reordered_indirect_syscall4,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_indirect_syscall4
    .hidden     rsix_reordered_indirect_syscall4
    .type       rsix_reordered_indirect_syscall4, @function
rsix_reordered_indirect_syscall4:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    push   esi
    .cfi_def_cfa_offset 12
    .cfi_offset esi, -12
    .cfi_offset ebx, -8
    mov    ebx,DWORD PTR [esp+0xc]
    mov    esi,DWORD PTR [esp+0x10]
    mov    eax,DWORD PTR [esp+0x14]
    call   DWORD PTR [esp+0x18]
    pop    esi
    .cfi_def_cfa_offset 8
    pop    ebx
    .cfi_def_cfa_offset 4
    ret    0x10
    .cfi_endproc
    .size rsix_reordered_indirect_syscall4, .-rsix_reordered_indirect_syscall4

    .section    .text.rsix_reordered_indirect_syscall5,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_indirect_syscall5
    .hidden     rsix_reordered_indirect_syscall5
    .type       rsix_reordered_indirect_syscall5, @function
rsix_reordered_indirect_syscall5:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    push   esi
    .cfi_def_cfa_offset 12
    push   edi
    .cfi_def_cfa_offset 16
    .cfi_offset edi, -16
    .cfi_offset esi, -12
    .cfi_offset ebx, -8
    mov    ebx,DWORD PTR [esp+0x10]
    mov    esi,DWORD PTR [esp+0x14]
    mov    edi,DWORD PTR [esp+0x18]
    mov    eax,DWORD PTR [esp+0x1c]
    call   DWORD PTR [esp+0x20]
    pop    edi
    .cfi_def_cfa_offset 12
    pop    esi
    .cfi_def_cfa_offset 8
    pop    ebx
    .cfi_def_cfa_offset 4
    ret    0x14
    .cfi_endproc
    .size rsix_reordered_indirect_syscall5, .-rsix_reordered_indirect_syscall5

    .section    .text.rsix_reordered_indirect_syscall6,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_indirect_syscall6
    .hidden     rsix_reordered_indirect_syscall6
    .type       rsix_reordered_indirect_syscall6, @function
rsix_reordered_indirect_syscall6:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    push   esi
    .cfi_def_cfa_offset 12
    push   edi
    .cfi_def_cfa_offset 16
    push   ebp
    .cfi_def_cfa_offset 20
    .cfi_offset ebp, -20
    .cfi_offset edi, -16
    .cfi_offset esi, -12
    .cfi_offset ebx, -8
    mov    ebx,DWORD PTR [esp+0x14]
    mov    esi,DWORD PTR [esp+0x18]
    mov    edi,DWORD PTR [esp+0x1c]
    mov    ebp,DWORD PTR [esp+0x20]
    mov    eax,DWORD PTR [esp+0x24]
    call   DWORD PTR [esp+0x28]
    pop    ebp
    .cfi_def_cfa_offset 16
    pop    edi
    .cfi_def_cfa_offset 12
    pop    esi
    .cfi_def_cfa_offset 8
    pop    ebx
    .cfi_def_cfa_offset 4
    ret    0x18
    .cfi_endproc
    .size rsix_reordered_indirect_syscall6, .-rsix_reordered_indirect_syscall6

    .section    .text.rsix_reordered_syscall0,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_syscall0
    .hidden     rsix_reordered_syscall0
    .type       rsix_reordered_syscall0, @function
rsix_reordered_syscall0:
    .cfi_startproc
    mov    eax,ecx
    int    0x80
    ret
    .cfi_endproc
    .size rsix_reordered_syscall0, .-rsix_reordered_syscall0

    .section    .text.rsix_reordered_syscall1,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_syscall1
    .hidden     rsix_reordered_syscall1
    .type       rsix_reordered_syscall1, @function
rsix_reordered_syscall1:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    .cfi_offset ebx, -8
    mov    eax,edx
    mov    ebx,ecx
    int    0x80
    pop    ebx
    .cfi_def_cfa_offset 4
    ret
    .cfi_endproc
    .size rsix_reordered_syscall1, .-rsix_reordered_syscall1

    .section    .text.rsix_reordered_syscall1_noreturn,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_syscall1_noreturn
    .hidden     rsix_reordered_syscall1_noreturn
    .type       rsix_reordered_syscall1_noreturn, @function
rsix_reordered_syscall1_noreturn:
    .cfi_startproc
    mov    eax,edx
    mov    ebx,ecx
    int    0x80
    ud2
    .cfi_endproc
    .size rsix_reordered_syscall1_noreturn, .-rsix_reordered_syscall1_noreturn

    .section    .text.rsix_reordered_syscall2,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_syscall2
    .hidden     rsix_reordered_syscall2
    .type       rsix_reordered_syscall2, @function
rsix_reordered_syscall2:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    .cfi_offset ebx, -8
    mov    ebx,edx
    mov    eax,DWORD PTR [esp+0x8]
    int    0x80
    pop    ebx
    .cfi_def_cfa_offset 4
    ret    0x4
    .cfi_endproc
    .size rsix_reordered_syscall2, .-rsix_reordered_syscall2

    .section    .text.rsix_reordered_syscall3,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_syscall3
    .hidden     rsix_reordered_syscall3
    .type       rsix_reordered_syscall3, @function
rsix_reordered_syscall3:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    .cfi_offset ebx, -8
    mov    ebx,DWORD PTR [esp+0x8]
    mov    eax,DWORD PTR [esp+0xc]
    int    0x80
    pop    ebx
    .cfi_def_cfa_offset 4
    ret    0x8
    .cfi_endproc
    .size rsix_reordered_syscall3, .-rsix_reordered_syscall3

    .section    .text.rsix_reordered_syscall4,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_syscall4
    .hidden     rsix_reordered_syscall4
    .type       rsix_reordered_syscall4, @function
rsix_reordered_syscall4:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    push   esi
    .cfi_def_cfa_offset 12
    .cfi_offset esi, -12
    .cfi_offset ebx, -8
    mov    ebx,DWORD PTR [esp+0xc]
    mov    esi,DWORD PTR [esp+0x10]
    mov    eax,DWORD PTR [esp+0x14]
    int    0x80
    pop    esi
    .cfi_def_cfa_offset 8
    pop    ebx
    .cfi_def_cfa_offset 4
    ret    0xc
    .cfi_endproc
    .size rsix_reordered_syscall4, .-rsix_reordered_syscall4

    .section    .text.rsix_reordered_syscall5,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_syscall5
    .hidden     rsix_reordered_syscall5
    .type       rsix_reordered_syscall5, @function
rsix_reordered_syscall5:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    push   edi
    .cfi_def_cfa_offset 12
    push   esi
    .cfi_def_cfa_offset 16
    .cfi_offset esi, -16
    .cfi_offset edi, -12
    .cfi_offset ebx, -8
    mov    ebx,DWORD PTR [esp+0x10]
    mov    esi,DWORD PTR [esp+0x14]
    mov    edi,DWORD PTR [esp+0x18]
    mov    eax,DWORD PTR [esp+0x1c]
    int    0x80
    pop    esi
    .cfi_def_cfa_offset 12
    pop    edi
    .cfi_def_cfa_offset 8
    pop    ebx
    .cfi_def_cfa_offset 4
    ret    0x10
    .cfi_endproc
    .size rsix_reordered_syscall5, .-rsix_reordered_syscall5

    .section    .text.rsix_reordered_syscall6,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_syscall6
    .hidden     rsix_reordered_syscall6
    .type       rsix_reordered_syscall6, @function
rsix_reordered_syscall6:
    .cfi_startproc
    push   ebp
    .cfi_def_cfa_offset 8
    push   ebx
    .cfi_def_cfa_offset 12
    push   edi
    .cfi_def_cfa_offset 16
    push   esi
    .cfi_def_cfa_offset 20
    .cfi_offset esi, -20
    .cfi_offset edi, -16
    .cfi_offset ebx, -12
    .cfi_offset ebp, -8
    mov    ebx,DWORD PTR [esp+0x14]
    mov    esi,DWORD PTR [esp+0x18]
    mov    edi,DWORD PTR [esp+0x1c]
    mov    ebp,DWORD PTR [esp+0x20]
    mov    eax,DWORD PTR [esp+0x24]
    int    0x80
    pop    esi
    .cfi_def_cfa_offset 16
    pop    edi
    .cfi_def_cfa_offset 12
    pop    ebx
    .cfi_def_cfa_offset 8
    pop    ebp
    .cfi_def_cfa_offset 4
    ret    0x14
    .cfi_endproc
    .size rsix_reordered_syscall6, .-rsix_reordered_syscall6

    .section    .text.rsix_int_0x80,"ax",@progbits
    .p2align    4
    .globl      rsix_int_0x80
    .hidden     rsix_int_0x80
    .type       rsix_int_0x80, @function
rsix_int_0x80:
    .cfi_startproc
    int    0x80
    ret
    .cfi_endproc
    .size rsix_int_0x80, .-rsix_int_0x80

    .section .note.GNU-stack,"",@progbits
