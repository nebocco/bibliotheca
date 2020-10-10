use std::marker::Sized;
use num_traits::{One, Zero};
use std::ops::{Add, Mul, Div, Neg, AddAssign, MulAssign, DivAssign};

/// 元
pub trait Element: Sized + Clone + PartialEq {}
impl<T: Sized + Clone + PartialEq> Element for T {}

/// 結合性
pub trait Associative: Magma {}

/// マグマ
pub trait Magma: Element + Add<Output=Self> {}
impl<T: Element + Add<Output=Self> + AddAssign> Magma for T {}

/// 半群
pub trait SemiGroup: Magma + Associative {}
impl<T: Magma + Associative> SemiGroup for T {}

/// モノイド
pub trait Monoid: SemiGroup + Zero {}
impl<T: SemiGroup + Zero> Monoid for T {}

pub trait ComMonoid: Monoid + AddAssign {}
impl<T: Monoid + AddAssign> ComMonoid for T {}

/// 群
pub trait Group: Monoid + Neg<Output=Self> {}
impl<T: Monoid + Neg<Output=Self>> Group for T {}

pub trait ComGroup: Group + ComMonoid {}
impl<T: Group + ComMonoid> ComGroup for T {}

/// 半環
pub trait SemiRing: ComMonoid + Mul<Output=Self> + One {}
impl<T: ComMonoid + Mul<Output=Self> + One> SemiRing for T {}

/// 環
pub trait Ring: ComGroup + SemiRing {}
impl<T: ComGroup + SemiRing> Ring for T {}

pub trait ComRing: Ring + MulAssign {}
impl<T: Ring + MulAssign> ComRing for T {}

/// 体
pub trait Field: ComRing + Div<Output=Self> + DivAssign {}
impl<T: ComRing + Div<Output=Self> + DivAssign> Field for T {}

#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
    }
}