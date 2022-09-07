/*

https://github.com/ngtkana/ac-adapter-rs/blob/master/crates/algolib/sparse_table/src/lib.rs

*/

use crate::utils::bounds::bounds_within;

// ------------ SparseTable start ------------

use std::ops::{Range, RangeBounds};

pub trait IdempotenceSemiGroup {
    type Val: Clone;
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}

enum ArgMin {}

impl IdempotenceSemiGroup for ArgMin {
    type Val = (usize, i64);
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        if left.1 > right.1 {
            *right
        } else {
            *left
        }
    }
}

#[derive(Debug, Clone)]
pub struct SparseTable<T: IdempotenceSemiGroup>(Vec<Vec<T::Val>>);

impl<T: IdempotenceSemiGroup> SparseTable<T> {
    pub fn query(&self, range: impl RangeBounds<usize>) -> T::Val {
        let Range { start, end } = bounds_within(range, self.0[0].len());
        let d = (end - start).next_power_of_two() >> 1;
        let row = &self.0[d.trailing_zeros() as usize];
        T::op(&row[start], &row[end - d])
    }
}

impl<T> std::iter::FromIterator<T::Val> for SparseTable<T>
where
    T: IdempotenceSemiGroup,
{
    fn from_iter<I: IntoIterator<Item = T::Val>>(iter: I) -> Self {
        let seq: Vec<T::Val> = iter.into_iter().collect();
        let n = seq.len();
        let mut table = Vec::with_capacity(n.next_power_of_two().trailing_zeros() as usize + 1);
        table.push(seq);
        let mut d = 1;
        while (d << 1) < n {
            let mut seq = table.last().unwrap().clone();
            for i in 0..n - d {
                seq[i] = T::op(&seq[i], &seq[i + d]);
            }
            table.push(seq);
            d <<= 1;
        }
        Self(table)
    }
}

// ------------ SparseTable end ------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_hand() {
        let a: &[i64] = &[4, 3, 5, 1, 3, 2];
        let spt = SparseTable::<ArgMin>::from_iter(a.iter().copied().enumerate());
        assert_eq!(spt.query(5..6), (5, 2));
        assert_eq!(spt.query(1..3), (1, 3));
        assert_eq!(spt.query(1..5), (3, 1));
        assert_eq!(spt.query(0..6), (3, 1));
        assert_eq!(spt.query(0..=3), (3, 1));
        assert_eq!(spt.query(0..=2), (1, 3));
        assert_eq!(spt.query(..), (3, 1));
    }
}
