use crate::utils::algebraic_traits::SemiGroup;

// * verified: https://judge.yosupo.jp/submission/28320
// ------------ module start ------------
use std::ops::Range;
use std::iter::successors;

pub struct DisjointSparseTable<T: SemiGroup>(Vec<Vec<T>>);

impl<T: SemiGroup> DisjointSparseTable<T> {
    pub fn fold(&self, rng: Range<usize>) -> T {
        let l = rng.start;
        let r = rng.end - 1;
        assert!(l <= r && r < self.0[0].len(), "index out of range: {}..{}", l, r + 1);
        if l  == r {
            self.0[0][l].clone()
        } else {
            let p = (std::usize::MAX.count_ones() - (l ^ r).leading_zeros() - 1) as usize;
            self.0[p][l ^ ((1 << p) - 1)].clone() + self.0[p][r].clone()
        }
    }
}

impl<T: SemiGroup> From<&Vec<T>> for DisjointSparseTable<T> {
    fn from(vec: &Vec<T>) -> Self {
        let size = vec.len();
        let mut table = Vec::with_capacity(31);
        table.push(vec.clone());
        for i in successors(Some(2), |&x| Some(x << 1)).take_while(|&x| x < size) {
            let mut l = Vec::with_capacity(size);
            for j in successors(Some(i), |&x| Some(x + (i << 1))).take_while(|&x| x < size) {
                l.push(table[0][j-1].clone());
                for k in 2..=i {
                    l.push(table[0][j-k].clone() + l.last().unwrap().clone());
                }
                l.push(table[0][j].clone());
                for k in (1..i).take_while(|x| x + j < size) {
                    l.push(l.last().unwrap().clone() + table[0][j + k].clone());
                }

            }
            table.push(l);
        }
        Self(table)
    }
}

// ------------ module end ------------

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
	use crate::utils::algebraic_traits::*;
    use std::ops::Add;
    use std::cmp::min;

	#[derive(Debug, Clone, PartialEq)]
    struct Am(i32);

    impl Add for Am {
		type Output = Self;
        fn add(self, right: Self) -> Self { Am(min(self.0, right.0)) }
	}

    impl Associative for Am {}

    #[test]
    fn rmq_test() {
        let dsp = DisjointSparseTable::from(&vec![Am(4), Am(2), Am(3)]);
        assert!(dsp.fold(0..1).0 == 4);
        assert!(dsp.fold(0..2).0 == 2);
        assert!(dsp.fold(0..3).0 == 2);
        assert!(dsp.fold(1..2).0 == 2);
        assert!(dsp.fold(1..3).0 == 2);
        assert!(dsp.fold(2..3).0 == 3);
	}

	#[test]
    fn corner_test() {
        let dsp = DisjointSparseTable::from(&vec![Am(1)]);
        assert!(dsp.fold(0..1).0 == 1);
    }

    #[test]
    fn random_one() {
        let mut rng = rand::thread_rng();
        let a = (0..10000).map(|_| Am(rng.gen())).collect::<Vec<Am>>();
        let dsp = DisjointSparseTable::from(&a);
        for _ in 0..10000 {
            let l = rng.gen_range(0, 10000);
            let r = rng.gen_range(l+1, 10001);
            assert_eq!(dsp.fold(l..r), (l..r).fold(Am(std::i32::MAX), |sum, x| sum + a[x].clone()));
        }

    }
}