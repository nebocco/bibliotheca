#[allow(unused_imports)]
use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};

/// 元
pub trait Element: Sized + Clone + PartialEq {}

/// 二項演算
pub trait Magma: Element {
    fn op(&self, rhs: &Self) -> Self;
}

/// 結合性
pub trait Associative: Magma {}

/// 可換性
pub trait Commutative: Magma {}

/// 単元
pub trait Unital: Magma {
    fn identity() -> Self;
}

/// 可逆性
pub trait Invertible: Magma {
    fn inverse(&self) -> Self;
}

/// 加法単元
pub trait Zero: Element + Add<Output=Self> + AddAssign {
    fn zero() -> Self;
}

/// 乗法単元
pub trait One: Element + Mul<Output=Self> + MulAssign {
    fn one() -> Self;
}

/// 半群
pub trait SemiGroup: Magma + Associative {}
impl<T: Magma + Associative> SemiGroup for T {}

/// モノイド
pub trait Monoid: SemiGroup + Unital {}
impl<T: SemiGroup + Unital> Monoid for T {}

pub trait ComMonoid: Monoid + Commutative {}
impl<T: Monoid + Commutative> ComMonoid for T {}

/// 群
pub trait Group: Monoid + Invertible {}
impl<T: Monoid + Invertible> Group for T {}

pub trait ComGroup: Group + Commutative {}
impl<T: Group + Commutative> ComGroup for T {}

/// 環
pub trait Ring: Element + Zero + One + Sub<Output=Self> + SubAssign + Neg {}
impl<T: Element + Zero + One + Sub<Output=Self> + SubAssign + Neg> Ring for T {}

pub trait ComRing: Ring + Commutative {}
impl<T: Ring + Commutative> ComRing for T {}

/// 体
pub trait Field: Ring + Div<Output=Self> + DivAssign {}

#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}