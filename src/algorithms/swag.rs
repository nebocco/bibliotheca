use crate::utils::algebraic_traits::SemiGroup;

// * verified: https://judge.yosupo.jp/submission/28460
// ------------ module start ------------

#[derive(Default)]
pub struct SWAG<T: SemiGroup> {
    front: Vec<(T, T)>,
    back: Vec<(T, T)>,
}

impl<T: SemiGroup> SWAG<T> {
    pub fn new() -> Self {
        Self {
            front: Vec::new(),
            back: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.front.len() + self.back.len()
    }

    pub fn is_empty(&self) -> bool {
        self.front.is_empty() && self.back.is_empty()
    }

    pub fn push(&mut self, v: T) {
        let s = if let Some((_, x)) = self.back.last() {
            x.clone() + v.clone()
        } else {
            v.clone()
        };
        self.back.push((v, s));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.front.is_empty() {
            let back = std::mem::replace(&mut self.back, Vec::new());
            for (v, _) in back.into_iter().rev() {
                let s = if let Some((_, x)) = self.front.last() {
                    v.clone() + x.clone()
                } else {
                    v.clone()
                };
                self.front.push((v, s));
            }
            // self.back.clear();
        }
        if let Some((x, _)) = self.front.pop() {
            Some(x)
        } else {
            None
        }
    }

    pub fn fold_all(&self) -> Option<T> {
        match (self.front.last(), self.back.last()) {
            (Some(u), Some(v)) => Some(u.1.clone() + v.1.clone()),
            (Some(u), None) => Some(u.1.clone()),
            (None, Some(v)) => Some(v.1.clone()),
            (None, None) => None,
        }
    }
}
// ------------ module end ------------

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