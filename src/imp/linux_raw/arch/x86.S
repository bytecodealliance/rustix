    .intel_syntax noprefix

    .p2align    4, 0x90
indirect_syscall0:
    .cfi_startproc
    mov    ecx,DWORD PTR [esp+0x4]
    mov    eax,DWORD PTR [esp+0x8]
    call   ecx
    ret
    .cfi_endproc
    .size indirect_syscall0, .-indirect_syscall0

    .p2align    4, 0x90
indirect_syscall1:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    .cfi_offset ebx, -8
    mov    ecx,DWORD PTR [esp+0x8]
    mov    eax,DWORD PTR [esp+0xc]
    mov    ebx,DWORD PTR [esp+0x10]
    call   ecx
    pop    ebx
    .cfi_def_cfa_offset 4
    ret
    .cfi_endproc
    .size indirect_syscall1, .-indirect_syscall1

    .p2align    4, 0x90
indirect_syscall1_noreturn:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    sub    esp,0x8
    .cfi_def_cfa_offset 16
    .cfi_offset ebx, -8
    mov    ecx,DWORD PTR [esp+0x10]
    mov    eax,DWORD PTR [esp+0x14]
    mov    ebx,DWORD PTR [esp+0x18]
    call   ecx
    ud2
    .cfi_endproc
    .size indirect_syscall1_noreturn, .-indirect_syscall1_noreturn

    .p2align    4, 0x90
indirect_syscall2:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    .cfi_offset ebx, -8
    mov    edx,DWORD PTR [esp+0x8]
    mov    eax,DWORD PTR [esp+0xc]
    mov    ebx,DWORD PTR [esp+0x10]
    mov    ecx,DWORD PTR [esp+0x14]
    call   edx
    pop    ebx
    .cfi_def_cfa_offset 4
    ret
    .cfi_endproc
    .size indirect_syscall2, .-indirect_syscall2

    .p2align    4, 0x90
indirect_syscall3:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    push   esi
    .cfi_def_cfa_offset 12
    .cfi_offset esi, -12
    .cfi_offset ebx, -8
    mov    esi,DWORD PTR [esp+0xc]
    mov    eax,DWORD PTR [esp+0x10]
    mov    ebx,DWORD PTR [esp+0x14]
    mov    ecx,DWORD PTR [esp+0x18]
    mov    edx,DWORD PTR [esp+0x1c]
    call   esi
    pop    esi
    .cfi_def_cfa_offset 8
    pop    ebx
    .cfi_def_cfa_offset 4
    ret
    .cfi_endproc
    .size indirect_syscall3, .-indirect_syscall3

    .p2align    4, 0x90
indirect_syscall4:
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
    mov    esi,DWORD PTR [esp+0x24]
    mov    edi,DWORD PTR [esp+0x10]
    mov    eax,DWORD PTR [esp+0x14]
    mov    ebx,DWORD PTR [esp+0x18]
    mov    ecx,DWORD PTR [esp+0x1c]
    mov    edx,DWORD PTR [esp+0x20]
    call   edi
    pop    esi
    .cfi_def_cfa_offset 12
    pop    edi
    .cfi_def_cfa_offset 8
    pop    ebx
    .cfi_def_cfa_offset 4
    ret
    .cfi_endproc
    .size indirect_syscall4, .-indirect_syscall4

    .p2align    4, 0x90
indirect_syscall5:
    .cfi_startproc
    push   ebp
    .cfi_def_cfa_offset 8
    push   ebx
    .cfi_def_cfa_offset 12
    push   edi
    .cfi_def_cfa_offset 16
    push   esi
    .cfi_def_cfa_offset 20
    sub    esp,0xc
    .cfi_def_cfa_offset 32
    .cfi_offset esi, -20
    .cfi_offset edi, -16
    .cfi_offset ebx, -12
    .cfi_offset ebp, -8
    mov    ebp,DWORD PTR [esp+0x20]
    mov    eax,DWORD PTR [esp+0x24]
    mov    esi,DWORD PTR [esp+0x34]
    mov    ebx,DWORD PTR [esp+0x28]
    mov    ecx,DWORD PTR [esp+0x2c]
    mov    edx,DWORD PTR [esp+0x30]
    mov    edi,DWORD PTR [esp+0x38]
    mov    DWORD PTR [esp],ebp
    mov    DWORD PTR [esp+0x4],esi
    mov    DWORD PTR [esp+0x8],eax
    mov    eax,esp
    push   ebp
    push   esi
    push   DWORD PTR [eax]
    mov    esi,DWORD PTR [eax+0x4]
    mov    eax,DWORD PTR [eax+0x8]
    call   DWORD PTR [esp]
    pop    esi
    pop    esi
    pop    ebp
    add    esp,0xc
    .cfi_def_cfa_offset 20
    pop    esi
    .cfi_def_cfa_offset 16
    pop    edi
    .cfi_def_cfa_offset 12
    pop    ebx
    .cfi_def_cfa_offset 8
    pop    ebp
    .cfi_def_cfa_offset 4
    ret
    .cfi_endproc
    .size indirect_syscall5, .-indirect_syscall5

    .p2align    4, 0x90
indirect_syscall6:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    push   edi
    .cfi_def_cfa_offset 12
    sub    esp,0x14
    .cfi_def_cfa_offset 32
    .cfi_offset edi, -12
    .cfi_offset ebx, -8
    mov    eax,DWORD PTR [esp+0x20]
    mov    ebx,DWORD PTR [esp+0x28]
    mov    ecx,DWORD PTR [esp+0x2c]
    mov    edx,DWORD PTR [esp+0x30]
    mov    edi,DWORD PTR [esp+0x38]
    mov    DWORD PTR [esp+0x4],eax
    mov    eax,DWORD PTR [esp+0x34]
    mov    DWORD PTR [esp+0x8],eax
    mov    eax,DWORD PTR [esp+0x3c]
    mov    DWORD PTR [esp+0xc],eax
    mov    eax,DWORD PTR [esp+0x24]
    mov    DWORD PTR [esp+0x10],eax
    lea    eax,[esp+0x4]
    push   ebp
    push   esi
    push   DWORD PTR [eax]
    mov    esi,DWORD PTR [eax+0x4]
    mov    ebp,DWORD PTR [eax+0x8]
    mov    eax,DWORD PTR [eax+0xc]
    call   DWORD PTR [esp]
    pop    esi
    pop    esi
    pop    ebp
    add    esp,0x14
    .cfi_def_cfa_offset 12
    pop    edi
    .cfi_def_cfa_offset 8
    pop    ebx
    .cfi_def_cfa_offset 4
    ret
    .cfi_endproc
    .size indirect_syscall6, .-indirect_syscall6

    .p2align    4, 0x90
syscall0:
    .cfi_startproc
    mov    eax,DWORD PTR [esp+0x4]
    int    0x80
    ret
    .cfi_endproc
    .size syscall0, .-syscall0

    .p2align    4, 0x90
syscall1:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    .cfi_offset ebx, -8
    mov    eax,DWORD PTR [esp+0x8]
    mov    ebx,DWORD PTR [esp+0xc]
    int    0x80
    pop    ebx
    .cfi_def_cfa_offset 4
    ret
    .cfi_endproc
    .size syscall1, .-syscall1

    .p2align    4, 0x90
syscall1_noreturn:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    .cfi_offset ebx, -8
    mov    eax,DWORD PTR [esp+0x8]
    mov    ebx,DWORD PTR [esp+0xc]
    int    0x80
    ud2
    .cfi_endproc
    .size syscall1, .-syscall1

    .p2align    4, 0x90
syscall2:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    .cfi_offset ebx, -8
    mov    eax,DWORD PTR [esp+0x8]
    mov    ebx,DWORD PTR [esp+0xc]
    mov    ecx,DWORD PTR [esp+0x10]
    int    0x80
    pop    ebx
    .cfi_def_cfa_offset 4
    ret
    .cfi_endproc
    .size syscall2, .-syscall2

    .p2align    4, 0x90
syscall3:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    .cfi_offset ebx, -8
    mov    eax,DWORD PTR [esp+0x8]
    mov    ebx,DWORD PTR [esp+0xc]
    mov    ecx,DWORD PTR [esp+0x10]
    mov    edx,DWORD PTR [esp+0x14]
    int    0x80
    pop    ebx
    .cfi_def_cfa_offset 4
    ret
    .cfi_endproc
    .size syscall3, .-syscall3

    .p2align    4, 0x90
syscall4:
    .cfi_startproc
    push   ebx
    .cfi_def_cfa_offset 8
    push   esi
    .cfi_def_cfa_offset 12
    .cfi_offset esi, -12
    .cfi_offset ebx, -8
    mov    esi,DWORD PTR [esp+0x1c]
    mov    eax,DWORD PTR [esp+0xc]
    mov    ebx,DWORD PTR [esp+0x10]
    mov    ecx,DWORD PTR [esp+0x14]
    mov    edx,DWORD PTR [esp+0x18]
    int    0x80
    pop    esi
    .cfi_def_cfa_offset 8
    pop    ebx
    .cfi_def_cfa_offset 4
    ret
    .cfi_endproc
    .size syscall4, .-syscall4

    .p2align    4, 0x90
syscall5:
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
    mov    esi,DWORD PTR [esp+0x20]
    mov    eax,DWORD PTR [esp+0x10]
    mov    ebx,DWORD PTR [esp+0x14]
    mov    ecx,DWORD PTR [esp+0x18]
    mov    edx,DWORD PTR [esp+0x1c]
    mov    edi,DWORD PTR [esp+0x24]
    int    0x80
    pop    esi
    .cfi_def_cfa_offset 12
    pop    edi
    .cfi_def_cfa_offset 8
    pop    ebx
    .cfi_def_cfa_offset 4
    ret
    .cfi_endproc
    .size syscall5, .-syscall5

    .p2align    4, 0x90
syscall6:
    .cfi_startproc
    push   ebp
    .cfi_def_cfa_offset 8
    push   ebx
    .cfi_def_cfa_offset 12
    push   edi
    .cfi_def_cfa_offset 16
    push   esi
    .cfi_def_cfa_offset 20
    sub    esp,0xc
    .cfi_def_cfa_offset 32
    .cfi_offset esi, -20
    .cfi_offset edi, -16
    .cfi_offset ebx, -12
    .cfi_offset ebp, -8
    mov    ebp,DWORD PTR [esp+0x30]
    mov    eax,DWORD PTR [esp+0x20]
    mov    esi,DWORD PTR [esp+0x38]
    mov    ebx,DWORD PTR [esp+0x24]
    mov    ecx,DWORD PTR [esp+0x28]
    mov    edx,DWORD PTR [esp+0x2c]
    mov    edi,DWORD PTR [esp+0x34]
    mov    DWORD PTR [esp],ebp
    mov    DWORD PTR [esp+0x4],esi
    mov    DWORD PTR [esp+0x8],eax
    mov    eax,esp
    push   ebp
    push   esi
    mov    esi,DWORD PTR [eax]
    mov    ebp,DWORD PTR [eax+0x4]
    mov    eax,DWORD PTR [eax+0x8]
    int    0x80
    pop    esi
    pop    ebp
    add    esp,0xc
    .cfi_def_cfa_offset 20
    pop    esi
    .cfi_def_cfa_offset 16
    pop    edi
    .cfi_def_cfa_offset 12
    pop    ebx
    .cfi_def_cfa_offset 8
    pop    ebp
    .cfi_def_cfa_offset 4
    ret
    .cfi_endproc
    .size syscall6, .-syscall6

    .p2align    4, 0x90
int_0x80:
    .cfi_startproc
    int    0x80
    ret
    .cfi_endproc
    .size int_0x80, .-int_0x80

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

    .globl   indirect_syscall0
    .globl   indirect_syscall0
    .globl   indirect_syscall1
    .globl   indirect_syscall1_noreturn
    .globl   indirect_syscall2
    .globl   indirect_syscall3
    .globl   indirect_syscall4
    .globl   indirect_syscall5
    .globl   indirect_syscall6

    .hidden  indirect_syscall0
    .hidden  indirect_syscall0
    .hidden  indirect_syscall1
    .hidden  indirect_syscall1_noreturn
    .hidden  indirect_syscall2
    .hidden  indirect_syscall3
    .hidden  indirect_syscall4
    .hidden  indirect_syscall5
    .hidden  indirect_syscall6

    .type    indirect_syscall0, @function
    .type    indirect_syscall0, @function
    .type    indirect_syscall1, @function
    .type    indirect_syscall1_noreturn, @function
    .type    indirect_syscall2, @function
    .type    indirect_syscall3, @function
    .type    indirect_syscall4, @function
    .type    indirect_syscall5, @function
    .type    indirect_syscall6, @function

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

    .globl   int_0x80

    .hidden  int_0x80

    .type    int_0x80, @function

    .section .note.GNU-stack,"",@progbits
