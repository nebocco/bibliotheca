use crate::utils::modint::*;
use std::ops::{Add, Mul, MulAssign, Sub};

// ---------- begin polynomial ----------

pub trait Zero: Sized + Add<Output = Self> {
    fn zero() -> Self;
}

impl<M: Modulus> Zero for StaticModInt<M> {
    fn zero() -> Self {
        StaticModInt::zero()
    }
}

impl Zero for usize {
    fn zero() -> Self {
        0
    }
}

pub trait ArrayAdd {
    type Item;
    fn add(&self, rhs: &[Self::Item]) -> Vec<Self::Item>;
}

impl<T> ArrayAdd for [T]
where
    T: Zero + Copy,
{
    type Item = T;
    fn add(&self, rhs: &[Self::Item]) -> Vec<Self::Item> {
        let mut c = vec![T::zero(); self.len().max(rhs.len())];
        c[..self.len()].copy_from_slice(self);
        c.add_assign(rhs);
        c
    }
}

pub trait ArrayAddAssign {
    type Item;
    fn add_assign(&mut self, rhs: &[Self::Item]);
}

impl<T> ArrayAddAssign for [T]
where
    T: Add<Output = T> + Copy,
{
    type Item = T;
    fn add_assign(&mut self, rhs: &[Self::Item]) {
        assert!(self.len() >= rhs.len());
        self.iter_mut().zip(rhs).for_each(|(x, a)| *x = *x + *a);
    }
}

impl<T> ArrayAddAssign for Vec<T>
where
    T: Zero + Add<Output = T> + Copy,
{
    type Item = T;
    fn add_assign(&mut self, rhs: &[Self::Item]) {
        if self.len() < rhs.len() {
            self.resize(rhs.len(), T::zero());
        }
        self.as_mut_slice().add_assign(rhs);
    }
}

pub trait ArraySub {
    type Item;
    fn sub(&self, rhs: &[Self::Item]) -> Vec<Self::Item>;
}

impl<T> ArraySub for [T]
where
    T: Zero + Sub<Output = T> + Copy,
{
    type Item = T;
    fn sub(&self, rhs: &[Self::Item]) -> Vec<Self::Item> {
        let mut c = vec![T::zero(); self.len().max(rhs.len())];
        c[..self.len()].copy_from_slice(self);
        c.sub_assign(rhs);
        c
    }
}

pub trait ArraySubAssign {
    type Item;
    fn sub_assign(&mut self, rhs: &[Self::Item]);
}

impl<T> ArraySubAssign for [T]
where
    T: Sub<Output = T> + Copy,
{
    type Item = T;
    fn sub_assign(&mut self, rhs: &[Self::Item]) {
        assert!(self.len() >= rhs.len());
        self.iter_mut().zip(rhs).for_each(|(x, a)| *x = *x - *a);
    }
}

impl<T> ArraySubAssign for Vec<T>
where
    T: Zero + Sub<Output = T> + Copy,
{
    type Item = T;
    fn sub_assign(&mut self, rhs: &[Self::Item]) {
        if self.len() < rhs.len() {
            self.resize(rhs.len(), T::zero());
        }
        self.as_mut_slice().sub_assign(rhs);
    }
}

pub trait ArrayDot {
    type Item;
    fn dot(&self, rhs: &[Self::Item]) -> Vec<Self::Item>;
}

impl<T> ArrayDot for [T]
where
    T: Mul<Output = T> + Copy,
{
    type Item = T;
    fn dot(&self, rhs: &[Self::Item]) -> Vec<Self::Item> {
        assert!(self.len() == rhs.len());
        self.iter().zip(rhs).map(|p| *p.0 * *p.1).collect()
    }
}

pub trait ArrayDotAssign {
    type Item;
    fn dot_assign(&mut self, rhs: &[Self::Item]);
}

impl<T> ArrayDotAssign for [T]
where
    T: MulAssign + Copy,
{
    type Item = T;
    fn dot_assign(&mut self, rhs: &[Self::Item]) {
        assert!(self.len() == rhs.len());
        self.iter_mut().zip(rhs).for_each(|(x, a)| *x *= *a);
    }
}

pub trait ArrayMul {
    type Item;
    fn mul(&self, rhs: &[Self::Item]) -> Vec<Self::Item>;
}

impl<T> ArrayMul for [T]
where
    T: Zero + Mul<Output = T> + Copy,
{
    type Item = T;
    fn mul(&self, rhs: &[Self::Item]) -> Vec<Self::Item> {
        if self.is_empty() || rhs.is_empty() {
            return vec![];
        }
        let mut res = vec![T::zero(); self.len() + rhs.len() - 1];
        for (i, a) in self.iter().enumerate() {
            for (c, b) in res[i..].iter_mut().zip(rhs) {
                *c = *c + *a * *b;
            }
        }
        res
    }
}

pub trait NTTFriendly: Modulus {
    const ORDER: usize;
    const ZETA: u32;
}

impl NTTFriendly for M998244353 {
    const ORDER: usize = 8388608;
    const ZETA: u32 = 15311432;
}

pub trait ArrayNTT {
    type Item;
    fn ntt(&mut self);
    fn intt(&mut self);
    fn multiply(&self, rhs: &[Self::Item]) -> Vec<Self::Item>;
}

impl<M: NTTFriendly> ArrayNTT for [StaticModInt<M>] {
    type Item = StaticModInt<M>;
    fn ntt(&mut self) {
        let f = self;
        let n = f.len();
        assert!(n.is_power_of_two());
        assert!(n <= M::ORDER);
        let len = n.trailing_zeros() as usize;
        let mut zeta = Vec::with_capacity(len);
        let mut r = StaticModInt::new(M::ZETA).pow((M::ORDER >> len) as u32);
        for _ in 0..len {
            zeta.push(r);
            r = r * r;
        }
        for (k, &z) in zeta.iter().rev().enumerate().rev() {
            let m = 1 << k;
            for f in f.chunks_exact_mut(2 * m) {
                let mut q = StaticModInt::new(1);
                let (x, y) = f.split_at_mut(m);
                for (x, y) in x.iter_mut().zip(y.iter_mut()) {
                    let a = *x;
                    let b = *y;
                    *x = a + b;
                    *y = (a - b) * q;
                    q *= z;
                }
            }
        }
    }

    fn intt(&mut self) {
        let f = self;
        let n = f.len();
        assert!(n.is_power_of_two());
        assert!(n <= M::ORDER);
        let len = n.trailing_zeros() as usize;
        let mut zeta = Vec::with_capacity(len);
        let mut r = StaticModInt::new(M::ZETA)
            .inv()
            .pow((M::ORDER >> len) as u32);
        for _ in 0..len {
            zeta.push(r);
            r = r * r;
        }
        for (k, &z) in zeta.iter().rev().enumerate() {
            let m = 1 << k;
            for f in f.chunks_exact_mut(2 * m) {
                let mut q = StaticModInt::new(1);
                let (x, y) = f.split_at_mut(m);
                for (x, y) in x.iter_mut().zip(y.iter_mut()) {
                    let a = *x;
                    let b = *y * q;
                    *x = a + b;
                    *y = a - b;
                    q *= z;
                }
            }
        }
        let ik = StaticModInt::new((M::MODULUS + 1) >> 1).pow(len as u32);
        for f in f.iter_mut() {
            *f *= ik;
        }
    }
    fn multiply(&self, rhs: &[Self::Item]) -> Vec<Self::Item> {
        if self.len().min(rhs.len()) <= 32 {
            return self.mul(rhs);
        }
        let n = self.len() + rhs.len() - 1;
        let k = n.next_power_of_two();
        assert!(k <= M::ORDER);
        let mut f = Vec::with_capacity(k);
        let mut g = Vec::with_capacity(k);
        f.extend_from_slice(self);
        f.resize(k, StaticModInt::zero());
        f.ntt();
        g.extend_from_slice(rhs);
        g.resize(k, StaticModInt::zero());
        g.ntt();
        f.dot_assign(&g);
        f.intt();
        f.truncate(n);
        f
    }
}

pub trait PolynomialOperation {
    type Item;
    fn eval(&self, x: Self::Item) -> Self::Item;
    fn derivative(&self) -> Vec<Self::Item>;
    fn integral(&self) -> Vec<Self::Item>;
}

impl<M: Modulus> PolynomialOperation for [StaticModInt<M>] {
    type Item = StaticModInt<M>;

    fn eval(&self, x: Self::Item) -> Self::Item {
        self.iter()
            .rev()
            .fold(StaticModInt::zero(), |s, a| s * x + *a)
    }

    fn derivative(&self) -> Vec<Self::Item> {
        if self.len() <= 1 {
            return Vec::new();
        }
        self[1..]
            .iter()
            .enumerate()
            .map(|(k, a)| StaticModInt::new(k as u32 + 1) * *a)
            .collect()
    }

    fn integral(&self) -> Vec<Self::Item> {
        if self.is_empty() {
            return vec![];
        }
        let mut inv = vec![StaticModInt::one(); self.len() + 1];
        let mut mul = StaticModInt::zero();
        for i in 1..=self.len() {
            mul += StaticModInt::one();
            inv[i] = inv[i - 1] * mul;
        }
        let mut prod = inv[self.len()].inv();
        for i in (1..=self.len()).rev() {
            inv[i] = self[i - 1] * inv[i - 1] * prod;
            prod *= mul;
            mul -= StaticModInt::one();
        }
        inv[0] = StaticModInt::zero();
        inv
    }
}

pub trait FPSOperation {
    type Item;
    fn inverse(&self, n: usize) -> Vec<Self::Item>;
    fn log(&self, n: usize) -> Vec<Self::Item>;
    fn exp(&self, n: usize) -> Vec<Self::Item>;
}

impl<M: NTTFriendly> FPSOperation for [StaticModInt<M>] {
    type Item = StaticModInt<M>;

    fn inverse(&self, n: usize) -> Vec<Self::Item> {
        assert!(self.len() > 0 && !self[0].to_inner() == 0);
        let len = n.next_power_of_two();
        assert!(2 * len <= M::ORDER);
        let mut b = vec![StaticModInt::zero(); n];
        b[0] = self[0].inv();
        let mut f = Vec::with_capacity(2 * len);
        let mut g = Vec::with_capacity(2 * len);
        let mut size = 1;
        while size < n {
            g.clear();
            g.extend(b.iter().take(size));
            g.resize(2 * size, StaticModInt::zero());
            f.clear();
            f.extend(self.iter().take(2 * size));
            f.resize(2 * size, StaticModInt::zero());
            f.ntt();
            g.ntt();
            f.dot_assign(&g);
            f.intt();
            f[..size].iter_mut().for_each(|f| *f = StaticModInt::zero());
            f.ntt();
            f.dot_assign(&g);
            f.intt();
            for (b, g) in b[size..].iter_mut().zip(&f[size..]) {
                *b = *b - *g;
            }
            size *= 2;
        }
        b
    }

    fn log(&self, n: usize) -> Vec<Self::Item> {
        assert!(self.get(0).map_or(false, |p| p.to_inner() == 1));
        let mut b = self.derivative().multiply(&self.inverse(n));
        b.truncate(n - 1);
        b.integral()
    }

    fn exp(&self, n: usize) -> Vec<Self::Item> {
        assert!(self.get(0).map_or(true, |a| a.to_inner() == 0));
        assert!(n <= M::ORDER);
        let mut b = vec![StaticModInt::one()];
        let mut size = 1;
        while size < n {
            size <<= 1;
            let f = b.log(size);
            let g = self[..self.len().min(size)].sub(&f);
            b = b.multiply(&g).add(&b);
            b.truncate(size);
        }
        b.truncate(n);
        b
    }
}
