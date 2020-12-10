use super::Polynomial;
use std::ops::*;
use crate::utils::{
    algebraic_traits::{ Associative, One, Zero },
    fp::{ Fp, Mod },
    transform::*,
};

impl<T: Mod> Associative for Polynomial<T> {}

impl<T: Mod> Zero for Polynomial<T> {
    fn zero() -> Self {
        Self::new(Vec::new())
    }
    fn is_zero(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Mod> One for Polynomial<T> {
    fn one() -> Self {
        Self::new(vec![Fp::one()])
    }
    fn is_one(&self) -> bool {
        *self == Self::one()
    }
}

impl<T: Mod> Add<Polynomial<T>> for Polynomial<T> {
    type Output = Polynomial<T>;
    fn add(self, rhs: Polynomial<T>) -> Self::Output {
        let mut res = self.0.clone();
        res.resize(std::cmp::max(self.len(), rhs.len()), Fp::zero());
        for (ans, a) in res.iter_mut().zip(rhs.0.iter()) {
            *ans += *a;
        }
        Polynomial::new(res)
    }
}

impl<'a, T: Mod> Add<Polynomial<T>> for &'a Polynomial<T> {
    type Output = Polynomial<T>;
    fn add(self, rhs: Polynomial<T>) -> Self::Output {
        let mut res = self.0.clone();
        res.resize(std::cmp::max(self.len(), rhs.len()), Fp::zero());
        for (ans, a) in res.iter_mut().zip(rhs.0.iter()) {
            *ans += *a;
        }
        Polynomial::new(res)
    }
}

impl<'a, T: Mod> Add<&'a Polynomial<T>> for Polynomial<T> {
    type Output = Polynomial<T>;
    fn add(self, rhs: &Polynomial<T>) -> Self::Output {
        let mut res = self.0.clone();
        res.resize(std::cmp::max(self.len(), rhs.len()), Fp::zero());
        for (ans, a) in res.iter_mut().zip(rhs.0.iter()) {
            *ans += *a;
        }
        Polynomial::new(res)
    }
}

impl<'a, T: Mod> Add<&'a Polynomial<T>> for &'a Polynomial<T> {
    type Output = Polynomial<T>;
    fn add(self, rhs: &Polynomial<T>) -> Self::Output {
        let mut res = self.0.clone();
        res.resize(std::cmp::max(self.len(), rhs.len()), Fp::zero());
        for (ans, a) in res.iter_mut().zip(rhs.0.iter()) {
            *ans += *a;
        }
        Polynomial::new(res)
    }
}

impl<T: Mod> Sub<Polynomial<T>> for Polynomial<T> {
    type Output = Polynomial<T>;
    fn sub(self, rhs: Polynomial<T>) -> Self::Output {
        let mut res = self.0.clone();
        res.resize(std::cmp::max(self.len(), rhs.len()), Fp::zero());
        for (ans, a) in res.iter_mut().zip(rhs.0.iter()) {
            *ans -= *a;
        }
        Polynomial::new(res)
    }
}

impl<'a, T: Mod> Sub<Polynomial<T>> for &'a Polynomial<T> {
    type Output = Polynomial<T>;
    fn sub(self, rhs: Polynomial<T>) -> Self::Output {
        let mut res = self.0.clone();
        res.resize(std::cmp::max(self.len(), rhs.len()), Fp::zero());
        for (ans, a) in res.iter_mut().zip(rhs.0.iter()) {
            *ans -= *a;
        }
        Polynomial::new(res)
    }
}

impl<'a, T: Mod> Sub<&'a Polynomial<T>> for Polynomial<T> {
    type Output = Polynomial<T>;
    fn sub(self, rhs: &Polynomial<T>) -> Self::Output {
        let mut res = self.0.clone();
        res.resize(std::cmp::max(self.len(), rhs.len()), Fp::zero());
        for (ans, a) in res.iter_mut().zip(rhs.0.iter()) {
            *ans -= *a;
        }
        Polynomial::new(res)
    }
}

impl<'a, T: Mod> Sub<&'a Polynomial<T>> for &'a Polynomial<T> {
    type Output = Polynomial<T>;
    fn sub(self, rhs: &Polynomial<T>) -> Self::Output {
        let mut res = self.0.clone();
        res.resize(std::cmp::max(self.len(), rhs.len()), Fp::zero());
        for (ans, a) in res.iter_mut().zip(rhs.0.iter()) {
            *ans -= *a;
        }
        Polynomial::new(res)
    }
}

impl<T: NTTFriendly> Mul for Polynomial<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(multiply(&self.0, &rhs.0))
    }
}

impl<'a, T: NTTFriendly> Mul<&'a Polynomial<T>> for Polynomial<T> {
    type Output = Polynomial<T>;
    fn mul(self, rhs: &'a Polynomial<T>) -> Self::Output {
        Polynomial::new(multiply(&self.0, &rhs.0))
    }
}

impl<'a, T: NTTFriendly> Mul<Polynomial<T>> for &'a Polynomial<T> {
    type Output = Polynomial<T>;
    fn mul(self, rhs: Polynomial<T>) -> Self::Output {
        Polynomial::new(multiply(&self.0, &rhs.0))
    }
}

impl<'a, T: NTTFriendly> Mul<&'a Polynomial<T>> for &'a Polynomial<T> {
    type Output = Polynomial<T>;
    fn mul(self, rhs: &'a Polynomial<T>) -> Self::Output {
        Polynomial::new(multiply(&self.0, &rhs.0))
    }
}

impl<T: Mod> AddAssign for Polynomial<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = (self as &_) + rhs;
    }
}

impl<T: Mod> SubAssign for Polynomial<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = (self as &_) - rhs;
    }
}

impl<T: NTTFriendly> MulAssign for Polynomial<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = (self as &_) * rhs;
    }
}