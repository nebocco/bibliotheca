use crate::utils::algebraic_traits::SemiGroup;

// * verified: https://judge.yosupo.jp/submission/28460
// ------------ Swag Queue start ------------
#[derive(Default)]
pub struct SwagQueue<T: SemiGroup> {
    front: Vec<(T, T)>,
    back: Vec<(T, T)>,
}

impl<T: SemiGroup> SwagQueue<T> {
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
// ------------ Swag Queue end ------------


// ------------ Swag Deque start ------------
#[derive(Default)]
pub struct SwagDeque<T: SemiGroup> {
    front: Vec<(T, T)>,
    back: Vec<(T, T)>,
}

impl<T: SemiGroup> SwagDeque<T> {
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

    fn _push<F>(stack: &mut Vec<(T, T)>, v: T, func: F)
        where
            F: FnOnce(T, T) -> T
    {
        let s = if let Some((_, x)) = stack.last() {
            func(x.clone(), v.clone())
        } else {
            v.clone()
        };
        stack.push((v, s));
    }

    fn _pop<F>(stack: &mut Vec<(T, T)>, other: &mut Vec<(T, T)>, func: F) -> Option<T>
        where
            F: Fn(T, T) -> T
    {
        if stack.is_empty() {
            let n = other.len();
            let temp = other.split_off((n+1)/2);
            for (v, _) in other.drain(..).rev() {
                let s = if let Some((_, x)) = stack.last() {
                    func(x.clone(), v.clone())
                } else {
                    v.clone()
                };
                stack.push((v, s));
            }
            for (v, _) in temp {
                let s = if let Some((_, x)) = other.last() {
                    func(v.clone(), x.clone())
                } else {
                    v.clone()
                };
                other.push((v, s));
            }
            // self.back.clear();
        }
        if let Some((x, _)) = stack.pop() {
            Some(x)
        } else {
            None
        }
    }

    pub fn push_back(&mut self, v: T) {
        Self::_push(&mut self.back, v, |x, v| x + v);
    }

    pub fn push_front(&mut self, v: T) {
        Self::_push(&mut self.front, v, |x, v| v + x);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        Self::_pop(&mut self.back, &mut self.front, |x, v| x + v)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        Self::_pop(&mut self.front, &mut self.back, |x, v| v + x)
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
// ------------ Swag Deque end ------------

#[cfg(test)]
mod tests {
    use std::ops::Add;
    use crate::utils::algebraic_traits::*;
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Min(i32);

    impl Add for Min {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Self(std::cmp::min(self.0, rhs.0))
        }
    }
    impl Associative for Min {}

    #[test]
    fn test_swag_queue_min() {
        let mut que = SwagQueue::<Min>::new();
        que.push(Min(2));
        que.push(Min(3));
        que.push(Min(5));
        que.push(Min(7));
        assert_eq!(que.fold_all(), Some(Min(2)));
        assert_eq!(que.pop(), Some(Min(2)));
        assert_eq!(que.pop(), Some(Min(3)));
        assert_eq!(que.fold_all(), Some(Min(5)));
        que.push(Min(-1));
        assert_eq!(que.fold_all(), Some(Min(-1)));
        assert_eq!(que.pop(), Some(Min(5)));
        assert_eq!(que.pop(), Some(Min(7)));
        assert_eq!(que.pop(), Some(Min(-1)));
        assert_eq!(que.fold_all(), None);
        assert_eq!(que.pop(), None);
    }

    #[test]
    fn test_swag_deque_min() {
        let mut que = SwagDeque::<Min>::new();
        que.push_back(Min(2));
        que.push_back(Min(3));
        que.push_front(Min(5));
        que.push_front(Min(7));
        assert_eq!(que.fold_all(), Some(Min(2)));
        assert_eq!(que.pop_back(), Some(Min(3)));
        assert_eq!(que.pop_back(), Some(Min(2)));
        assert_eq!(que.fold_all(), Some(Min(5)));
        que.push_front(Min(-1));
        assert_eq!(que.fold_all(), Some(Min(-1)));
        assert_eq!(que.pop_back(), Some(Min(5)));
        assert_eq!(que.pop_back(), Some(Min(7)));
        assert_eq!(que.pop_back(), Some(Min(-1)));
        assert_eq!(que.fold_all(), None);
        assert_eq!(que.pop_front(), None);
    }

    const MOD: u64 = 998_244_353;

    #[derive(Debug, Clone, PartialEq)]
    struct Affine(u64, u64);

    impl Affine {
        fn eval(&self, x: u64) -> u64 {
            (self.0 * x + self.1) % MOD
        }
    }

    impl Add for Affine {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Self(
                rhs.0 * self.0 % MOD,
                (rhs.0 * self.1 + rhs.1) % MOD,
            )
        }
    }
    impl Associative for Affine {}

    #[test]
    fn test_swag_queue_affine() {
        let mut que = SwagQueue::<Affine>::new();
        que.push(Affine(4, 5));
        assert_eq!(que.fold_all().unwrap().eval(3), 17);
        que.push(Affine(2, 1));
        assert_eq!(que.fold_all().unwrap().eval(2), 27);
        assert_eq!(que.pop(), Some(Affine(4, 5)));
        assert_eq!(que.fold_all().unwrap().eval(3), 7);
    }

    #[test]
    fn test_swag_deque_affine() {
        let mut que = SwagDeque::<Affine>::new();
        que.push_back(Affine(4, 5));
        assert_eq!(que.fold_all().unwrap().eval(3), 17);
        que.push_back(Affine(2, 1));
        assert_eq!(que.fold_all().unwrap().eval(2), 27);
        assert_eq!(que.pop_front(), Some(Affine(4, 5)));
        assert_eq!(que.fold_all().unwrap().eval(3), 7);

        que.push_front(Affine(3, 2));
        assert_eq!(que.fold_all().unwrap().eval(5), 35);
        assert_eq!(que.pop_back(), Some(Affine(2, 1)));
        assert_eq!(que.fold_all().unwrap().eval(6), 20);
        assert_eq!(que.pop_back(), Some(Affine(3, 2)));
        assert_eq!(que.fold_all(), None);
        assert_eq!(que.pop_front(), None);
    }
}