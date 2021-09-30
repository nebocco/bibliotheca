// written by tmaehara
// https://judge.yosupo.jp/submission/19876

pub struct AssignmentProblem {
    row_size: usize,
    col_size: usize,
    cost: Vec<Vec<i64>>,
    potential: Vec<i64>,
    col_match: Vec<Option<usize>>,
    row_match: Vec<Option<usize>>,
}
impl AssignmentProblem {
    pub fn new(cost: Vec<Vec<i64>>) -> Self {
        let row_size = cost.len();
        let col_size = cost[0].len();
        let mut solver = Self {
            row_size,
            col_size,
            cost,
            potential: vec![0i64; col_size],
            row_match: vec![None; row_size],
            col_match: vec![None; col_size],
        };
        solver.initialize();
        solver.augment();
        solver
    }
    fn residual(&self, row: usize, col: usize) -> i64 {
        self.cost[row][col] - self.potential[col]
    }
    fn initialize(&mut self) {
        // column reduction
        let mut transferrable = vec![false; self.row_size];
        for col in 0..self.col_size {
            let row = (0..self.row_size)
                .min_by(|&r1, &r2| self.cost[r1][col].partial_cmp(&self.cost[r2][col]).unwrap())
                .unwrap();
            self.potential[col] = self.cost[row][col];
            if self.row_match[row].is_none() {
                self.col_match[col] = Some(row);
                self.row_match[row] = Some(col);
                transferrable[row] = true;
            } else {
                transferrable[row] = false;
            }
        }
        // reduction transfer
        for row in 0..self.row_size {
            if transferrable[row] {
                let col = self.row_match[row].unwrap();
                let c = (0..self.col_size)
                    .filter(|&c| c != col)
                    .min_by(|&c1, &c2| {
                        self.residual(row, c1)
                            .partial_cmp(&self.residual(row, c2))
                            .unwrap()
                    })
                    .unwrap();
                self.potential[col] -= self.residual(row, c);
            }
        }
        // augmenting row reduction
        for _ in 0..2 {
            for row in 0..self.row_size {
                if self.row_match[row].is_some() {
                    continue;
                }
                let (u1, u2, c1) = (0..self.col_size).fold(
                    (self.residual(row, 0), std::i64::MAX, 0),
                    |(u1, u2, c1), c| {
                        let u = self.residual(row, c);
                        if u < u1 || (u == u1 && self.col_match[c1].is_some()) {
                            (u, u1, c)
                        } else if u < u2 {
                            (u1, u, c1)
                        } else {
                            (u1, u2, c1)
                        }
                    },
                );
                if u1 < u2 {
                    self.potential[c1] -= u2 - u1;
                }
                if let Some(r1) = self.col_match[c1].take() {
                    self.row_match[r1] = None;
                }
                self.row_match[row] = Some(c1);
                self.col_match[c1] = Some(row);
            }
        }
    }
    fn augment(&mut self) {
        let mut cols = (0..self.col_size).collect::<Vec<_>>();
        for row in 0..self.row_size {
            if self.row_match[row].is_some() {
                continue;
            }
            let mut dist = (0..self.col_size)
                .map(|c| self.residual(row, c))
                .collect::<Vec<_>>();
            let mut pred = vec![row; self.col_size];

            let mut scanned = 0;
            let mut labeled = 0;
            let mut last = 0;
            let col = 'repeat: loop {
                if scanned == labeled {
                    last = scanned;
                    let mut min = dist[cols[scanned]];
                    for j in scanned..cols.len() {
                        let c = cols[j];
                        if dist[c] <= min {
                            if dist[c] < min {
                                min = dist[c];
                                labeled = scanned;
                            }
                            cols.swap(j, labeled);
                            labeled += 1;
                        }
                    }
                    for &c in &cols[scanned..labeled] {
                        if self.col_match[c].is_none() {
                            break 'repeat c;
                        }
                    }
                }
                let c1 = cols[scanned];
                scanned += 1;
                let r1 = self.col_match[c1].unwrap();
                for j in labeled..cols.len() {
                    let c2 = cols[j];
                    let len = self.residual(r1, c2) - self.residual(r1, c1);
                    assert!(len >= 0);
                    if dist[c1] + len < dist[c2] {
                        dist[c2] = dist[c1] + len;
                        pred[c2] = r1;
                        if len == 0i64 {
                            if self.col_match[c2].is_none() {
                                break 'repeat c2;
                            }
                            cols.swap(labeled, j);
                            labeled += 1;
                        }
                    }
                }
            };
            for &c in &cols[..last] {
                self.potential[c] += dist[c] - dist[col];
            }
            let mut curr = Some(col);
            while let Some(col) = curr {
                let row = pred[col];
                self.col_match[col] = Some(row);
                std::mem::swap(&mut self.row_match[row], &mut curr);
            }
        }
    }
    pub fn solution(&mut self) -> (i64, Vec<(usize, usize)>) {
        let mut opt = 0i64;
        let mut sol = vec![];
        for row in 0..self.row_size {
            let col = self.row_match[row].unwrap();
            opt += self.cost[row][col];
            sol.push((row, col));
        }
        (opt, sol)
    }
}
