use crate::utils::algebraic_traits::{
	ComGroup, ComMonoid, Group, Monoid, SemiRing
};
use num_traits::{one, zero, Zero};
// use std::convert::From;
use std::ops::{Add, AddAssign, Mul, Neg, Shl, Sub};

#[derive(Clone)]
pub struct Polynomial<T: Monoid>{
    pub coef: Vec<T>,
}

impl<T: Monoid> Polynomial<T>{
    pub fn new() -> Self {
        Vec::new().into()
    }

    pub fn bound(&mut self, len: usize){
        if self.coef.len() > len {
            self.coef = self.coef.split_off(len);
        }
	}

    pub fn evaluate<U>(&self, x: &U) -> U
    where
        T: Mul<U, Output = U>,
        U: SemiRing,
    {
        let mut res: U = zero();
        let mut pow: U = one();
        for c in self.coef.iter() {
            res += c.clone() * pow.clone();
            pow = pow * x.clone();
        }
        res
    }
}

impl<T: Monoid> Add<Self> for Polynomial<T> {
    type Output = Self;
    fn add(mut self, mut rhs: Self) -> Self {
        let n = self.coef.len();
        let m = rhs.coef.len();
        if n < m {
            self.coef.extend(vec![zero(); m - n]);
        } else {
            rhs.coef.extend(vec![zero(); n - m]);
        }
        self.coef.iter().zip(rhs.coef.iter()).map(|(a, b)| a.clone() + b.clone()).collect()
    }
}

impl<T: ComMonoid> AddAssign<Self> for Polynomial<T> {
    fn add_assign(&mut self, rhs: Self) {
        let n = self.coef.len();
        let m = rhs.coef.len();
        if n < m {
            self.coef.extend(vec![zero(); m - n]);
        }
        for i in 0..m {
			self.coef[i] += rhs.coef[i].clone();
		}
    }
}

impl<T: SemiRing> Mul for Polynomial<T> {
    type Output = Self;
    fn mul(self, right: Self) -> Self {
        let n = self.coef.len();
        let m = right.coef.len();
        let mut res = vec![zero::<T>(); n + m - 1];
        for (i, a) in self.coef.iter().enumerate() {
            for (j, b) in right.coef.iter().enumerate() {
                res[i + j] += a.clone() * b.clone();
            }
        }
        Self { coef: res }
    }
}

impl<T: Group> Neg for Polynomial<T> {
    type Output = Self;
    fn neg(self) -> Self {
        self.coef.into_iter().map(|s| -s).collect()
    }
}

impl<T: ComGroup> Sub for Polynomial<T> {
    type Output = Self;
    fn sub(self, right: Self) -> Self {
        self + -right
    }
}

impl<T: Monoid> Shl<usize> for Polynomial<T> {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self {
        let mut res = vec![zero(); rhs];
        res.extend(self.coef);
        Self { coef: res }
    }
}

impl<T: Monoid> From<T> for Polynomial<T> {
    fn from(value: T) -> Self {
        vec![value].into()
    }
}

impl<T: Monoid> From<Vec<T>> for Polynomial<T> {
    fn from(coef: Vec<T>) -> Self {
        Self { coef }
    }
}

impl<T: Monoid> std::ops::Index<usize> for Polynomial<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.coef[index]
    }
}

impl<T: Monoid> Zero for Polynomial<T> {
    fn zero() -> Self {
        Self::new()
    }

    fn is_zero(&self) -> bool {
        self.coef.is_empty()
    }
}

impl<T: Monoid> std::iter::FromIterator<T> for Polynomial<T>{
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = T>,
    {
        Vec::from_iter(iter).into()
    }
}