#[allow(unused_imports)]
use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};

/// 元
trait Element: Sized + Clone + PartialEq {}

/// 二項演算
trait Magma: Element {
    fn op(&self, rhs: &Self) -> Self;
}

/// 結合性
trait Associative: Magma {}

/// 可換性
trait Commutative: Magma {}

/// 単元
trait Unital: Magma {
    fn identity() -> Self;
}

/// 可逆性
trait Invertible: Magma {
    fn inverse(&self) -> Self;
}

/// 加法単元
trait Zero: Element + Add<Output=Self> + AddAssign {
    fn zero() -> Self;
}

/// 乗法単元
trait One: Element + Mul<Output=Self> + MulAssign {
    fn one() -> Self;
}

/// 半群
trait SemiGroup: Magma + Associative {}
impl<T: Magma + Associative> SemiGroup for T {}

/// モノイド
trait Monoid: SemiGroup + Unital {}
impl<T: SemiGroup + Unital> Monoid for T {}

trait ComMonoid: Monoid + Commutative {}
impl<T: Monoid + Commutative> ComMonoid for T {}

/// 群
trait Group: Monoid + Invertible {}
impl<T: Monoid + Invertible> Group for T {}

trait ComGroup: Group + Commutative {}
impl<T: Group + Commutative> ComGroup for T {}

/// 環
trait Ring: Element + Zero + One + Sub<Output=Self> + SubAssign + Neg {}
impl<T: Element + Zero + One + Sub<Output=Self> + SubAssign + Neg> Ring for T {}

trait ComRing: Ring + Commutative {}
impl<T: Ring + Commutative> ComRing for T {}

/// 体
trait Field: Ring + Div<Output=Self> + DivAssign {}