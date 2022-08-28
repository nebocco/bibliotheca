use crate::data_structures::interval_heap::DoublePriorityHeap;

pub struct SlopeTrick {
    lo: DoublePriorityHeap<i64>,
    hi: DoublePriorityHeap<i64>,
    min: i64,
}

impl SlopeTrick {
    pub fn new() -> Self {
        Self {
            lo: DoublePriorityHeap::new(),
            hi: DoublePriorityHeap::new(),
            min: 0,
        }
    }

    pub fn get_min(&self) -> (Option<i64>, Option<i64>, i64) {
        (
            self.lo.get_max().cloned(),
            self.hi.get_min().cloned(),
            self.min,
        )
    }

    pub fn add_const(&mut self, val: i64) {
        self.min += val;
    }

    // add max(x - val, 0)
    pub fn add_plus(&mut self, val: i64) {
        if *self.lo.get_max().unwrap_or(&val) <= val {
            self.hi.push(val);
        } else {
            let lo0 = self.lo.pop_max().unwrap();
            self.min += lo0 - val;
            self.lo.push(val);
            self.hi.push(lo0)
        }
    }

    // add max(val - x, 0)
    pub fn add_minus(&mut self, val: i64) {
        if *self.hi.get_min().unwrap_or(&val) >= val {
            self.lo.push(val);
        } else {
            let hi0 = self.hi.pop_min().unwrap();
            self.min += val - hi0;
            self.hi.push(val);
            self.lo.push(hi0);
        }
    }

    pub fn add_abs(&mut self, val: i64) {
        self.add_plus(val);
        self.add_minus(val);
    }
}
