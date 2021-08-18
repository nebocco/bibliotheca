// * verified: https://judge.yosupo.jp/submission/30795
// ------------ Binary Trie start ------------
pub struct BinaryTrie {
    cnt: usize,
    lch: Option<Box<BinaryTrie>>,
    rch: Option<Box<BinaryTrie>>,
}

impl BinaryTrie {
    pub fn new() -> Self {
        Self {
            cnt: 0,
            lch: None,
            rch: None,
        }
    }

    pub fn size(&self) -> usize {
        self.cnt
    }

    pub fn contains(&mut self, mut val: u32) -> bool {
        let mut node = self;
        val = val.reverse_bits();
        for _ in 0..32 {
            node = if val & 1 == 0 {
                if node.lch.is_none() {
                    return false;
                }
                node.lch.as_deref_mut().unwrap()
            } else {
                if node.rch.is_none() {
                    return false;
                }
                node.rch.as_deref_mut().unwrap()
            };
            val >>= 1;
        }
        true
    }

    pub fn insert(&mut self, mut val: u32) {
        if self.contains(val) {
            return;
        }
        self.cnt += 1;
        let mut node = self;
        val = val.reverse_bits();
        for _ in 0..32 {
            node = if val & 1 == 0 {
                if node.lch.is_none() {
                    node.lch = Some(Box::new(Self::new()));
                }
                node.lch.as_deref_mut().unwrap()
            } else {
                if node.rch.is_none() {
                    node.rch = Some(Box::new(Self::new()));
                }
                node.rch.as_deref_mut().unwrap()
            };
            node.cnt += 1;
            val >>= 1;
        }
    }

    pub fn erase(&mut self, mut val: u32) {
        if !self.contains(val) {
            return;
        }
        self.cnt -= 1;
        let mut node = self;
        val = val.reverse_bits();
        for _ in 0..32 {
            node = if val & 1 == 0 {
                assert!(node.lch.is_some());
                if node.lch.as_ref().unwrap().cnt == 1 {
                    node.lch = None;
                    return;
                }
                node.lch.as_deref_mut().unwrap()
            } else {
                assert!(node.rch.is_some());
                if node.rch.as_ref().unwrap().cnt == 1 {
                    node.rch = None;
                    return;
                }
                node.rch.as_deref_mut().unwrap()
            };
            node.cnt -= 1;
            val >>= 1;
        }
    }

    pub fn xor_min(&mut self, mut val: u32) -> u32 {
        let mut node = self;
        val = val.reverse_bits();
        let mut res = 0;
        for _ in 0..32 {
            res <<= 1;
            node = if val & 1 == 0 && node.lch.is_some() || node.rch.is_none() {
                node.lch.as_deref_mut().unwrap()
            } else {
                res |= 1;
                node.rch.as_deref_mut().unwrap()
            };
            val >>= 1;
        }
        res
    }

    pub fn max(&mut self) -> u32 {
        self.xor_min(!0)
    }

    pub fn min(&mut self) -> u32 {
        self.xor_min(0)
    }

    pub fn count_lower_than(&mut self, mut val: u32) -> usize {
        let mut node = self;
        val = val.reverse_bits();
        let mut res = 0;
        for _ in 0..32 {
            if val & 1 == 1 && node.lch.is_some() {
                res += node.lch.as_deref().unwrap().cnt;
            }
            node = if val & 1 == 0 {
                if node.lch.is_none() {
                    return res;
                }
                node.lch.as_deref_mut().unwrap()
            } else {
                if node.rch.is_none() {
                    return res;
                }
                node.rch.as_deref_mut().unwrap()
            };
            val >>= 1;
        }
        res
    }
}

// ------------ Binary Trie end ------------

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_binary_trie() {
        let mut trie = BinaryTrie::new();
        trie.insert(1);
        trie.insert(10);
        trie.insert(100);
        assert_eq!(trie.size(), 3);
        assert!(trie.contains(10));
        trie.insert(1000);
        trie.insert(10000);
        trie.erase(10);
        assert_eq!(trie.min(), 1);
        assert_eq!(trie.max(), 10000);
        assert_eq!(trie.xor_min(96), 100);
        assert_eq!(trie.xor_min(46), 1);
        trie.insert(100000);
        trie.insert(1000000);
        trie.insert(10000000);
        trie.erase(1);
        assert!(trie.contains(100000));
        assert!(!trie.contains(10));
        assert_eq!(trie.count_lower_than(5000), 2);
        assert_eq!(trie.count_lower_than(1000000), 4);
    }
}
