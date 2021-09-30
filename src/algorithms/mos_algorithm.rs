pub struct Mo {
    data: (),
    queries: Vec<(usize, usize)>,
    range: std::ops::Range<usize>
}

impl Mo {
    pub fn new() -> Self {
        Self {
            data: (),
            queries: Vec::new(),
            range: 0..0
        }
    }

    pub fn add(&mut self, query: (usize, usize)) {
        self.queries.push(query);
        self.range = self.range.start.min(query.0)..self.range.end.max(query.1);
    }

    pub fn set_queries(&mut self, queries: Vec<(usize, usize)>) {
        self.queries = queries;
    }

    pub fn push_left(&mut self, idx: usize) {
        &self.data; &idx;
    }

    pub fn push_right(&mut self, idx: usize) {
        &self.data; &idx;
    }

    pub fn pop_left(&mut self, idx: usize) {
        &self.data; &idx;
    }

    pub fn pop_right(&mut self, idx: usize) {
        &self.data; &idx;
    }

    pub fn solve(&mut self) -> Vec<()> {
        let q = self.queries.len();
        let n = self.range.len();
        let mut ans = vec![(); q];
        let bs = n / n.min((q as f64 + 1.).sqrt().ceil() as usize);
        let mut indice: Vec<usize> = (0..q).collect();
        indice.sort_by(|&i, &j| {
            let bi = self.queries[i].0 / bs;
            let bj = self.queries[j].0 / bs;
            bi.cmp(&bj)
                .then(if bi & 1 == 0 {
                    self.queries[i].1.cmp(&self.queries[j].1)
                } else {
                    self.queries[j].1.cmp(&self.queries[i].1)    
                })
        });
        let mut l = 0;
        let mut r = 0;
        for idx in indice {
            let (nl, nr) = self.queries[idx];
            while l > nl {
                l -= 1;
                self.push_left(l);
            }
            while r < nr {
                self.push_right(r);
                r += 1;
            }
            while l < nl {
                self.pop_left(l);
                l += 1;
            }
            while r > nr {
                r -= 1;
                self.pop_right(r);
            }
            ans[idx] = ();
        }
        ans
    }
}