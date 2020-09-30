pub struct Bitset {
    data: Vec<u8>,
    size: usize
}

impl Bitset {
    pub fn new(size: usize) -> Self {
        let data = vec![0; size / 8 + 1];
        Bitset{ data, size }
    }

    pub fn one_for_all(&mut self) {
        (0..self.size).for_each(|i| self.data[i] = 0xff );
    }

    pub fn access(&self, pos: usize) -> bool {
        (self.data[pos >> 3] >> (pos & 7)) & 1 == 1
    }

    pub fn set(&mut self, pos: usize, v: bool) {
        if v {
            self.data[pos >> 3] |= 1 << (pos & 7);
        } else {
            self.data[pos >> 3] &= !(1 << (pos & 7));
        }
    }

    pub fn flip(&mut self, pos: usize) {
        self.data[pos >> 3] ^= 1 << (pos & 7);
    }

    pub fn collect(&self) -> Vec<u64> {
        let mut ret = Vec::new();
        for i in 0..self.size+1 {
            if self.access(i) {
                ret.push(i as u64);
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
