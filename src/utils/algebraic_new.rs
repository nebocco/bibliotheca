#![allow(unused_imports)]
use std::marker::Sized;
use num_traits::{One, Zero};
use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};

/// 結合性
trait Associative: Magma {}

/// 可換性
trait Commutative: Magma {}

/// 可逆性
trait Invertible: Magma {
    fn inverse(&self) -> Self;
}

/// 元
trait Element: Sized + Clone + PartialEq {}
impl<T: Sized + Clone + PartialEq> Element for T {}

/// マグマ
trait Magma: Element + Add<Output = Self> + AddAssign {}
impl<T: Element + Add + AddAssign> Magma for T {}

/// 半群
trait SemiGroup: Magma + Associative {}
impl<T: Magma + Associative> SemiGroup for T {}

/// モノイド
trait Monoid: SemiGroup + Zero {}
impl<T: SemiGroup + Zero> Monoid for T {}

trait ComMonoid: Monoid + Commutative {}
impl<T: Monoid + Commutative> ComMonoid for T {}

/// 群
trait Group: Monoid + Sub<Output=Self> + SubAssign + Neg<Output=Self> {}
impl<T: Monoid + Sub + SubAssign + Neg> Group for T {}

trait ComGroup: Group + Commutative {}
impl<T: Group + Commutative> ComGroup for T {}

/// 半環
trait SemiRing

/// 環
trait Ring: Element + Zero + One + Sub<Output=Self> + SubAssign + Neg {}
impl<T: Element + Zero + One + Sub<Output=Self> + SubAssign + Neg> Ring for T {}

trait ComRing: Ring + Commutative {}
impl<T: Ring + Commutative> ComRing for T {}

/// 体
trait Field: Ring + Div<Output=Self> + DivAssign {}

#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}