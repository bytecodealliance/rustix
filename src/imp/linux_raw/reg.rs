//! Encapsulation for system call arguments and return values.
//!
//! The inline-asm and outline-asm code paths do some amount of reordering
//! of arguments; to ensure that we don't accidentally misroute an argument
//! or return value, we use distinct types for each argument index and
//! return value.
//!
//! # Safety
//!
//! The `ToAsm` and `FromAsm` traits are unsafe to use; they should only be
//! used by the syscall code which executes actual syscall machine
//! instructions.

#![allow(unsafe_code)]

use super::c;
use super::fd::RawFd;
use core::marker::PhantomData;

pub(super) trait ToAsm: private::Sealed {
    /// Convert `self` to a `usize` ready to be passed to a syscall
    /// machine instruction.
    ///
    /// # Safety
    ///
    /// This should be used immediately before the syscall instruction, and
    /// the returned value shouldn't be used for any other purpose.
    unsafe fn to_asm(self) -> usize;
}

pub(super) trait FromAsm: private::Sealed {
    /// Convert `bits` from a value produced by a syscall machine instruction
    /// into a `Self`.
    ///
    /// # Safety
    ///
    /// This should be used immediately after the syscall instruction, and
    /// the operand value shouldn't be used for any other purpose.
    unsafe fn from_asm(bits: usize) -> Self;
}

// Argument numbers.
pub(super) struct A0 {}
pub(super) struct A1 {}
pub(super) struct A2 {}
pub(super) struct A3 {}
pub(super) struct A4 {}
pub(super) struct A5 {}
#[cfg(target_arch = "x86")]
pub(super) struct SocketArg {}

pub(super) trait ArgNumber: private::Sealed {}
impl ArgNumber for A0 {}
impl ArgNumber for A1 {}
impl ArgNumber for A2 {}
impl ArgNumber for A3 {}
impl ArgNumber for A4 {}
impl ArgNumber for A5 {}
#[cfg(target_arch = "x86")]
impl ArgNumber for SocketArg {}

// Return value numbers.
pub(super) struct R0 {}
pub(super) struct R1 {}

pub(super) trait RetNumber: private::Sealed {}
impl RetNumber for R0 {}
impl RetNumber for R1 {}

/// Syscall arguments use register-sized types. We use a newtype to
/// discourage accidental misuse of the raw integer values.
///
/// Note that it doesn't implement `Clone` or `Copy`; it should be used
/// exactly once. And it has a lifetime to ensure that it doesn't outlive
/// any resources it might be pointing to.
#[repr(transparent)]
pub(super) struct ArgReg<'a, Num: ArgNumber> {
    bits: usize,
    _phantom: PhantomData<(&'a (), Num)>,
}

impl<'a, Num: ArgNumber> ToAsm for ArgReg<'a, Num> {
    #[inline]
    unsafe fn to_asm(self) -> usize {
        self.bits
    }
}

/// Syscall return values use register-sized types. We use a newtype to
/// discourage accidental misuse of the raw integer values.
///
/// Note that it doesn't implement `Clone` or `Copy`; it should be used
/// exactly once. And it has a lifetime to ensure that it doesn't outlive
/// any resources it might be pointing to.
#[repr(transparent)]
pub(super) struct RetReg<Num: RetNumber> {
    bits: usize,
    _phantom: PhantomData<Num>,
}

impl<Num: RetNumber> RetReg<Num> {
    #[inline]
    fn decode(self) -> usize {
        debug_assert!(!(-4095..0).contains(&(self.bits as isize)));
        self.bits
    }

    #[inline]
    pub(super) fn decode_usize(self) -> usize {
        self.decode()
    }

    #[inline]
    pub(super) fn decode_raw_fd(self) -> RawFd {
        let bits = self.decode();
        let raw_fd = bits as RawFd;

        // Converting `raw` to `RawFd` should be lossless.
        debug_assert_eq!(raw_fd as usize, bits);

        raw_fd
    }

    #[inline]
    pub(super) fn decode_c_int(self) -> c::c_int {
        let bits = self.decode();
        let c_int_ = bits as c::c_int;

        // Converting `raw` to `c_int` should be lossless.
        debug_assert_eq!(c_int_ as usize, bits);

        c_int_
    }

    #[inline]
    pub(super) fn decode_c_uint(self) -> c::c_uint {
        let bits = self.decode();
        let c_uint_ = bits as c::c_uint;

        // Converting `raw` to `c_uint` should be lossless.
        debug_assert_eq!(c_uint_ as usize, bits);

        c_uint_
    }

    #[inline]
    pub(super) fn decode_void_star(self) -> *mut c::c_void {
        self.decode() as *mut c::c_void
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub(super) fn decode_u64(self) -> u64 {
        self.decode() as u64
    }

    #[inline]
    pub(super) fn decode_void(self) {
        let _ = self.decode();
    }

    #[inline]
    pub(super) fn decode_error_code(self) -> u16 {
        let bits: usize = self.bits;

        // `raw` must be in `-4095..0`. Linux always returns errors in
        // `-4095..0`, and we double-check it here.
        debug_assert!((-4095..0).contains(&(bits as isize)));

        bits as u16
    }

    #[inline]
    pub(super) fn is_nonzero(&self) -> bool {
        self.bits != 0
    }

    #[inline]
    pub(super) fn is_negative(&self) -> bool {
        (self.bits as isize) < 0
    }

    #[inline]
    pub(super) fn is_in_range(&self, range: core::ops::Range<isize>) -> bool {
        range.contains(&(self.bits as isize))
    }
}

impl<Num: RetNumber> FromAsm for RetReg<Num> {
    #[inline]
    unsafe fn from_asm(bits: usize) -> Self {
        Self {
            bits,
            _phantom: PhantomData,
        }
    }
}

#[repr(transparent)]
pub(super) struct SyscallNumber<'a> {
    nr: usize,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> ToAsm for SyscallNumber<'a> {
    #[inline]
    unsafe fn to_asm(self) -> usize {
        self.nr
    }
}

/// Encode a system call argument as an `ArgReg`.
#[inline]
pub(super) fn raw_arg<'a, Num: ArgNumber>(bits: usize) -> ArgReg<'a, Num> {
    ArgReg {
        bits,
        _phantom: PhantomData,
    }
}

/// Encode a system call number (a `__NR_*` constant) as a `SyscallNumber`.
#[inline]
pub(super) const fn nr<'a>(nr: u32) -> SyscallNumber<'a> {
    SyscallNumber {
        nr: nr as usize,
        _phantom: PhantomData,
    }
}

/// Seal our various traits using the technique documented [here].
///
/// [here]: https://rust-lang.github.io/api-guidelines/future-proofing.html
mod private {
    pub trait Sealed {}

    // Implement for those same types, but no others.
    impl<'a, Num: super::ArgNumber> Sealed for super::ArgReg<'a, Num> {}
    impl<Num: super::RetNumber> Sealed for super::RetReg<Num> {}
    impl<'a> Sealed for super::SyscallNumber<'a> {}
    impl Sealed for super::A0 {}
    impl Sealed for super::A1 {}
    impl Sealed for super::A2 {}
    impl Sealed for super::A3 {}
    impl Sealed for super::A4 {}
    impl Sealed for super::A5 {}
    #[cfg(target_arch = "x86")]
    impl Sealed for super::SocketArg {}
    impl Sealed for super::R0 {}
    impl Sealed for super::R1 {}
}
