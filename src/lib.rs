#![no_std]
#![doc = include_str!("../README.md")]

use core::fmt;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

pub mod iec;
pub mod si;

use iec::{GIB, GiB, KIB, KiB, MIB, MiB, PIB, PiB, TIB, TiB};
use si::{GB, GB_BYTES, KB, KB_BYTES, MB, MB_BYTES, PB, PB_BYTES, TB, TB_BYTES};

/// Smallest unit of data storage
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Bytes(pub u64);

impl Bytes {
    pub const fn new(val: u64) -> Self {
        Self(val)
    }

    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    // IEC Conversions
    pub const fn as_kib(self) -> KiB {
        KiB(self.0 / KIB)
    }

    pub const fn as_mib(self) -> MiB {
        MiB(self.0 / MIB)
    }

    pub const fn as_gib(self) -> GiB {
        GiB(self.0 / GIB)
    }

    pub const fn as_tib(self) -> TiB {
        TiB(self.0 / TIB)
    }

    pub const fn as_pib(self) -> PiB {
        PiB(self.0 / PIB)
    }

    // SI Conversions
    pub const fn as_kb(self) -> KB {
        KB(self.0 / KB_BYTES)
    }

    pub const fn as_mb(self) -> MB {
        MB(self.0 / MB_BYTES)
    }

    pub const fn as_gb(self) -> GB {
        GB(self.0 / GB_BYTES)
    }

    pub const fn as_tb(self) -> TB {
        TB(self.0 / TB_BYTES)
    }

    pub const fn as_pb(self) -> PB {
        PB(self.0 / PB_BYTES)
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} B", self.0)
    }
}

impl Add for Bytes {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl AddAssign for Bytes {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Bytes {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl SubAssign for Bytes {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul<u64> for Bytes {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}
impl Mul<Bytes> for u64 {
    type Output = Bytes;
    fn mul(self, rhs: Bytes) -> Self::Output {
        Bytes(self * rhs.0)
    }
}
impl MulAssign<u64> for Bytes {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 *= rhs;
    }
}

impl Div<u64> for Bytes {
    type Output = Self;
    fn div(self, rhs: u64) -> Self::Output {
        Self(self.0 / rhs)
    }
}
impl DivAssign<u64> for Bytes {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs;
    }
}

impl Div for Bytes {
    type Output = u64;
    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl Rem for Bytes {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl RemAssign for Bytes {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}

// From conversions
impl From<u64> for Bytes {
    fn from(val: u64) -> Self {
        Bytes(val)
    }
}

impl From<usize> for Bytes {
    fn from(val: usize) -> Self {
        Bytes(val as u64)
    }
}

impl From<Bytes> for u64 {
    fn from(bytes: Bytes) -> u64 {
        bytes.0
    }
}

#[macro_export]
macro_rules! impl_comparison {
    ($target:ident, $($other:ident),+) => {
        $(
            impl PartialEq<$other> for $target {
                fn eq(&self, other: &$other) -> bool {
                    $crate::Bytes::from(*self) == $crate::Bytes::from(*other)
                }
            }

            impl PartialEq<$target> for $other {
                fn eq(&self, other: &$target) -> bool {
                    $crate::Bytes::from(*self) == $crate::Bytes::from(*other)
                }
            }

            impl PartialOrd<$other> for $target {
                fn partial_cmp(&self, other: &$other) -> Option<core::cmp::Ordering> {
                    $crate::Bytes::from(*self).partial_cmp(&$crate::Bytes::from(*other))
                }
            }

            impl PartialOrd<$target> for $other {
                fn partial_cmp(&self, other: &$target) -> Option<core::cmp::Ordering> {
                    $crate::Bytes::from(*self).partial_cmp(&$crate::Bytes::from(*other))
                }
            }
        )+
    };
}
