//! IEC units (binary)
//! For more information visit <https://en.wikipedia.org/wiki/Binary_prefix>

use crate::Bytes;
use crate::impl_comparison;
use core::fmt;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

/// 1 KiB = 1024 Bytes
pub const KIB: u64 = 1024;
/// 1 MiB = 1024 * 1024 Bytes
pub const MIB: u64 = 1024 * KIB;
/// 1 GiB = 1024 * 1024 * 1024 Bytes
pub const GIB: u64 = 1024 * MIB;
/// 1 TiB = 1024 * 1024 * 1024 * 1024 Bytes
pub const TIB: u64 = 1024 * GIB;
/// 1 PiB = 1024 * 1024 * 1024 * 1024 * 1024 Bytes
pub const PIB: u64 = 1024 * TIB;

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

            pub const fn as_kib(self) -> KiB {
                KiB((self.0 * $multiplier) / KIB)
            }

            pub const fn as_mib(self) -> MiB {
                MiB((self.0 * $multiplier) / MIB)
            }

            pub const fn as_gib(self) -> GiB {
                GiB((self.0 * $multiplier) / GIB)
            }

            pub const fn as_tib(self) -> TiB {
                TiB((self.0 * $multiplier) / TIB)
            }

            pub const fn as_pib(self) -> PiB {
                PiB((self.0 * $multiplier) / PIB)
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

impl_unit!(KiB, "KiB", KIB);
impl_unit!(MiB, "MiB", MIB);
impl_unit!(GiB, "GiB", GIB);
impl_unit!(TiB, "TiB", TIB);
impl_unit!(PiB, "PiB", PIB);

// Conversions to Bytes
impl From<KiB> for Bytes {
    fn from(val: KiB) -> Self {
        Self(val.0 * KIB)
    }
}

impl From<MiB> for Bytes {
    fn from(val: MiB) -> Self {
        Self(val.0 * MIB)
    }
}

impl From<GiB> for Bytes {
    fn from(val: GiB) -> Self {
        Self(val.0 * GIB)
    }
}

impl From<TiB> for Bytes {
    fn from(val: TiB) -> Self {
        Self(val.0 * TIB)
    }
}

impl From<PiB> for Bytes {
    fn from(val: PiB) -> Self {
        Self(val.0 * PIB)
    }
}

impl From<MiB> for KiB {
    fn from(val: MiB) -> Self {
        Self(val.0 * 1024)
    }
}

impl From<GiB> for MiB {
    fn from(val: GiB) -> Self {
        Self(val.0 * 1024)
    }
}

impl From<TiB> for GiB {
    fn from(val: TiB) -> Self {
        Self(val.0 * 1024)
    }
}

impl From<PiB> for TiB {
    fn from(val: PiB) -> Self {
        Self(val.0 * 1024)
    }
}

impl_comparison!(Bytes, KiB, MiB, GiB, TiB, PiB);
impl_comparison!(KiB, MiB, GiB, TiB, PiB);
impl_comparison!(MiB, GiB, TiB, PiB);
impl_comparison!(GiB, TiB, PiB);
impl_comparison!(TiB, PiB);

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use crate::Bytes;
    use std::format;

    #[test]
    fn test_unit_constants() {
        assert_eq!(KIB, 1024);
        assert_eq!(MIB, 1024 * 1024);
        assert_eq!(GIB, 1024 * 1024 * 1024);
        assert_eq!(TIB, 1024 * 1024 * 1024 * 1024);
        assert_eq!(PIB, 1024 * 1024 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_new_and_as_u64() {
        let b = Bytes::new(100);
        assert_eq!(b.as_u64(), 100);

        let k = KiB::new(5);
        assert_eq!(k.as_u64(), 5);
    }

    #[test]
    fn test_conversions() {
        // Conversions to Bytes
        assert_eq!(Bytes::from(KiB(1)), Bytes(1024));
        assert_eq!(Bytes::from(MiB(1)), Bytes(1024 * 1024));
        assert_eq!(Bytes::from(GiB(1)), Bytes(1024 * 1024 * 1024));
        assert_eq!(Bytes::from(TiB(1)), Bytes(1024 * 1024 * 1024 * 1024));
        assert_eq!(Bytes::from(PiB(1)), Bytes(1024 * 1024 * 1024 * 1024 * 1024));

        // Internal inter-conversions
        assert_eq!(KiB::from(MiB(1)), KiB(1024));
        assert_eq!(MiB::from(GiB(1)), MiB(1024));
        assert_eq!(GiB::from(TiB(1)), GiB(1024));
        assert_eq!(TiB::from(PiB(1)), TiB(1024));
        assert_eq!(GiB::from(GiB(1)), Bytes(1024 * 1024 * 1024));
    }

    #[test]
    fn test_arithmetic() {
        // Add
        assert_eq!(Bytes(10) + Bytes(20), Bytes(30));
        let mut b = Bytes(10);
        b += Bytes(20);
        assert_eq!(b, Bytes(30));

        // Sub
        assert_eq!(MiB(50) - MiB(20), MiB(30));
        let mut m = MiB(50);
        m -= MiB(20);
        assert_eq!(m, MiB(30));

        // Mul (scalar)
        assert_eq!(Bytes(10) * 5, Bytes(50));
        assert_eq!(5 * Bytes(10), Bytes(50));
        let mut b = Bytes(10);
        b *= 5;
        assert_eq!(b, Bytes(50));

        // Div (scalar)
        assert_eq!(KiB(100) / 2, KiB(50));
        let mut k = KiB(100);
        k /= 2;
        assert_eq!(k, KiB(50));

        // Div (self)
        assert_eq!(Bytes(100) / Bytes(20), 5);

        // Rem
        assert_eq!(Bytes(10) % Bytes(3), Bytes(1));
        let mut r = Bytes(10);
        r %= Bytes(3);
        assert_eq!(r, Bytes(1));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Bytes(100)), "100 B");
        assert_eq!(format!("{}", KiB(5)), "5 KiB");
        assert_eq!(format!("{}", MiB(10)), "10 MiB");
        assert_eq!(format!("{}", GiB(2)), "2 GiB");
        assert_eq!(format!("{}", TiB(3)), "3 TiB");
        assert_eq!(format!("{}", PiB(4)), "4 PiB");
    }

    #[test]
    fn test_comparisons() {
        assert!(Bytes(1025) > KiB(1));
        assert!(Bytes(1023) < KiB(1));
        assert!(MiB(1) == KiB(1024));
        assert!(MiB(1) > KiB(100));
        assert!(GiB(1) > MiB(100));
        assert!(TiB(1) > GiB(100));
        assert!(PiB(1) > TiB(100));
    }
}
