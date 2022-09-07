// * verified: https://judge.yosupo.jp/submission/28460
// ------------ Swag Queue start ------------

pub trait SemiGroup {
    type Val: Clone;
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}
#[derive(Default)]
pub struct SwagQueue<G: SemiGroup> {
    front: Vec<(G::Val, G::Val)>,
    back: Vec<(G::Val, G::Val)>,
}

impl<G: SemiGroup> SwagQueue<G> {
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

    pub fn push(&mut self, v: G::Val) {
        let s = if let Some((_, x)) = self.back.last() {
            G::op(x, &v)
        } else {
            v.clone()
        };
        self.back.push((v, s));
    }

    pub fn pop(&mut self) -> Option<G::Val> {
        if self.front.is_empty() {
            let back = std::mem::take(&mut self.back);
            for (v, _) in back.into_iter().rev() {
                let s = if let Some((_, x)) = self.front.last() {
                    G::op(&v, x)
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

    pub fn fold_all(&self) -> Option<G::Val> {
        match (self.front.last(), self.back.last()) {
            (Some(u), Some(v)) => Some(G::op(&u.1, &v.1)),
            (Some(u), None) => Some(u.1.clone()),
            (None, Some(v)) => Some(v.1.clone()),
            (None, None) => None,
        }
    }
}
// ------------ Swag Queue end ------------

// ------------ Swag Deque start ------------
#[derive(Default)]
pub struct SwagDeque<G: SemiGroup> {
    front: Vec<(G::Val, G::Val)>,
    back: Vec<(G::Val, G::Val)>,
}

impl<G: SemiGroup> SwagDeque<G> {
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

    fn _push<F>(stack: &mut Vec<(G::Val, G::Val)>, v: G::Val, op: F)
    where
        F: FnOnce(&G::Val, &G::Val) -> G::Val,
    {
        let s = if let Some((_, x)) = stack.last() {
            op(x, &v)
        } else {
            v.clone()
        };
        stack.push((v, s));
    }

    fn _pop<F>(
        stack: &mut Vec<(G::Val, G::Val)>,
        other: &mut Vec<(G::Val, G::Val)>,
        op: F,
    ) -> Option<G::Val>
    where
        F: Fn(&G::Val, &G::Val) -> G::Val,
    {
        if stack.is_empty() {
            let n = other.len();
            let temp = other.split_off((n + 1) / 2);
            for (v, _) in other.drain(..).rev() {
                let s = if let Some((_, x)) = stack.last() {
                    op(x, &v)
                } else {
                    v.clone()
                };
                stack.push((v, s));
            }
            for (v, _) in temp {
                let s = if let Some((_, x)) = other.last() {
                    op(&v, x)
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

    pub fn push_back(&mut self, v: G::Val) {
        Self::_push(&mut self.back, v, |x, v| G::op(x, v));
    }

    pub fn push_front(&mut self, v: G::Val) {
        Self::_push(&mut self.front, v, |x, v| G::op(v, x));
    }

    pub fn pop_back(&mut self) -> Option<G::Val> {
        Self::_pop(&mut self.back, &mut self.front, |x, v| G::op(x, v))
    }

    pub fn pop_front(&mut self) -> Option<G::Val> {
        Self::_pop(&mut self.front, &mut self.back, |x, v| G::op(v, x))
    }

    pub fn fold_all(&self) -> Option<G::Val> {
        match (self.front.last(), self.back.last()) {
            (Some(u), Some(v)) => Some(G::op(&u.1, &v.1)),
            (Some(u), None) => Some(u.1.clone()),
            (None, Some(v)) => Some(v.1.clone()),
            (None, None) => None,
        }
    }
}
// ------------ Swag Deque end ------------

#[cfg(test)]
mod tests {
    use super::*;

    enum Min {}

    impl SemiGroup for Min {
        type Val = i32;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            *left.min(right)
        }
    }

    #[test]
    fn test_swag_queue_min() {
        let mut que = SwagQueue::<Min>::new();
        que.push(2);
        que.push(3);
        que.push(5);
        que.push(7);
        assert_eq!(que.fold_all(), Some(2));
        assert_eq!(que.pop(), Some(2));
        assert_eq!(que.pop(), Some(3));
        assert_eq!(que.fold_all(), Some(5));
        que.push(-1);
        assert_eq!(que.fold_all(), Some(-1));
        assert_eq!(que.pop(), Some(5));
        assert_eq!(que.pop(), Some(7));
        assert_eq!(que.pop(), Some(-1));
        assert_eq!(que.fold_all(), None);
        assert_eq!(que.pop(), None);
    }

    #[test]
    fn test_swag_deque_min() {
        let mut que = SwagDeque::<Min>::new();
        que.push_back(2);
        que.push_back(3);
        que.push_front(5);
        que.push_front(7);
        assert_eq!(que.fold_all(), Some(2));
        assert_eq!(que.pop_back(), Some(3));
        assert_eq!(que.pop_back(), Some(2));
        assert_eq!(que.fold_all(), Some(5));
        que.push_front(-1);
        assert_eq!(que.fold_all(), Some(-1));
        assert_eq!(que.pop_back(), Some(5));
        assert_eq!(que.pop_back(), Some(7));
        assert_eq!(que.pop_back(), Some(-1));
        assert_eq!(que.fold_all(), None);
        assert_eq!(que.pop_front(), None);
    }

    const MOD: u64 = 998_244_353;

    enum Affine {}

    impl Affine {
        fn eval(val: <Affine as SemiGroup>::Val, x: u64) -> u64 {
            (val.0 * x + val.1) % MOD
        }
    }

    impl SemiGroup for Affine {
        type Val = (u64, u64);
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            (right.0 * left.0 % MOD, (right.0 * left.1 + right.1) % MOD)
        }
    }

    #[test]
    fn test_swag_queue_affine() {
        let mut que = SwagQueue::<Affine>::new();
        que.push((4, 5));
        assert_eq!(Affine::eval(que.fold_all().unwrap(), 3), 17);
        que.push((2, 1));
        assert_eq!(Affine::eval(que.fold_all().unwrap(), 2), 27);
        assert_eq!(que.pop(), Some((4, 5)));
        assert_eq!(Affine::eval(que.fold_all().unwrap(), 3), 7);
    }

    #[test]
    fn test_swag_deque_affine() {
        let mut que = SwagDeque::<Affine>::new();
        que.push_back((4, 5));
        assert_eq!(Affine::eval(que.fold_all().unwrap(), 3), 17);
        que.push_back((2, 1));
        assert_eq!(Affine::eval(que.fold_all().unwrap(), 2), 27);
        assert_eq!(que.pop_front(), Some((4, 5)));
        assert_eq!(Affine::eval(que.fold_all().unwrap(), 3), 7);

        que.push_front((3, 2));
        assert_eq!(Affine::eval(que.fold_all().unwrap(), 5), 35);
        assert_eq!(que.pop_back(), Some((2, 1)));
        assert_eq!(Affine::eval(que.fold_all().unwrap(), 6), 20);
        assert_eq!(que.pop_back(), Some((3, 2)));
        assert_eq!(que.fold_all(), None);
        assert_eq!(que.pop_front(), None);
    }
}
