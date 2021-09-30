use crate::utils::bounds::bounds_within;

// verified:
// Range update/chGCD Range max/sum: https://yukicoder.me/submissions/703569

// FAILED:
// Range chmin/chmax/add Range sum: https://judge.yosupo.jp/submission/61988
// ------------ Segment Tree Beats start ------------

pub trait Operation {
    type Val: Clone + PartialEq + Beats;
    type Eff: Clone + PartialEq;
    const ZERO_VAL: Self::Val;
    const ZERO_EFF: Self::Eff;
    fn op_val(left: &Self::Val, right: &Self::Val) -> Self::Val;
    fn op_eff(left: &Self::Eff, right: &Self::Eff) -> Self::Eff;
    fn effect(val: &Self::Val, eff: &Self::Eff) -> Self::Val;
    fn multiply(eff: &Self::Eff, _times: u32) -> Self::Eff { eff.clone() }
}

pub trait Beats {
    fn is_failed(&self) -> bool;
}

#[derive(Clone)]
struct Node<T, E> {
    val: T,
    lazy: E,
}

impl<T, E> Node<T, E> {
    fn new(val: T, lazy: E) -> Self {
        Self { val, lazy }
    }
}

pub struct SegmentTreeBeats<O: Operation> {
    node: Box<[Node<O::Val, O::Eff>]>,
    size: usize,
    dep: u32,
}

impl<O: Operation> SegmentTreeBeats<O> {
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        let dep = size.trailing_zeros() + 1;
        let node = vec![Node::new(O::ZERO_VAL, O::ZERO_EFF); size << 1]
            .into_boxed_slice();
        Self { node, size, dep }
    }

    #[inline]
    fn degree(&self, i: usize) -> u32 {
        1 << (i.leading_zeros() + self.dep - 64)
    }

    fn effect(&mut self, i: usize, e: &O::Eff) {
        self.node[i].val = O::effect(&self.node[i].val, &O::multiply(e, self.degree(i)));
        if i <= self.size {
            self.node[i].lazy = O::op_eff(&self.node[i].lazy, e);
            if self.node[i].val.is_failed() {
                self.push(i);
                self.node[i].val = O::op_val(&self.node[i << 1].val, &self.node[(i << 1) + 1].val);
            }
        }
    }

    fn push(&mut self, i: usize) {
        let e = std::mem::replace(&mut self.node[i].lazy, O::ZERO_EFF);
        if e != O::ZERO_EFF && i <= self.size {
            self.effect(i << 1, &e);
            self.effect((i << 1) + 1, &e);
        }
    }

    fn infuse(&mut self, mut i: usize) {
        i >>= i.trailing_zeros();
        while i > 1 {
            i >>= 1;
            self.node[i].val = O::op_val(&self.node[i << 1].val, &self.node[(i << 1) + 1].val);
        }
    }

    fn infiltrate(&mut self, i: usize) {
        if i < self.size << 1 {
            let d = i.trailing_zeros();
            for j in (d..self.dep).rev() {
                self.push(i >> j);
            }
        }
    }

    pub fn update<Rng: std::ops::RangeBounds<usize>>(&mut self, rng: Rng, e: O::Eff) {
        let rng = bounds_within(rng, self.size);
        let mut l = rng.start + self.size;
        let mut r = rng.end + self.size;
        self.infiltrate(l);
        self.infiltrate(r);
        while l < r {
            if l & 1 == 1 {
                self.effect(l, &e);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.effect(r, &e);
            }
            l >>= 1;
            r >>= 1;
        }
        self.infuse(rng.start + self.size);
        self.infuse(rng.end + self.size);
    }

    pub fn fold<Rng: std::ops::RangeBounds<usize>>(&mut self, rng: Rng) -> O::Val {
        let rng = bounds_within(rng, self.size);
        let mut l = rng.start + self.size;
        let mut r = rng.end + self.size;
        self.infiltrate(l);
        self.infiltrate(r);
        let mut lx = O::ZERO_VAL;
        let mut rx = O::ZERO_VAL;
        while l < r {
            if l & 1 == 1 {
                lx = O::op_val(&lx, &self.node[l].val);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                rx = O::op_val(&self.node[r].val, &rx);
            }
            l >>= 1;
            r >>= 1;
        }
        O::op_val(&lx, &rx)
    }
}

impl<O: Operation> From<&Vec<O::Val>> for SegmentTreeBeats<O> {
    fn from(arr: &Vec<O::Val>) -> Self {
        let size = arr.len().next_power_of_two();
        let dep = size.trailing_zeros() + 1;
        let mut node = vec![Node::new(O::ZERO_VAL, O::ZERO_EFF); size << 1];
        for i in 0..arr.len() {
            node[i + size].val = arr[i].clone();
        }
        for i in (1..size).rev() {
            node[i].val = O::op_val(&node[i << 1].val, &node[(i << 1) + 1].val);
        }
        Self { node: node.into_boxed_slice(), size, dep }
    }
}

// ------------ Segment Tree Beats end ------------

mod chmin_chmax {
    use super::*;

    fn second_lowest(l1: i64, l2: i64, r1: i64, r2: i64) -> i64 {
        if l1 == r1 {
            l2.min(r2)
        } else if l2 <= r1 {
            l2
        } else if r2 <= l1 {
            r2
        } else {
            l1.max(r1)
        }
    }

    fn second_highest(l1: i64, l2: i64, r1: i64, r2: i64) -> i64 {
        if l1 == r1 {
            l2.max(r2)
        } else if l2 >= r1 {
            l2
        } else if r2 >= l1 {
            r2
        } else {
            l1.min(r1)
        }
    }

    #[derive(Clone, PartialEq)]
    struct Data {
        lo: i64, hi: i64, lo2: i64, hi2: i64,
        sum: i64, size: i64, nlo: i64, nhi: i64,
        fail: bool
    }

    impl Data {
        fn sized(val: i64, size: i64) -> Self {
            Self {
                lo: val, lo2: std::i64::MAX, hi: val, hi2: std::i64::MIN,
                sum: val * size, size, nlo: size, nhi: size, fail: false
            }
        }
    }

    impl Beats for Data {
        fn is_failed(&self) -> bool { self.fail } 
    }

    #[derive(Clone, PartialEq)]
    struct Op {
        lb: i64, ub: i64, bias: i64
    }

    struct ChminChmaxAddSum;

    impl Operation for ChminChmaxAddSum {
        type Val = Data;
        type Eff = Op;
        const ZERO_VAL: Self::Val = Data{
            lo: std::i64::MAX, lo2: std::i64::MAX, hi: std::i64::MIN, hi2: std::i64::MIN,
            sum: 0, size: 0, nlo: 0, nhi: 0, fail: false
        };
        const ZERO_EFF: Self::Eff = Op {
            lb: std::i64::MIN, ub: std::i64::MAX, bias: 0
        };
        fn op_val(left: &Self::Val, right: &Self::Val) -> Self::Val {
            Data {
                lo: left.lo.min(right.lo),
                lo2: second_lowest(left.lo, left.lo2, right.lo, right.lo2),
                hi: left.hi.max(right.hi),
                hi2: second_highest(left.hi, left.hi2, right.hi, right.hi2),
                sum: left.sum + right.sum,
                size: left.size + right.size,
                nlo: if left.lo < right.lo {
                    left.nlo
                } else if left.lo > right.lo {
                    right.nlo
                } else {
                    left.nlo + right.nlo
                },
                nhi: if left.hi > right.hi {
                    left.nhi
                } else if left.hi < right.hi {
                    right.nhi
                } else {
                    left.nhi + right.nhi
                },
                fail: left.fail | right.fail
            }
        }
        fn op_eff(left: &Self::Eff, right: &Self::Eff) -> Self::Eff {
            Op {
                lb: (left.lb + left.bias).min(right.ub).max(right.lb) - left.bias,
                ub: (left.ub + left.bias).max(right.lb).min(right.ub) - left.bias,
                bias: left.bias + right.bias
            }
        }
        fn effect(val: &Self::Val, eff: &Self::Eff) -> Self::Val {
            let mut res = val.clone();
            
            if val.size == 0 {
                return Self::ZERO_VAL
            } else if val.lo == val.hi || eff.lb == eff.ub || eff.lb >= val.hi || eff.ub <= val.lo {
                return Data::sized(val.lo.max(eff.lb).min(eff.ub), val.size);
            } else if val.lo2 == val.hi {
                res.lo = res.lo.max(eff.lb) + eff.bias;
                res.hi2 = res.lo.max(eff.lb) + eff.bias;
                res.hi = res.hi.min(eff.ub) + eff.bias;
                res.lo2 = res.hi.min(eff.ub) + eff.bias;
                res.sum = res.nlo * res.lo + res.nhi * val.hi;
                return res;
            } else if eff.lb < val.lo2 && eff.ub > val.hi2 {
                let next_lo = val.lo.max(eff.lb);
                let next_hi = val.hi.min(eff.ub);
                res.sum += (next_lo - val.lo) * val.nlo - (val.hi - next_hi) * val.nhi + eff.bias * val.size;
                return res;
            }
            res.fail = true;
            res

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            a %= b;
            std::mem::swap(&mut a, &mut b);
        }
        a
    }
    
    pub fn lcm(a: i64, b: i64) -> i64 {
        if a == 0 && b == 0 {
            0
        } else {
            (a / gcd(a, b)).saturating_mul(b)
        }
    }
    
    #[derive(Clone, PartialEq)]
    struct Rucms {
        max: i64,
        lcm: i64,
        sum: i64,
        size: i64,
        fail: bool
    }
    
    impl Rucms {
        fn new(val: i64) -> Self {
            Self {
                max: val,
                lcm: val,
                sum: val,
                size: 1,
                fail: false
            }
        }
    }
    
    impl Beats for Rucms {
        fn is_failed(&self) -> bool { self.fail }
    }
    
    #[derive(Clone, PartialEq)]
    enum Op {
        Gcd(i64),
        Update(i64),
        None
    }
    
    struct RangeUpdateChgcdMaxSum;
    impl Operation for RangeUpdateChgcdMaxSum {
        type Val = Rucms;
        type Eff = Op;
        const ZERO_VAL: Self::Val = Rucms{
            max: 0, lcm: 1, sum: 0, size: 0, fail: false
        };
        const ZERO_EFF: Self::Eff = Op::None;
        fn op_val(left: &Self::Val, right: &Self::Val) -> Self::Val {
            Self::Val {
                max: left.max.max(right.max),
                lcm: lcm(left.lcm, right.lcm),
                sum: left.sum + right.sum,
                size: left.size + right.size,
                fail: left.fail | right.fail
            }
        }
        fn op_eff(left: &Self::Eff, right: &Self::Eff) -> Self::Eff {
            match (left, right) {
                (left, Op::None) => left.clone(),
                (Op::None, right) => right.clone(),
                (_, Op::Update(_)) => right.clone(),
                (Op::Gcd(a), Op::Gcd(b)) => Op::Gcd(gcd(*a, *b)),
                (Op::Update(a), Op::Gcd(b)) => Op::Update(gcd(*a, *b))
            }
        }
        fn effect(val: &Self::Val, eff: &Self::Eff) -> Self::Val {
            let mut res = val.clone();
            match *eff {
                Op::Gcd(g) => {
                    if val.size == 1 {
                        res = Rucms::new(gcd(g, val.max));
                    } else if g % val.lcm != 0 {
                        res.fail = true;
                    }
                }
                Op::Update(u) => {
                    res.max = u;
                    res.lcm = u;
                    res.sum = u * res.size;
                }
                Op::None => ()
            };
            res
        }
    }

    #[test]
    fn segtree_beats_update_chgcd_max_sum() {
        let lis = vec![1, 6, 8, 7, 3];
        let mut seg = SegmentTreeBeats::<RangeUpdateChgcdMaxSum>::from(
            &lis.into_iter().map(Rucms::new).collect()
        );
        assert_eq!(seg.fold(0..5).max, 8);
        assert_eq!(seg.fold(0..5).sum, 25);
        seg.update(0..5, Op::Gcd(6));
        assert_eq!(seg.fold(0..5).max, 6);
        assert_eq!(seg.fold(1..4).sum, 9);
        seg.update(0..5, Op::Update(10));
        assert_eq!(seg.fold(1..4).max, 10);
        assert_eq!(seg.fold(2..5).sum, 30);
        seg.update(2..4, Op::Gcd(3));
        assert_eq!(seg.fold(1..3).max, 10);
        assert_eq!(seg.fold(3..5).sum, 11);
    }
}
