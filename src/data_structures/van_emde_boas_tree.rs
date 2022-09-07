/// The van Emde Boas tree itself.
#[derive(Debug, Clone)]
pub struct VEBTree {
    children: Vec<Option<VEBTree>>,
    summary: Option<Box<VEBTree>>,
    min: i64,
    max: i64,
    rem: i64,
    quo: i64,
    data: u64,
    length: i64,
    threshold: i64,
}

impl VEBTree {
    #[inline]
    fn lsb(val: u64) -> i64 {
        val.trailing_zeros() as i64
    }

    #[inline]
    fn msb(val: u64) -> i64 {
        63 - val.leading_zeros() as i64
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.min > self.max
    }

    #[inline]
    fn is_one(&self) -> bool {
        self.min == self.max
    }

    #[inline]
    fn index(&self, val: i64) -> i64 {
        if val >= self.rem * (self.quo + 1) {
            self.rem + (val - self.rem * (self.quo + 1)) / self.quo
        } else {
            val / (self.quo + 1)
        }
    }

    #[inline]
    fn sum(&self, idx: i64) -> i64 {
        let idx = idx as i64;
        if idx > self.rem {
            self.rem * (self.quo + 1) + (idx - self.rem) * self.quo
        } else {
            idx * (self.quo + 1)
        }
    }

    fn small_find(&self, val: i64) -> bool {
        self.data >> val & 1 == 1
    }

    fn small_insert(&mut self, val: i64) -> bool {
        !self.small_find(val) && {
            self.data ^= 1 << val;
            true
        }
    }

    fn small_erase(&mut self, val: i64) {
        if val == self.min {
            self.min = Self::lsb(self.data);
            self.data ^= 1 << self.min;
        } else {
            self.data ^= 1 << val;
            if self.data == 0 {
                self.max = self.min;
            } else if val == self.max {
                self.max = Self::msb(self.data);
            }
        }
    }

    fn small_predecessor(&self, val: i64) -> i64 {
        let tmp = self.data & ((1 << val) - 1);
        if tmp != 0 {
            Self::msb(tmp)
        } else {
            self.min
        }
    }

    fn small_successor(&self, val: i64) -> i64 {
        Self::lsb(self.data & !((1 << (val + 1)) - 1))
    }

    pub fn find(&self, val: i64) -> bool {
        if val == self.min {
            true
        } else if self.length <= self.threshold {
            self.small_find(val)
        } else {
            let idx = self.index(val);
            let child = self.children[idx as usize].as_ref().unwrap();
            child.find(val - self.sum(idx))
        }
    }

    pub fn insert(&mut self, mut val: i64) -> bool {
        if self.is_empty() {
            self.min = val;
            self.max = val;
            return true;
        } else if val == self.min {
            return false;
        } else if val < self.min {
            std::mem::swap(&mut val, &mut self.min);
        } else if val > self.max {
            self.max = val;
        }
        if self.length <= self.threshold {
            return self.small_insert(val);
        }
        let idx = self.index(val);
        let child_val = val - self.sum(idx);
        let child = self.children[idx as usize].as_mut().unwrap();
        if child.insert(child_val) {
            self.summary.as_mut().unwrap().insert(idx)
        } else {
            false
        }
    }

    pub fn erase(&mut self, mut val: i64) {
        if self.is_one() {
            self.max = -1;
            self.min = self.length;
        }
        if self.length <= self.threshold {
            return self.small_erase(val);
        }
        if val == self.min {
            let idx = self.summary.as_mut().unwrap().min;
            val = self.sum(idx) + self.children[idx as usize].as_mut().unwrap().min;
            self.min = val;
        }
        let idx = self.index(val);
        let child_val = val - self.sum(idx);
        let child = self.children[idx as usize].as_mut().unwrap();
        child.erase(child_val);
        if child.is_empty() {
            self.summary.as_mut().unwrap().erase(idx);
        }
    }

    pub fn predecessor(&self, val: i64) -> Option<i64> {
        if self.min >= val {
            return None;
        } else if val > self.max {
            return Some(self.max);
        }
        if self.length <= self.threshold {
            return Some(self.small_predecessor(val));
        }
        let idx = self.index(val);
        let sm = self.sum(idx);
        let child = self.children[idx as usize].as_ref().unwrap();
        if val > sm + child.min {
            Some(sm + child.predecessor(val - sm).unwrap())
        } else {
            self.summary
                .as_ref()
                .unwrap()
                .predecessor(idx)
                .map(|idx| self.sum(idx) + self.children[idx as usize].as_ref().unwrap().max)
        }
    }

    pub fn successor(&self, val: i64) -> Option<i64> {
        if val < self.min {
            return Some(self.min);
        } else if val > self.max {
            return None;
        }
        if self.length <= self.threshold {
            return Some(self.small_successor(val));
        }
        let idx = self.index(val);
        let sm = self.sum(idx);
        let child = self.children[idx as usize].as_ref().unwrap();
        if val < sm + child.max {
            Some(sm + child.successor(val - sm).unwrap())
        } else {
            self.summary
                .as_ref()
                .unwrap()
                .successor(idx)
                .map(|idx| self.sum(idx) + self.children[idx as usize].as_ref().unwrap().min)
        }
    }
}
