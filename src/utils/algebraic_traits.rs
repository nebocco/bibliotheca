#![allow(unused_imports)]
use std::marker::Sized;
use num_traits::{One, Zero};
use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};

/// 元
pub trait Element: Sized + Clone + PartialEq {}
impl<T: Sized + Clone + PartialEq> Element for T {}

/// 結合性
pub trait Associative: Magma {}

/// 可換性
pub trait Commutative: Magma {}

/// 可逆性
pub trait Invertible: Magma {
    fn inverse(&self) -> Self;
}

/// マグマ
pub trait Magma: Element + Add<Output=Self> + AddAssign {}
impl<T: Element + Add<Output=Self> + AddAssign> Magma for T {}

/// 半群
pub trait SemiGroup: Magma + Associative {}
impl<T: Magma + Associative> SemiGroup for T {}

/// モノイド
pub trait Monoid: SemiGroup + Zero {}
impl<T: SemiGroup + Zero> Monoid for T {}

pub trait ComMonoid: Monoid + Commutative {}
impl<T: Monoid + Commutative> ComMonoid for T {}

/// 群
pub trait Group: Monoid + Sub<Output=Self> + SubAssign + Neg<Output=Self> {}
impl<T: Monoid + Sub<Output=Self> + SubAssign + Neg<Output=Self>> Group for T {}

pub trait ComGroup: Group + Commutative {}
impl<T: Group + Commutative> ComGroup for T {}

/// 半環
pub trait SemiRing: ComMonoid + Mul<Output=Self> + MulAssign + One {}
impl<T: ComMonoid + Mul + MulAssign + One> SemiRing for T {}

/// 環
pub trait Ring: ComMonoid + Group {}
impl<T: ComMonoid + Group> Ring for T {}

pub trait ComRing: Ring + Commutative {}
impl<T: Ring + Commutative> ComRing for T {}
// TODO: need fix

/// 体
pub trait Field: Ring + Div<Output=Self> + DivAssign {}
impl<T: Ring + Div<Output=Self> + DivAssign> Field for T {}

#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}