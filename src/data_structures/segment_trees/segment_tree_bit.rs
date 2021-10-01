use crate::data_structures::fenwick_tree::{FenwickTree, Monoid};

// https://ei1333.github.io/algorithm/segment-tree.html
pub struct SegmentTreeBIT<G: Monoid> {
    size: usize,
    seg: Vec<Box<FenwickTree<G>>>,
    beet: Vec<Vec<usize>>,
}

impl<G: Monoid> SegmentTreeBIT<G> {
    pub fn new(size: usize) -> Self {
        let size = size.next_power_of_two();
        let seg = Vec::new();
        let beet = vec![Vec::new(); size << 1];
        Self { size, seg, beet }
    }

    pub fn update(&mut self, mut x: usize, y: usize, z: G::Val) {
        x += self.size;
        while x > 0 {
            self.seg[x].add(self.beet[x].binary_search(&y).unwrap(), z.clone());
            x >>= 1;
        }
    }

    pub fn build(&mut self) {
        let mut seg = Vec::with_capacity(self.size << 1);
        self.beet.iter_mut().for_each(|lis| {
            lis.sort();
            lis.dedup();
            seg.push(Box::new(FenwickTree::new(lis.len())))
        });
        self.seg = seg;
    }

    // fn _query(&mut self, a: usize, b: usize, x: usize, y: usize, k: usize, l: usize, r: usize) {
    //     unimplemented!();
    // }
}
