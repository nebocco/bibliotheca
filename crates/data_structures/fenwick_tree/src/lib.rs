// TODO: use generics

pub struct Fenwick(Vec<i64>);

impl Fenwick {
    pub fn new(len: usize) -> Self {
        Fenwick(vec![0; len + 1])
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

#[cfg(test)]
mod tests {
    use super::Fenwick;

    #[test]
    fn test_0() {
        let mut bit = Fenwick::new(5);
        bit.add(0, 1);
        bit.add(1, 2);
        assert_eq!(bit.prefix_sum(1), 1);
        assert_eq!(bit.prefix_sum(2), 3);
        assert_eq!(bit.prefix_sum(3), 3);
        bit.add(2, 4);
        bit.add(3, 8);
        bit.add(4, 16);
        assert_eq!(bit.prefix_sum(5), 31);
        bit.set(0, 5);
        assert_eq!(bit.prefix_sum(3), 11);
        assert_eq!(bit.access(0), 5);
        assert_eq!(bit.access(1), 2);
        assert_eq!(bit.access(2), 4);
        assert_eq!(bit.access(3), 8);
        assert_eq!(bit.access(4), 16);
    }

    #[test]
    fn test_1() {
        let a = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        let bit = Fenwick::build_from_slice(&a);
        for i in 0..9 {
            assert_eq!(bit.prefix_sum(i), a[..i].iter().sum::<i64>())
        }

        assert_eq!(bit.lower_bound(7), 2);
        assert_eq!(bit.lower_bound(10), 4);
        assert_eq!(bit.lower_bound(14), 4);
        assert_eq!(bit.upper_bound(14), 5);
        assert_eq!(bit.lower_bound(15), 5);
    }
}
