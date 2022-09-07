// * verified: https://judge.yosupo.jp/submission/28320
// ------------ Disjoint SparseTable start ------------
use std::iter::successors;
use std::ops::Range;

pub trait SemiGroup {
    type Val: Clone;
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}

pub struct DisjointSparseTable<G: SemiGroup>(Vec<Vec<G::Val>>);

impl<G: SemiGroup> DisjointSparseTable<G> {
    pub fn fold(&self, rng: Range<usize>) -> G::Val {
        let l = rng.start;
        let r = rng.end - 1;
        assert!(
            l <= r && r < self.0[0].len(),
            "index out of range: {}..{}",
            l,
            r + 1
        );
        if l == r {
            self.0[0][l].clone()
        } else {
            let p = (std::usize::MAX.count_ones() - (l ^ r).leading_zeros() - 1) as usize;
            G::op(&self.0[p][l ^ ((1 << p) - 1)], &self.0[p][r])
        }
    }
}

impl<G: SemiGroup> From<&[G::Val]> for DisjointSparseTable<G> {
    fn from(vec: &[G::Val]) -> Self {
        let size = vec.len();
        let mut table = Vec::with_capacity(31);
        table.push(vec.to_vec());
        for i in successors(Some(2), |&x| Some(x << 1)).take_while(|&x| x < size) {
            let mut l = Vec::with_capacity(size);
            for j in successors(Some(i), |&x| Some(x + (i << 1))).take_while(|&x| x < size) {
                l.push(table[0][j - 1].clone());
                for k in 2..=i {
                    l.push(G::op(&table[0][j - k], &l.last().unwrap()));
                }
                l.push(table[0][j].clone());
                for k in (1..i).take_while(|x| x + j < size) {
                    l.push(G::op(&l.last().unwrap(), &table[0][j + k]));
                }
            }
            table.push(l);
        }
        Self(table)
    }
}

// ------------ Disjoint SparseTable end ------------

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    enum Am {}

    impl SemiGroup for Am {
        type Val = i32;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            *left.min(right)
        }
    }

    #[test]
    fn rmq_test() {
        let dsp = DisjointSparseTable::<Am>::from([4, 2, 3].as_slice());
        assert!(dsp.fold(0..1) == 4);
        assert!(dsp.fold(0..2) == 2);
        assert!(dsp.fold(0..3) == 2);
        assert!(dsp.fold(1..2) == 2);
        assert!(dsp.fold(1..3) == 2);
        assert!(dsp.fold(2..3) == 3);
    }

    #[test]
    fn corner_test() {
        let dsp = DisjointSparseTable::<Am>::from([1].as_slice());
        assert!(dsp.fold(0..1) == 1);
    }

    #[test]
    fn random_one() {
        let mut rng = rand::thread_rng();
        let a = (0..10000).map(|_| rng.gen()).collect::<Vec<i32>>();
        let dsp = DisjointSparseTable::<Am>::from(a.as_slice());
        for _ in 0..10000 {
            let l = rng.gen_range(0..10000);
            let r = rng.gen_range(l + 1..10001);
            assert_eq!(
                dsp.fold(l..r),
                (l..r).fold(std::i32::MAX, |sum, x| sum.min(a[x]))
            );
        }
    }
}
