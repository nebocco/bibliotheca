// TODO: use generics

pub struct Fenwick(Vec<i64>);

impl Fenwick {
    pub fn new(len: usize) -> Self {
        Fenwick(vec![0; len])
    }

    pub fn build_from_slice(src: &[i64]) -> Self {
        let mut table = std::iter::once(0)
            .chain(src.iter().cloned())
            .collect::<Vec<i64>>();
        let n = table.len();
        (1..n)
            .map(|i| (i, i + lsb(i)))
            .filter(|&(_, j)| j < n)
            .for_each(|(i, j)| {
                table[j] += table[i];
            });
        Self(table)
    }

    pub fn prefix_sum(&self, i: usize) -> i64 {
        std::iter::successors(Some(i), |&i| Some(i - lsb(i)))
            .take_while(|&i| i != 0)
            .map(|i| self.0[i])
            .sum::<i64>()
    }

    pub fn add(&mut self, i: usize, x: i64) {
        let n = self.0.len();
        std::iter::successors(Some(i + 1), |&i| Some(i + lsb(i)))
            .take_while(|&i| i < n)
            .for_each(|i| self.0[i] += x);
    }

    fn partition(&self, pred: impl Fn(usize, i64) -> bool) -> (usize, i64) {
        let mut j = 0;
        let mut current = self.0[0];
        let n = self.0.len();
        for d in std::iter::successors(Some(n.next_power_of_two() >> 1), |&d| { Some(d >> 1)})
            .take_while(|&d| d != 0)
        {
            assert!(pred(0, self.0[0]), "need to be pred(0, 0)");
            if j + d < n {
                let next = current + self.0[j + d];
                if pred(j + d, next) {
                    current = next;
                    j += d;
                }
            }
        }
        (j, current)
    }

    pub fn lower_bound(&self, x: i64) -> usize {
        self.partition(|_, y| y < x).0
    }

    pub fn upper_bound(&self, x: i64) -> usize {
        self.partition(|_, y| y <= x).0
    }

    pub fn access(&self, i: usize) -> i64 {
        assert!(i < self.0.len() - 1, "index out of range");
        self.prefix_sum(i + 1) - self.prefix_sum(i)
    }

    pub fn set(&mut self, i: usize, x: i64) {
        self.add(i, x - self.access(i));
    }

}

fn lsb(x: usize) -> usize {
    x & x.wrapping_neg()
}

// use sd::ops;

// #[derive(Clone, Debug)]
// pub struct FenwickTree<T> {
//     table: Vec<T>
// }

// impl<'a, T> FenwickTree<T> where
//     T: Clone + std::fmt::Debug + ops::Add<Output = T> + ops::AddAssign + std::iter::Sum<T>,
// {
//     pub fn new(zero: T) -> Self {
//         FenwickTree {
//             table: vec![zero]
//         }
//     }
// }

// TODO: create tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
