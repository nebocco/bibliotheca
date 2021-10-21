// ------------ fp start ------------
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter,
    marker::PhantomData,
    ops::*,
};

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
pub struct Mod998244353;
pub type Fp998244353 = Fp<Mod998244353>;
impl Mod for Mod998244353 {
    const MOD: i64 = 998244353;
}

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
pub struct Mod1000000007;
pub type Fp1000000007 = Fp<Mod1000000007>;
impl Mod for Mod1000000007 {
    const MOD: i64 = 1000000007;
}

#[derive(Clone, PartialEq, Copy, Eq, Hash)]
pub struct Fp<T>(i64, PhantomData<T>);
pub trait Mod: Debug + Clone + PartialEq + Copy + Eq + Hash {
    const MOD: i64;
}
impl<T: Mod> Fp<T> {
    pub fn new(mut x: i64) -> Self {
        x %= T::MOD;
        Self::unchecked(if x < 0 { x + T::MOD } else { x })
    }
    pub fn into_inner(self) -> i64 {
        self.0
    }
    pub fn r#mod() -> i64 {
        T::MOD
    }
    pub fn inv(self) -> Self {
        assert_ne!(self.0, 0, "Zero division error");
        let (sign, x) = if self.0 * 2 < T::MOD {
            (1, self.0)
        } else {
            (-1, T::MOD - self.0)
        };
        let (g, _a, b) = ext_gcd(T::MOD, x);
        let ans = sign * b;
        assert_eq!(g, 1);
        Self::unchecked(if ans < 0 { ans + T::MOD } else { ans })
    }
    pub fn frac(x: i64, y: i64) -> Self {
        Fp::new(x) / Fp::new(y)
    }
    pub fn pow(mut self, mut p: u64) -> Self {
        let mut ans = Fp::new(1);
        while p != 0 {
            if p & 1 == 1 {
                ans *= self;
            }
            self *= self;
            p >>= 1;
        }
        ans
    }
    fn unchecked(x: i64) -> Self {
        Self(x, PhantomData)
    }
}
impl<T: Mod> iter::Sum<Fp<T>> for Fp<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: iter::Iterator<Item = Fp<T>>,
    {
        iter.fold(Fp::new(0), Add::add)
    }
}
impl<'a, T: 'a + Mod> iter::Sum<&'a Fp<T>> for Fp<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: iter::Iterator<Item = &'a Fp<T>>,
    {
        iter.fold(Fp::new(0), Add::add)
    }
}
impl<T: Mod> iter::Product<Fp<T>> for Fp<T> {
    fn product<I>(iter: I) -> Self
    where
        I: iter::Iterator<Item = Fp<T>>,
    {
        iter.fold(Self::new(1), Mul::mul)
    }
}
impl<'a, T: 'a + Mod> iter::Product<&'a Fp<T>> for Fp<T> {
    fn product<I>(iter: I) -> Self
    where
        I: iter::Iterator<Item = &'a Fp<T>>,
    {
        iter.fold(Self::new(1), Mul::mul)
    }
}
impl<T: Mod> Debug for Fp<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}
impl<T: Mod> Display for Fp<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

fn ext_gcd(x: i64, y: i64) -> (i64, i64, i64) {
    let (b, g) = {
        let mut x = x;
        let mut y = y;
        let mut u = 0;
        let mut v = 1;
        while x != 0 {
            let q = y / x;
            y -= q * x;
            v -= q * u;
            std::mem::swap(&mut x, &mut y);
            std::mem::swap(&mut u, &mut v);
        }
        (v, y)
    };
    assert_eq!((g - b * y) % x, 0);
    let a = (g - b * y) / x;
    (g, a, b)
}

// ------------ impl arith start ------------
impl<T: Mod> Add for Fp<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let res: i64 = self.0 + rhs.0;
        Self::unchecked(if T::MOD <= res { res - T::MOD } else { res })
    }
}
impl<T: Mod> Sub for Fp<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let res: i64 = self.0 - rhs.0;
        Self::unchecked(if res < 0 { res + T::MOD } else { res })
    }
}
impl<T: Mod> Mul for Fp<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.0 * rhs.0)
    }
}
impl<T: Mod> Div for Fp<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}
impl<M: Mod> Neg for Fp<M> {
    type Output = Self;
    fn neg(self) -> Self {
        if self.0 == 0 {
            Self::unchecked(0)
        } else {
            Self::unchecked(M::MOD - self.0)
        }
    }
}
impl<M: Mod> Neg for &Fp<M> {
    type Output = Fp<M>;
    fn neg(self) -> Self::Output {
        if self.0 == 0 {
            Fp::unchecked(0)
        } else {
            Fp::unchecked(M::MOD - self.0)
        }
    }
}

macro_rules! forward_assign_biop {
    ($(impl $trait:ident, $fn_assign:ident, $fn:ident)*) => {
        $(
            impl<M: Mod> $trait for Fp<M> {
                fn $fn_assign(&mut self, rhs: Self) {
                    *self = self.$fn(rhs);
                }
            }
        )*
    };
}

forward_assign_biop! {
    impl AddAssign, add_assign, add
    impl SubAssign, sub_assign, sub
    impl MulAssign, mul_assign, mul
    impl DivAssign, div_assign, div
}

macro_rules! forward_ref_binop {
    ($(impl $imp:ident, $method:ident)*) => {
        $(
            impl<'a, T: Mod> $imp<Fp<T>> for &'a Fp<T> {
                type Output = Fp<T>;
                fn $method(self, other: Fp<T>) -> Self::Output {
                    $imp::$method(*self, other)
                }
            }

            impl<'a, T: Mod> $imp<&'a Fp<T>> for Fp<T> {
                type Output = Fp<T>;
                fn $method(self, other: &Fp<T>) -> Self::Output {
                    $imp::$method(self, *other)
                }
            }

            impl<'a, T: Mod> $imp<&'a Fp<T>> for &'a Fp<T> {
                type Output = Fp<T>;
                fn $method(self, other: &Fp<T>) -> Self::Output {
                    $imp::$method(*self, *other)
                }
            }
        )*
    };
}

forward_ref_binop! {
    impl Add, add
    impl Sub, sub
    impl Mul, mul
    impl Div, div
}

// ------------ impl arith end ------------

// ------------ fp end ------------

#[cfg(test)]
mod tests {
    use super::{Fp, Mod};
    
    #[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
    struct Mod1009;
    type F1009 = Fp<Mod1009>;
    impl Mod for Mod1009 {
        const MOD: i64 = 1009;
    }

    #[test]
    fn test_inv() {
        type Fp = F1009;
        (1..Fp::r#mod()).for_each(|i| {
            assert_eq!(Fp::new(i), Fp::new(i).inv().inv());
            assert_eq!(Fp::new(1), Fp::new(i).inv() * Fp::new(i));
        });
    }

    #[test]
    fn test_sum() {
        type Fp = F1009;
        let res = (0..10).map(Fp::new).sum::<Fp>();
        assert_eq!(res, Fp::new(45));
        let res = (0..Fp::r#mod()).map(Fp::new).sum::<Fp>();
        assert_eq!(res, Fp::new(0));
    }

    #[test]
    fn test_product() {
        type Fp = F1009;
        let res = (1..=6).map(Fp::new).product::<Fp>();
        assert_eq!(res, Fp::new(720));
        let res = (1..Fp::r#mod()).map(Fp::new).product::<Fp>();
        assert_eq!(res, Fp::new(-1));
    }
}
