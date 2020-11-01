#![allow(dead_code)]

use crate::utils::algebraic_traits::SemiGroup;

struct SWAG<T: SemiGroup> {
    front: Vec<T>,
    back: Vec<T>,
    fsum: Vec<T>,
    bsum: Vec<T>,
}

impl<T: SemiGroup> SWAG<T> {
    fn new() -> Self {
        Self {
            front: Vec::new(),
            back: Vec::new(),
            fsum: Vec::new(),
            bsum: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.front.len() + self.back.len()
    }

    fn is_empty(&self) -> bool {
        self.front.is_empty() && self.back.is_empty()
    }

    fn push(&mut self, v: T) {
        let s = if let Some(x) = self.bsum.last() {
            x.clone() + v.clone()
        } else {
            v.clone()
        };
        self.bsum.push(s);
        self.back.push(v);
    }

    fn pop(&mut self) -> Option<T> {
        if self.front.is_empty() {
            while let Some(v) = self.back.pop() {
                let s = if let Some(x) = self.fsum.last() {
                    x.clone() + v.clone()
                } else {
                    v.clone()
                };
                self.fsum.push(s);
                self.front.push(v);
            }
            self.bsum.clear();
        }
        self.fsum.pop();
        self.front.pop()
    }

    fn fold_all(&self) -> Option<T> {
        match (self.fsum.last(), self.bsum.last()) {
            (Some(u), Some(v)) => Some(u.clone() + v.clone()),
            (Some(u), None) => Some(u.clone()),
            (None, Some(v)) => Some(v.clone()),
            (None, None) => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use std::ops::Add;
    use crate::utils::algebraic_traits::*;
    use super::*;

    #[test]
    fn test_min() {
        #[derive(Debug, Clone, PartialEq)]
        struct Ele(i32);

        impl Add for Ele {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self(std::cmp::min(self.0, rhs.0))
            }
        }
        impl Associative for Ele {}

        let mut que = SWAG::<Ele>::new();
        que.push(Ele(2));
        que.push(Ele(3));
        que.push(Ele(5));
        que.push(Ele(7));
        assert_eq!(que.fold_all(), Some(Ele(2)));
        assert_eq!(que.pop(), Some(Ele(2)));
        assert_eq!(que.pop(), Some(Ele(3)));
        assert_eq!(que.fold_all(), Some(Ele(5)));
        que.push(Ele(-1));
        assert_eq!(que.fold_all(), Some(Ele(-1)));
        assert_eq!(que.pop(), Some(Ele(5)));
        assert_eq!(que.pop(), Some(Ele(7)));
        assert_eq!(que.pop(), Some(Ele(-1)));
        assert_eq!(que.fold_all(), None);
        assert_eq!(que.pop(), None);
    }

    #[test]
    fn test_affine() {
        const MOD: u64 = 998_244_353;

        #[derive(Debug, Clone, PartialEq)]
        struct Ele(u64, u64);

        impl Ele {
            fn eval(&self, x: u64) -> u64 {
                (self.0 * x + self.1) % MOD
            }
        }

        impl Add for Ele {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self(
                    rhs.0 * self.0 % MOD,
                    (rhs.0 * self.1 + rhs.1) % MOD,
                )
            }
        }
        impl Associative for Ele {}

        let mut que = SWAG::<Ele>::new();
        que.push(Ele(4, 5));
        assert_eq!(que.fold_all().unwrap().eval(3), 17);
        que.push(Ele(2, 1));
        assert_eq!(que.fold_all().unwrap().eval(2), 27);
        assert_eq!(que.pop(), Some(Ele(4, 5)));
        assert_eq!(que.fold_all().unwrap().eval(3), 7);
    }
}