pub struct BinaryTrie {
    cnt: usize,
    lch: Option<Box<BinaryTrie>>,
    rch: Option<Box<BinaryTrie>>
}

impl BinaryTrie {
    pub fn new() -> Self {
        Self {
            cnt: 0,
            lch: None,
            rch: None
        }
    }

    pub fn insert(mut self, mut l: u32) {
        self.cnt += 1;
        l = l.reverse_bits();
        for _ in 0..32 {
            if l & 1 == 0 {
                if self.lch.is_none() {
                    self.lch = Some(Box::new(Self::new()));
                }
                self = *self.lch.unwrap();
            } else {
                if self.rch.is_none() {
                    self.rch = Some(Box::new(Self::new()));
                }
                self = *self.rch.unwrap();
            }
            self.cnt += 1;
            l >>= 1;
        }
    }
}