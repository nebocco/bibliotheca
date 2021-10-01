// verified: https://judge.yosupo.jp/submission/61203

#[derive(Clone, Debug)]
pub struct SuccinctIndexableDictionary {
    length: usize,
    blocks: usize,
    bit: Vec<u32>,
    sum: Vec<u32>,
}

impl SuccinctIndexableDictionary {
    pub fn new(length: usize) -> Self {
        let blocks = (length + 31) >> 5;
        Self {
            length,
            blocks,
            bit: vec![0; blocks],
            sum: vec![0; blocks],
        }
    }

    pub fn set(&mut self, k: usize) {
        self.bit[k >> 5] |= 1 << (k & 31);
    }

    pub fn build(&mut self) {
        for i in 1..self.blocks {
            self.sum[i] = self.sum[i - 1] + self.bit[i - 1].count_ones();
        }
    }

    fn _rank(&self, k: usize) -> usize {
        (self.sum[k >> 5] + (self.bit[k >> 5] & ((1 << (k & 31)) - 1)).count_ones()) as usize
    }

    pub fn rank(&self, f: bool, k: usize) -> usize {
        if f {
            self._rank(k)
        } else {
            k - self._rank(k)
        }
    }
}

impl std::ops::Index<usize> for SuccinctIndexableDictionary {
    type Output = bool;
    fn index(&self, k: usize) -> &Self::Output {
        if self.bit[k >> 5] >> (k & 31) & 1 == 1 {
            &true
        } else {
            &false
        }
    }
}

use std::ops::Range;

pub struct WaveletMatrix {
    _length: usize,
    matrix: Vec<SuccinctIndexableDictionary>,
    mid: Vec<usize>,
}

impl WaveletMatrix {
    const MAXLOG: usize = 32;

    pub fn new(mut vec: Vec<u32>) -> Self {
        let length = vec.len();
        let mut l = Vec::with_capacity(length);
        let mut r = Vec::with_capacity(length);
        let mut matrix = vec![SuccinctIndexableDictionary::new(length + 1); Self::MAXLOG];
        let mut mid = vec![0; Self::MAXLOG];
        for level in (0..Self::MAXLOG).rev() {
            for i in 0..length {
                if vec[i] >> level & 1 == 1 {
                    matrix[level].set(i);
                    r.push(vec[i]);
                } else {
                    l.push(vec[i]);
                }
            }
            matrix[level].build();
            mid[level] = l.len();
            vec.clear();
            vec.append(&mut l);
            vec.append(&mut r);
        }
        Self {
            _length: length,
            matrix,
            mid,
        }
    }

    #[inline]
    fn _succ(&self, f: bool, k: usize, level: usize) -> usize {
        self.matrix[level].rank(f, k) + if f { self.mid[level] } else { 0 }
    }

    pub fn succ(&self, f: bool, l: usize, r: usize, level: usize) -> (usize, usize) {
        (self._succ(f, l, level), self._succ(f, r, level))
    }

    /// vec[k]
    pub fn access(&self, mut k: usize) -> u32 {
        let mut ret = 0;
        for level in (0..Self::MAXLOG).rev() {
            let f = self.matrix[level][k];
            if f {
                ret |= 1 << level;
            }
            k = self._succ(f, k, level);
        }
        ret
    }

    /// vec[..r].filter(|&x| x == val).count()
    pub fn rank(&self, val: u32, mut r: usize) -> usize {
        let mut l = 0;
        for level in (0..Self::MAXLOG).rev() {
            let (ll, rr) = self.succ(val >> level & 1 == 1, l, r, level);
            l = ll;
            r = rr;
        }
        r - l
    }

    /// k-th(0-indexed) smallest element in vec[rng]
    pub fn kth_smallest(&self, rng: Range<usize>, mut k: usize) -> u32 {
        assert!(k < rng.len());
        let mut l = rng.start;
        let mut r = rng.end;
        let mut ret = 0;
        for level in (0..Self::MAXLOG).rev() {
            let cnt = self.matrix[level].rank(false, r) - self.matrix[level].rank(false, l);
            let f = cnt <= k;
            if f {
                ret |= 1 << level;
                k -= cnt;
            }
            let (ll, rr) = self.succ(f, l, r, level);
            l = ll;
            r = rr;
        }
        ret
    }

    pub fn kth_largest(&self, rng: Range<usize>, k: usize) -> u32 {
        let l = rng.start;
        let r = rng.end;
        self.kth_smallest(rng, r - l - k - 1)
    }

    /// vec[rng].iter().filter(|&x| x < upper).count()
    pub fn _range_freq(&self, rng: Range<usize>, upper: u32) -> usize {
        let mut l = rng.start;
        let mut r = rng.end;
        let mut ret = 0;
        for level in (0..Self::MAXLOG).rev() {
            let f = upper >> level & 1 == 1;
            if f {
                ret += self.matrix[level].rank(false, r) - self.matrix[level].rank(false, l);
            }
            let (ll, rr) = self.succ(f, l, r, level);
            l = ll;
            r = rr;
        }
        ret
    }

    /// vec[rng].iter().filter(|&x| x in target).count()
    pub fn range_freq(&self, rng: Range<usize>, target: Range<u32>) -> usize {
        self._range_freq(rng.clone(), target.end) - self._range_freq(rng, target.start)
    }

    /// vec[rng].iter().filter(|&x| x < upper).max()
    pub fn prev_value(&self, rng: Range<usize>, upper: u32) -> Option<u32> {
        let cnt = self._range_freq(rng.clone(), upper);
        if cnt == 0 {
            None
        } else {
            Some(self.kth_smallest(rng, cnt - 1))
        }
    }

    /// vec[rng].iter().filter(|&x| x >= lower).min()
    pub fn next_value(&self, rng: Range<usize>, lower: u32) -> Option<u32> {
        let cnt = self._range_freq(rng.clone(), lower);
        if cnt == rng.len() {
            None
        } else {
            Some(self.kth_smallest(rng, cnt))
        }
    }
}

pub struct CompressedWaveletMatrix<T: Clone + Ord> {
    matrix: WaveletMatrix,
    vec: Vec<T>,
}

impl<T: Clone + Ord> CompressedWaveletMatrix<T> {
    pub fn new(vec: &Vec<T>) -> Self {
        let mut ys = vec.clone();
        ys.sort();
        ys.dedup();
        let vec: Vec<u32> = vec
            .iter()
            .map(|x| ys.binary_search(x).unwrap() as u32)
            .collect();
        let matrix = WaveletMatrix::new(vec.clone());
        Self { matrix, vec: ys }
    }

    pub fn access(&self, k: usize) -> &T {
        &self.vec[self.matrix.access(k) as usize]
    }

    pub fn rank(&self, x: &T, r: usize) -> usize {
        if let Ok(pos) = self.vec.binary_search(x) {
            self.matrix.rank(pos as u32, r)
        } else {
            0
        }
    }

    pub fn kth_smallest(&self, rng: Range<usize>, k: usize) -> &T {
        &self.vec[self.matrix.kth_smallest(rng, k) as usize]
    }

    pub fn kth_largest(&self, rng: Range<usize>, k: usize) -> &T {
        &self.vec[self.matrix.kth_largest(rng, k) as usize]
    }

    fn _range_freq(&self, rng: Range<usize>, upper: &T) -> usize {
        let upper = match self.vec.binary_search(upper) {
            Ok(x) => x,
            Err(x) => x,
        } as u32;
        self.matrix._range_freq(rng, upper)
    }

    pub fn range_freq(&self, rng: Range<usize>, target: Range<T>) -> usize {
        let upper = &target.start;
        let lower = &target.end;
        let upper = match self.vec.binary_search(upper) {
            Ok(x) => x,
            Err(x) => x,
        } as u32;
        let lower = match self.vec.binary_search(lower) {
            Ok(x) => x,
            Err(x) => x,
        } as u32;
        self.matrix.range_freq(rng, lower..upper)
    }

    pub fn prev_value(&self, rng: Range<usize>, upper: &T) -> Option<&T> {
        let upper = match self.vec.binary_search(upper) {
            Ok(x) => x,
            Err(x) => x,
        } as u32;
        self.matrix
            .prev_value(rng, upper)
            .map(|x| &self.vec[x as usize])
    }

    pub fn next_value(&self, rng: Range<usize>, lower: &T) -> Option<&T> {
        let lower = match self.vec.binary_search(lower) {
            Ok(x) => x,
            Err(x) => x,
        } as u32;
        self.matrix
            .next_value(rng, lower)
            .map(|x| &self.vec[x as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wavelet_matrix() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        let wm = WaveletMatrix::new(vec);

        assert_eq!(wm.access(0), 3);
        assert_eq!(wm.access(4), 5);

        assert_eq!(wm.kth_smallest(0..4, 1), 1);
        assert_eq!(wm.kth_smallest(3..8, 3), 6);

        assert_eq!(wm._range_freq(0..4, 2), 2);
        assert_eq!(wm._range_freq(3..9, 7), 5);

        assert_eq!(wm.range_freq(0..5, 0..3), 2);
        assert_eq!(wm.range_freq(3..9, 4..7), 3);

        assert_eq!(wm.prev_value(0..5, 4), Some(3));
        assert_eq!(wm.prev_value(0..5, 1), None);

        assert_eq!(wm.next_value(3..9, 2), Some(2));
        assert_eq!(wm.next_value(3..9, 12), None);
    }

    // TODO: make tests for compressed
}
