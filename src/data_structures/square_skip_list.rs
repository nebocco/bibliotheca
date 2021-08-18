// TODO: verify
// ------------ Square Skip List start ------------
struct XorShift(i64);

impl XorShift {
    fn new(seed: i64) -> Self {
        Self(seed)
    }

    fn next(&mut self) -> i64 {
        self.0 ^= (self.0 & 0x0007_ffff) << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= (self.0 & 0x07ff_ffff) << 5;
        self.0
    }
}

pub struct SquareSkipList {
    square: i64,
    layer0: Vec<Vec<i64>>,
    layer1: Vec<i64>,
    rand: XorShift,
}

impl Default for SquareSkipList {
    fn default() -> Self {
        Self::new()
    }
}

impl SquareSkipList {
    pub fn new() -> Self {
        let square = 1000;
        let layer0 = vec![Vec::new()];
        let layer1 = vec![std::i64::MAX];
        let rand = XorShift::new(42);
        Self {
            square,
            layer0,
            layer1,
            rand,
        }
    }

    pub fn push(&mut self, x: i64) {
        let idx1 = self.layer1.binary_search(&x).unwrap_or_else(|s| s);
        let idx0 = self.layer0[idx1].binary_search(&x).unwrap_or_else(|s| s);
        if self.rand.next() % self.square == 0 {
            self.layer1.insert(idx1, x);
            let vec1 = self.layer0[idx1].split_off(idx0);
            self.layer0.insert(idx1 + 1, vec1);
        } else {
            self.layer0[idx1].insert(idx0, x);
        }
    }

    // if x not in list ...
    pub fn remove(&mut self, x: i64) -> Result<(), ()> {
        let idx1 = self.layer1.binary_search(&x).unwrap_or_else(|s| s);
        let idx0 = self.layer0[idx1].binary_search(&x).unwrap_or_else(|s| s);
        if idx0 == self.layer0[idx1].len() {
            if self.layer1[idx1] == x {
                let mut vec1 = self.layer0.remove(idx1 + 1);
                self.layer0[idx1].append(&mut vec1);
                self.layer1.remove(idx1);
                Ok(())
            } else {
                Err(())
            }
        } else if self.layer0[idx1][idx0] == x {
            self.layer0[idx1].remove(idx0);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn search_higher_equal(&self, x: i64) -> Option<i64> {
        let idx1 = self.layer1.binary_search(&x);
        if idx1.is_ok() {
            return Some(x);
        }
        let idx1 = idx1.unwrap_err();
        if idx1 == self.layer1.len() {
            return None;
        }
        let idx0 = self.layer0[idx1].binary_search(&x);
        if idx0.is_ok() {
            return Some(x);
        }
        let idx0 = idx0.unwrap_err();
        if idx0 == self.layer0[idx1].len() {
            if idx1 == self.layer1.len() - 1 {
                return None;
            }
            Some(self.layer1[idx1])
        } else {
            Some(self.layer0[idx1][idx0])
        }
    }

    pub fn search_lower_equal(&self, x: i64) -> Option<i64> {
        let idx1 = self.layer1.binary_search(&x);
        if idx1.is_ok() {
            return Some(x);
        }
        let idx1 = idx1.unwrap_err();
        let idx0 = self.layer0[idx1].binary_search(&x);
        if idx0.is_ok() {
            return Some(x);
        }
        let idx0 = idx0.unwrap_err();
        if idx0 == 0 {
            if idx1 == 0 {
                None
            } else {
                Some(self.layer1[idx1 - 1])
            }
        } else {
            Some(self.layer0[idx1][idx0 - 1])
        }
    }

    pub fn pop(&mut self, idx: usize) -> i64 {
        let mut s = 0;
        let mut li = self.layer1.len();
        for (i, e) in self.layer0.iter().enumerate() {
            s += e.len() + 1;
            if s > idx {
                li = i;
                break;
            }
        }
        if s == idx + 1 {
            let mut vec1 = self.layer0.remove(li + 1);
            self.layer0[li].append(&mut vec1);
            self.layer1.remove(li)
        } else {
            let i = idx + self.layer0[li].len() - s;
            self.layer0[li].remove(i)
        }
    }
}
// ------------ Square Skip List end ------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tset_ssl() {
        let mut sl = SquareSkipList::new();
        for i in 0..100_000 {
            sl.push(i * 3);
        }
        assert_eq!(sl.pop(10001), 30000);
        assert_eq!(sl.search_higher_equal(13), Some(15));
        assert_eq!(sl.search_higher_equal(14), Some(15));
        assert_eq!(sl.search_higher_equal(15), Some(15));
        assert_eq!(sl.search_lower_equal(15), Some(15));
        assert_eq!(sl.search_lower_equal(16), Some(15));
        assert_eq!(sl.search_lower_equal(17), Some(15));
        assert_eq!(sl.remove(2000), Err(()));
        assert_eq!(sl.remove(2001), Ok(()));
        assert_eq!(sl.search_higher_equal(1999), Some(2004));
    }

    #[test]
    fn tset_minimal_ssl() {
        let mut sl = SquareSkipList::new();
        sl.push(1);
        sl.push(2);
        println!("{:?}", &sl.layer0);
        println!("{:?}", &sl.layer1);
        assert_eq!(sl.search_higher_equal(6), None);
        assert_eq!(sl.search_lower_equal(0), None);
        assert_eq!(sl.search_higher_equal(1), Some(1));
        assert_eq!(sl.search_lower_equal(1), Some(1));
        assert!(sl.remove(1).is_ok());
        assert_eq!(sl.search_higher_equal(1), Some(2));
        assert_eq!(sl.search_lower_equal(1), None);
    }
}
