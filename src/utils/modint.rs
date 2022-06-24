use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type Mod998244353 = StaticModInt<M998244353>;
pub type Mod1000000007 = StaticModInt<M1000000007>;

pub enum M998244353 {}
pub enum M1000000007 {}

impl Modulus for M998244353 {
    const MODULUS: u32 = 998_244_353;
}

impl Modulus for M1000000007 {
    const MODULUS: u32 = 1_000_000_007;
}

/// rem := modpow(!MODULUS + 1, !032 >> 2)
pub trait Modulus {
    const MODULUS: u32;
}

#[derive(Eq, PartialEq)]
pub struct StaticModInt<M: Modulus>(u32, PhantomData<fn() -> M>);

impl<M: Modulus> Clone for StaticModInt<M> {
    fn clone(&self) -> Self {
        Self::new_unchecked(self.0)
    }
}

impl<M: Modulus> Copy for StaticModInt<M> {}

impl<M: Modulus> StaticModInt<M> {
    #[inline]
    pub fn new(v: u32) -> Self {
        Self::new_unchecked(v % M::MODULUS)
    }

    #[inline]
    fn new_unchecked(v: u32) -> Self {
        Self(v, PhantomData)
    }

    #[inline]
    pub fn zero() -> Self {
        Self::new_unchecked(0)
    }

    #[inline]
    pub fn one() -> Self {
        Self::new_unchecked(1)
    }

    #[inline]
    pub fn to_inner(self) -> u32 {
        self.0
    }

    pub fn pow(self, mut n: u32) -> Self {
        let mut x = self;
        let mut y = Self::new_unchecked(1);
        while n > 0 {
            if n & 1 == 1 {
                y *= x;
            }
            x *= x;
            n >>= 1;
        }
        y
    }

    pub fn inv(self) -> Self {
        self.pow(M::MODULUS - 2)
    }

    pub fn inv_euclid(self) -> Self {
        let m = M::MODULUS as i64;
        let mut a = self.0 as i64;
        let mut b = m;
        let mut u = 1;
        let mut v = 0;
        while b > 0 {
            let t = a / b;
            a -= t * b;
            u -= t * v;
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut u, &mut v);
        }
        let ans = u.rem_euclid(m) as u32;
        Self::new_unchecked(ans)
    }
}

impl<M: Modulus> Add for StaticModInt<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut sum = self.0 + rhs.0;
        if sum >= M::MODULUS {
            sum -= M::MODULUS;
        }
        Self::new_unchecked(sum)
    }
}

impl<M: Modulus> AddAssign for StaticModInt<M> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<M: Modulus> Sub for StaticModInt<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut res = self.0 + M::MODULUS - rhs.0;
        if res >= M::MODULUS {
            res -= M::MODULUS;
        }
        Self::new_unchecked(res)
    }
}

impl<M: Modulus> SubAssign for StaticModInt<M> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<M: Modulus> Mul for StaticModInt<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new_unchecked((self.0 as u64 * rhs.0 as u64 % M::MODULUS as u64) as u32)
    }
}

impl<M: Modulus> MulAssign for StaticModInt<M> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<M: Modulus> Div for StaticModInt<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self::new_unchecked((self.0 as u64 * rhs.inv().0 as u64 % M::MODULUS as u64) as u32)
    }
}

impl<M: Modulus> DivAssign for StaticModInt<M> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<M: Modulus> Neg for StaticModInt<M> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new_unchecked(if self.0 == 0 { 0 } else { M::MODULUS - self.0 })
    }
}

impl<M: Modulus> From<usize> for StaticModInt<M> {
    fn from(val: usize) -> Self {
        Self::new_unchecked((val % M::MODULUS as usize) as u32)
    }
}

impl<M: Modulus> From<u64> for StaticModInt<M> {
    fn from(val: u64) -> Self {
        Self::new_unchecked((val % M::MODULUS as u64) as u32)
    }
}

impl<M: Modulus> From<i64> for StaticModInt<M> {
    fn from(val: i64) -> Self {
        let m = M::MODULUS as i64;
        Self::new((val % m + m) as u32)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
