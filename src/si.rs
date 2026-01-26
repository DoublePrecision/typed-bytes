//! SI Units
//! For more information visit <https://en.wikipedia.org/wiki/Binary_prefix>

use crate::Bytes;
use crate::impl_comparison;
use core::fmt;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

/// 1 KB = 1000 Bytes
pub const KB_BYTES: u64 = 1000;
/// 1 MB = 1000 * 1000 Bytes
pub const MB_BYTES: u64 = 1000 * KB_BYTES;
/// 1 GB = 1000 * 1000 * 1000 Bytes
pub const GB_BYTES: u64 = 1000 * MB_BYTES;
/// 1 TB = 1000 * 1000 * 1000 * 1000 Bytes
pub const TB_BYTES: u64 = 1000 * GB_BYTES;
/// 1 PB = 1000 * 1000 * 1000 * 1000 * 1000 Bytes
pub const PB_BYTES: u64 = 1000 * TB_BYTES;

macro_rules! impl_unit {
    ($name:ident, $unit_str:expr, $multiplier:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $name(pub u64);

        impl $name {
            pub const fn new(val: u64) -> Self {
                Self(val)
            }

            pub const fn as_u64(&self) -> u64 {
                self.0
            }

            pub const fn as_bytes(self) -> Bytes {
                Bytes(self.0 * $multiplier)
            }

            pub const fn as_kb(self) -> KB {
                KB((self.0 * $multiplier) / KB_BYTES)
            }

            pub const fn as_mb(self) -> MB {
                MB((self.0 * $multiplier) / MB_BYTES)
            }

            pub const fn as_gb(self) -> GB {
                GB((self.0 * $multiplier) / GB_BYTES)
            }

            pub const fn as_tb(self) -> TB {
                TB((self.0 * $multiplier) / TB_BYTES)
            }

            pub const fn as_pb(self) -> PB {
                PB((self.0 * $multiplier) / PB_BYTES)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{} {}", self.0, $unit_str)
            }
        }

        impl Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }
        impl AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl Sub for $name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }
        impl SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl Mul<u64> for $name {
            type Output = Self;
            fn mul(self, rhs: u64) -> Self::Output {
                Self(self.0 * rhs)
            }
        }
        impl Mul<$name> for u64 {
            type Output = $name;
            fn mul(self, rhs: $name) -> Self::Output {
                $name(self * rhs.0)
            }
        }

        impl MulAssign<u64> for $name {
            fn mul_assign(&mut self, rhs: u64) {
                self.0 *= rhs;
            }
        }

        impl Div<u64> for $name {
            type Output = Self;
            fn div(self, rhs: u64) -> Self::Output {
                Self(self.0 / rhs)
            }
        }
        impl DivAssign<u64> for $name {
            fn div_assign(&mut self, rhs: u64) {
                self.0 /= rhs;
            }
        }

        impl Div for $name {
            type Output = u64;
            fn div(self, rhs: Self) -> Self::Output {
                self.0 / rhs.0
            }
        }

        impl Rem for $name {
            type Output = Self;
            fn rem(self, rhs: Self) -> Self::Output {
                Self(self.0 % rhs.0)
            }
        }

        impl RemAssign for $name {
            fn rem_assign(&mut self, rhs: Self) {
                self.0 %= rhs.0;
            }
        }
    };
}

impl_unit!(KB, "KB", KB_BYTES);
impl_unit!(MB, "MB", MB_BYTES);
impl_unit!(GB, "GB", GB_BYTES);
impl_unit!(TB, "TB", TB_BYTES);
impl_unit!(PB, "PB", PB_BYTES);

// Conversions to Bytes
impl From<KB> for Bytes {
    fn from(val: KB) -> Self {
        Self(val.0 * KB_BYTES)
    }
}

impl From<MB> for Bytes {
    fn from(val: MB) -> Self {
        Self(val.0 * MB_BYTES)
    }
}

impl From<GB> for Bytes {
    fn from(val: GB) -> Self {
        Self(val.0 * GB_BYTES)
    }
}

impl From<TB> for Bytes {
    fn from(val: TB) -> Self {
        Self(val.0 * TB_BYTES)
    }
}

impl From<PB> for Bytes {
    fn from(val: PB) -> Self {
        Self(val.0 * PB_BYTES)
    }
}

// Conversions between SI units
impl From<MB> for KB {
    fn from(val: MB) -> Self {
        Self(val.0 * 1000)
    }
}
impl From<GB> for MB {
    fn from(val: GB) -> Self {
        Self(val.0 * 1000)
    }
}
impl From<TB> for GB {
    fn from(val: TB) -> Self {
        Self(val.0 * 1000)
    }
}
impl From<PB> for TB {
    fn from(val: PB) -> Self {
        Self(val.0 * 1000)
    }
}

impl_comparison!(Bytes, KB, MB, GB, TB, PB);
impl_comparison!(KB, MB, GB, TB, PB);
impl_comparison!(MB, GB, TB, PB);
impl_comparison!(GB, TB, PB);
impl_comparison!(TB, PB);

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use crate::Bytes;
    use std::format;

    #[test]
    fn test_si_constants() {
        assert_eq!(KB_BYTES, 1000);
        assert_eq!(MB_BYTES, 1000 * 1000);
        assert_eq!(GB_BYTES, 1000 * 1000 * 1000);
        assert_eq!(TB_BYTES, 1000 * 1000 * 1000 * 1000);
        assert_eq!(PB_BYTES, 1000 * 1000 * 1000 * 1000 * 1000);
    }

    #[test]
    fn test_si_conversions() {
        assert_eq!(Bytes::from(KB(1)), Bytes(1000));
        assert_eq!(Bytes::from(MB(1)), Bytes(1000 * 1000));
        assert_eq!(KB::from(MB(1)), KB(1000));
    }

    #[test]
    fn test_si_arithmetic() {
        assert_eq!(KB(10) + KB(20), KB(30));
        assert_eq!(MB(50) - MB(20), MB(30));
        assert_eq!(KB(10) * 5, KB(50));
        assert_eq!(KB(100) / 2, KB(50));
    }

    #[test]
    fn test_si_display() {
        assert_eq!(format!("{}", KB(5)), "5 KB");
        assert_eq!(format!("{}", MB(10)), "10 MB");
    }

    #[test]
    fn test_si_comparisons() {
        assert!(MB(1) == KB(1000));
        assert!(MB(1) > KB(100));
    }
}
