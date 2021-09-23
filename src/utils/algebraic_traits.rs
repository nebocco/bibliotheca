// ------------ algebraic traits start ------------
use std::marker::Sized;
use std::ops::*;

/// 元
pub trait Element: Sized + Clone + PartialEq {}
impl<T: Sized + Clone + PartialEq> Element for T {}

/// 結合性
pub trait Associative: Magma {}

/// マグマ
pub trait Magma: Element + Add<Output = Self> {}
impl<T: Element + Add<Output = Self>> Magma for T {}

/// 半群
pub trait SemiGroup: Magma + Associative {}
impl<T: Magma + Associative> SemiGroup for T {}

/// モノイド
pub trait Monoid: SemiGroup + Zero {}
impl<T: SemiGroup + Zero> Monoid for T {}

pub trait ComMonoid: Monoid + AddAssign {}
impl<T: Monoid + AddAssign> ComMonoid for T {}

/// 群
pub trait Group: Monoid + Neg<Output = Self> {}
impl<T: Monoid + Neg<Output = Self>> Group for T {}

pub trait ComGroup: Group + ComMonoid {}
impl<T: Group + ComMonoid> ComGroup for T {}

/// 半環
pub trait SemiRing: ComMonoid + Mul<Output = Self> + One {}
impl<T: ComMonoid + Mul<Output = Self> + One> SemiRing for T {}

/// 環
pub trait Ring: ComGroup + SemiRing {}
impl<T: ComGroup + SemiRing> Ring for T {}

pub trait ComRing: Ring + MulAssign {}
impl<T: Ring + MulAssign> ComRing for T {}

/// 体
pub trait Field: ComRing + Div<Output = Self> + DivAssign {}
impl<T: ComRing + Div<Output = Self> + DivAssign> Field for T {}

/// 加法単元
pub trait Zero: Element {
    fn zero() -> Self;
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

/// 乗法単元
pub trait One: Element {
    fn one() -> Self;
    fn is_one(&self) -> bool {
        *self == Self::one()
    }
}

macro_rules! impl_integer {
    ($($T:ty,)*) => {
        $(
            impl Associative for $T {}

            impl Zero for $T {
                fn zero() -> Self { 0 }
                fn is_zero(&self) -> bool { *self == 0 }
            }

            impl<'a> Zero for &'a $T {
                fn zero() -> Self { &0 }
                fn is_zero(&self) -> bool { *self == &0 }
            }

            impl One for $T {
                fn one() -> Self { 1 }
                fn is_one(&self) -> bool { *self == 1 }
            }

            impl<'a> One for &'a $T {
                fn one() -> Self { &1 }
                fn is_one(&self) -> bool { *self == &1 }
            }
        )*
    };
}

impl_integer! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}
// ------------ algebraic traits end ------------
