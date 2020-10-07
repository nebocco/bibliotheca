#![allow(unused_imports)]
use std::marker::Sized;
use num_traits::{One, Zero};
use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};

/// 元
trait Element: Sized + Clone + PartialEq {}
impl<T: Sized + Clone + PartialEq> Element for T {}

/// 結合性
trait Associative: Magma {}

/// 可換性
trait Commutative: Magma {}

/// 可逆性
trait Invertible: Magma {
    fn inverse(&self) -> Self;
}

/// マグマ
trait Magma: Element + Add<Output=Self> + AddAssign {}
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
trait SemiRing: ComMonoid + Mul<Output=Self> + MulAssign + One {}
impl<T: ComMonoid + Mul + MulAssign + One> Semiring for T {}

/// 環
trait Ring {}
impl<T> Ring for T {}

trait ComRing {}
impl<T> ComRing for T {}

/// 体
trait Field: Ring + Div<Output=Self> + DivAssign {}
impl<T: Ring + Div + DivAssign> Field for T {}

#[cfg(test)]
mod tests {
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}