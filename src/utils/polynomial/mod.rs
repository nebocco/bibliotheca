use crate::utils::{
    fp::{Fp, Mod},
    transform::*,
};

// ---------- begin polynomial ----------

mod poly_arith;

#[derive(Clone, PartialEq, Eq)]
pub struct Polynomial<T>(Vec<Fp<T>>);

impl<T: Mod> Polynomial<T> {
    pub fn new(a: Vec<Fp<T>>) -> Self {
        let mut a = Self(a);
        a.fix();
        a
    }

    pub fn from_slice(a: &[Fp<T>]) -> Self {
        Self::new(a.to_vec())
    }

    pub fn get(&self, x: usize) -> Fp<T> {
        self.0.get(x).cloned().unwrap_or_else(|| Fp::new(0))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn resize(&mut self, n: usize) {
        self.0.resize(n, Fp::new(0));
    }

    pub fn reverse(&self, n: usize) -> Self {
        assert!(self.len() >= n);
        let mut a = self.0.clone();
        a.resize(n, Fp::new(0));
        a.reverse();
        Self::new(a)
    }

    pub fn truncate(&self, n: usize) -> Self {
        let mut b = self.0.clone();
        b.truncate(n);
        Polynomial::new(b)
    }

    pub fn eval(&self, x: Fp<T>) -> Fp<T> {
        let mut ans = Fp::new(0);
        for a in self.0.iter().rev() {
            ans = ans * x + *a;
        }
        ans
    }

    pub fn fix(&mut self) {
        while self.0.last().map_or(false, |x| x.into_inner() == 0) {
            self.0.pop();
        }
    }

    pub fn derivative(&self) -> Self {
        if self.len() < 2 {
            return Self::new(Vec::new());
        }
        let b = self
            .0
            .iter()
            .skip(1)
            .enumerate()
            .map(|(i, a)| *a * Fp::new((i + 1) as i64))
            .collect();
        Polynomial::new(b)
    }

    pub fn integral(&self) -> Self {
        if self.is_empty() {
            return Self::new(Vec::new());
        }
        let mut b = vec![Fp::new(0); self.len() + 1];
        let mut inv = vec![Fp::new(1); self.len() + 1];
        b[1] = self.0[0];
        for (i, (b, a)) in b[1..].iter_mut().zip(self.0.iter()).enumerate().skip(1) {
            let k = i + 1;
            inv[k] = -inv[T::MOD as usize % k] * Fp::new(T::MOD / k as i64);
            *b = *a * inv[k];
        }
        Polynomial::new(b)
    }
}
impl<T: NTTFriendly> Polynomial<T> {
    pub fn inverse(&self, n: usize) -> Self {
        let len = n.next_power_of_two();
        assert!(2 * len <= T::order());
        let mut b = Vec::with_capacity(len);
        b.push(self.0[0].inv());
        let mut f = Vec::with_capacity(2 * len);
        let mut g = Vec::with_capacity(2 * len);
        let mut size = 1;
        while b.len() < n {
            size <<= 1;
            f.clear();
            f.extend_from_slice(&b);
            f.resize(2 * size, Fp::new(0));
            g.clear();
            if self.0.len() >= size {
                g.extend_from_slice(&self.0[..size]);
            } else {
                g.extend_from_slice(&self.0);
            }
            g.resize(2 * size, Fp::new(0));
            ntt(&mut f);
            ntt(&mut g);
            for (g, f) in g.iter_mut().zip(f.iter()) {
                *g *= *f * *f;
            }
            intt(&mut g);
            b.resize(size, Fp::new(0));
            for (b, g) in b.iter_mut().zip(g.iter()) {
                *b = *b + *b - *g;
            }
        }
        b.truncate(n);
        Polynomial::new(b)
    }

    pub fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        let n = self.len();
        let m = rhs.len();
        assert!(m > 0);
        if n < m {
            return (Self::new(Vec::new()), self.clone());
        }
        let ia = self.reverse(n).truncate(n - m + 1);
        let ib = rhs.reverse(m).inverse(n - m + 1);
        let id = (ia * ib).truncate(n - m + 1);
        let div = id.reverse(n - m + 1);
        let rem = self - (rhs * &div).truncate(m - 1);
        (div, rem)
    }

    pub fn rem(&self, rhs: &Self) -> Self {
        self.div_rem(rhs).1
    }

    pub fn log(&self, n: usize) -> Self {
        assert!(!self.is_empty() && self.0[0].into_inner() == 1);
        (self.derivative() * self.inverse(n))
            .truncate(n - 1)
            .integral()
    }

    pub fn exp(&self, n: usize) -> Self {
        assert!(self.0.get(0).map_or(true, |x| x.into_inner() == 0) && n <= T::order());
        let mut b = Polynomial::new(vec![Fp::new(1)]);
        for size in std::iter::successors(Some(1), |&x| Some(x << 1)).take_while(|&x| x < n) {
            let f = b.log(size);
            let f = Polynomial::from_slice(&self.0[..std::cmp::min(self.len(), size)]) - f;
            b += (&b * f).truncate(size);
        }
        b.truncate(n)
    }

    pub fn multi_eval(&self, x: &[Fp<T>]) -> Vec<Fp<T>> {
        let size = x.len().next_power_of_two();
        let mut seg = vec![Some(Self::new(vec![Fp::new(1)])); 2 * size];
        for (seg, x) in seg[size..].iter_mut().zip(x.iter()) {
            *seg = Some(Polynomial::from_slice(&[-*x, Fp::new(1)]));
        }
        for i in (1..size).rev() {
            seg[i] = Some(seg[2 * i].as_ref().unwrap() * seg[2 * i + 1].as_ref().unwrap());
        }
        let mut rem = vec![None; 2 * size];
        rem[1] = Some(self.rem(&seg[1].take().unwrap()));
        for i in 1..size {
            let a = rem[i].take().unwrap();
            rem[2 * i] = Some(a.rem(&seg[2 * i].take().unwrap()));
            rem[2 * i + 1] = Some(a.rem(&seg[2 * i + 1].take().unwrap()));
        }
        let mut ans = Vec::with_capacity(x.len());
        for a in rem[size..].iter_mut().take(x.len()) {
            ans.push(a.take().unwrap().get(0));
        }
        ans
    }
    pub fn interpolation(x: &[Fp<T>], y: &[Fp<T>]) -> Self {
        assert!(!x.is_empty() && x.len() == y.len());
        let size = x.len().next_power_of_two();
        let mut p = vec![Self::new(vec![Fp::new(1)]); 2 * size];
        for (p, x) in p[size..].iter_mut().zip(x.iter()) {
            *p = Polynomial::new(vec![-*x, Fp::new(1)]);
        }
        for i in (1..size).rev() {
            p[i] = &p[2 * i] * &p[2 * i + 1];
        }
        let z = p[1].derivative().multi_eval(x);
        let mut a = vec![Self::new(Vec::new()); 2 * size];
        for (a, (z, y)) in a[size..].iter_mut().zip(z.iter().zip(y.iter())) {
            *a = Polynomial::new(vec![y * z.inv()]);
        }
        for i in (1..size).rev() {
            a[i] = &a[2 * i] * &p[2 * i + 1] + &a[2 * i + 1] * &p[2 * i];
        }
        a.swap_remove(1)
    }
}

// ---------- begin polynomial ----------
