use crate::utils::algebraic_traits::Element;

// ------------ DoublePriorityHeap start ------------

#[derive(Default)]
pub struct DoublePriorityHeap<T: Element + Ord>(Vec<T>);

impl<T: Element + Ord> DoublePriorityHeap<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from(vec: &[T]) -> Self {
        let mut l = Self(vec.to_vec());
        l.build();
        l
    }

    pub fn push(&mut self, x: T) {
        self.0.push(x);
        self.up(self.0.len() - 1, 1);
    }

    pub fn pop_min(&mut self) -> Option<T> {
        if self.0.len() < 3 {
            self.0.pop()
        } else {
            let ret = self.0.swap_remove(1);
            let k = self.down(1);
            self.up(k, 1);
            Some(ret)
        }
    }

    pub fn pop_max(&mut self) -> Option<T> {
        if self.0.len() < 2 {
            self.0.pop()
        } else {
            let ret = self.0.swap_remove(0);
            let k = self.down(0);
            self.up(k, 1);
            Some(ret)
        }
    }

    pub fn get_min(&self) -> Option<&T> {
        if self.0.len() < 2 {
            self.0.get(0)
        } else {
            self.0.get(1)
        }
    }

    pub fn get_max(&self) -> Option<&T> {
        self.0.get(0)
    }

    fn build(&mut self) {
        let n = self.0.len();
        for i in (0..n).rev() {
            if i & 1 == 1 && self.0[i - 1] < self.0[i] {
                self.0.swap(i - 1, i);
            }
            let k = self.down(i);
            self.up(k, i);
        }
    }

    #[inline]
    fn parent(k: usize) -> usize {
        (k >> 1).wrapping_sub(1) & !1
    }

    fn down(&mut self, mut k: usize) -> usize {
        let n = self.0.len();
        let mut c: usize;
        if k & 1 == 1 {
            // min heap
            while 2 * k + 1 < n {
                c = 2 * k + 3;
                if n <= c || self.0[c - 2] < self.0[c] {
                    c -= 2;
                }
                if c < n && self.0[c] < self.0[k] {
                    self.0.swap(k, c);
                    k = c;
                } else {
                    break;
                }
            }
        } else {
            // max heap
            while 2 * k + 2 < n {
                c = 2 * k + 4;
                if n <= c || self.0[c] < self.0[c - 2] {
                    c -= 2;
                }
                if c < n && self.0[k] < self.0[c] {
                    self.0.swap(k, c);
                    k = c;
                } else {
                    break;
                }
            }
        }
        k
    }

    fn up(&mut self, mut k: usize, root: usize) {
        if (k | 1) < self.0.len() && self.0[k & !1] < self.0[k | 1] {
            self.0.swap(k & !1, k | 1);
            k ^= 1;
        }
        let mut p = Self::parent(k);
        // max heap
        while root < k && self.0[p] < self.0[k] {
            self.0.swap(k, p);
            k = p;
            p = Self::parent(k)
        }
        // min heap
        p |= 1;
        while root < k && self.0[k] < self.0[p] {
            self.0.swap(k, p);
            k = p;
            p = Self::parent(k) | 1;
        }
    }
}

// ------------ DoublePriorityHeap end ------------

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_i32() {
        let mut hq = DoublePriorityHeap::<i32>::new();
        for i in 0..6 {
            hq.push(i);
        }
        assert_eq!(hq.get_min(), Some(&0));
        assert_eq!(hq.get_max(), Some(&5));
        assert_eq!(hq.pop_min(), Some(0));
        assert_eq!(hq.pop_min(), Some(1));
        assert_eq!(hq.get_min(), Some(&2));
        for i in 0..6 {
            hq.push(i);
        }
        assert_eq!(hq.pop_max(), Some(5));
        assert_eq!(hq.get_max(), Some(&5));
        assert_eq!(hq.pop_min(), Some(0));
    }

    #[test]
    fn build_u64() {
        let v = (0..8).collect::<Vec<u64>>();
        let mut hq = DoublePriorityHeap::<u64>::from(&v);
        assert_eq!(hq.get_min(), Some(&0));
        assert_eq!(hq.get_max(), Some(&7));
        assert_eq!(hq.pop_min(), Some(0));
        assert_eq!(hq.pop_min(), Some(1));
        assert_eq!(hq.get_min(), Some(&2));
        for i in 0..6 {
            hq.push(i);
        }
        assert_eq!(hq.pop_max(), Some(7));
        assert_eq!(hq.get_max(), Some(&6));
        assert_eq!(hq.pop_min(), Some(0));
    }
}
