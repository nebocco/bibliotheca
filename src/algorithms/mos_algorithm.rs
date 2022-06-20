use std::ops::Range;
pub trait MoState {
    type A;
    type Q;

    fn query(&self, q: &Self::Q) -> Self::A;

    fn push_left(&mut self, idx: usize);

    fn push_right(&mut self, idx: usize) {
        self.push_left(idx);
    }

    fn pop_left(&mut self, idx: usize);

    fn pop_right(&mut self, idx: usize) {
        self.pop_left(idx);
    }

    fn solve(&mut self, queries: &[(Range<usize>, Self::Q)]) -> Vec<Self::A> {
        let q = queries.len();
        let mut q_positions: Vec<usize> = (0..q).collect();
        if let Some(max_r) = queries.iter().map(|(rng, _)| rng.end).max() {
            let q_adjusted = q as f64;
            let bucket_width = 1 + max_r / q_adjusted.sqrt() as usize;
            q_positions.sort_unstable_by_key(|&i| {
                let rng = &queries[i].0;
                let bucket = rng.start / bucket_width;
                let r = if bucket & 1 == 0 {
                    rng.end
                } else {
                    max_r - rng.end
                };
                (bucket, r)
            });
        }

        let (mut cur_l, mut cur_r) = (0, 0);
        let mut answers = Vec::with_capacity(queries.len());
        for i in q_positions {
            let (rng, q) = &queries[i];
            let (l, r) = (rng.start, rng.end);
            while cur_l > l {
                cur_l -= 1;
                self.push_left(cur_l);
            }
            while cur_r < r {
                self.push_right(cur_r);
                cur_r += 1;
            }
            while cur_l < l {
                self.pop_left(cur_l);
                cur_l += 1;
            }
            while cur_r > r {
                cur_r -= 1;
                self.pop_right(cur_r);
            }
            answers.push((i, self.query(q)));
        }
        answers.sort_unstable_by_key(|&(i, _)| i);
        answers.into_iter().map(|(_, ans)| ans).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::BTreeMap;

    pub struct ValueModality {
        values: Vec<usize>,
        btree: BTreeMap<usize, usize>,
        modality: usize,
    }
    
    impl ValueModality {
        pub fn new(values: Vec<usize>) -> Self {
            Self {
                values,
                btree: BTreeMap::new(),
                modality: 0,
            }
        }
    }
    
    impl MoState for ValueModality {
        type Q = ();
        type A = usize;
    
        fn query(&self, _: &Self::Q) -> Self::A {
            self.modality
        }
    
        fn push_left(&mut self, idx: usize) {
            let v = self.values[idx];
            let e = self.btree.entry(v).or_insert(0);
            if *e == 0 {
                self.modality += 1;
            }
            *e += 1;
        }
    
        fn pop_left(&mut self, idx: usize) {
            let v = self.values[idx];
            let e = self.btree.entry(v).or_insert(0);
            *e -= 1;
            if *e == 0 {
                self.modality -= 1;
            }
        }
    }

    #[test]
    fn test_mos_algorithm() {
        let queries = vec![(0..3, ()), (1..6, ()), (6..7, ()), (8..11, ())];
        let values = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

        let answers = ValueModality::new(values).solve(&queries);

        assert_eq!(answers, vec![3, 4, 1, 2]);
    }
}