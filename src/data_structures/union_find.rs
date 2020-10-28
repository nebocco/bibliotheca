use crate::utils::algebraic_traits::ComGroup;

#[derive(Clone, Copy, Debug)]
enum PatentOrSize {
    Parent(usize),
    Size(usize),
}

#[derive(Clone, Debug)]
pub struct UnionFind(Vec<PatentOrSize>);

impl UnionFind {
    pub fn new(len: usize) -> Self {
        Self(vec![PatentOrSize::Size(1); len])
    }

    pub fn find(&mut self, i: usize) -> usize {
        self._climb(i).0
    }

    pub fn size(&mut self, i: usize) -> usize {
        self._climb(i).1
    }

    pub fn unite(&mut self, u: usize, v: usize) -> Result<(), ()> {
        let (mut u, su) = self._climb(u);
        let (mut v, sv) = self._climb(v);
        if u == v { return Err(()); }
        if su < sv {
            std::mem::swap(&mut u, &mut v);
        }
        self.0[u] = PatentOrSize::Size(su + sv);
        self.0[v] = PatentOrSize::Parent(u);
        Ok(())
    }

    pub fn is_same(&mut self, u: usize, v:usize) -> bool {
        self.find(u) == self.find(v)
    }

    fn _climb(&mut self, i: usize) -> (usize, usize) {
        assert!(i < self.0.len());
        match self.0[i] {
            PatentOrSize::Parent(p) => {
                let ret = self._climb(p);
                self.0[i] = PatentOrSize::Parent(ret.0);
                ret
            }
            PatentOrSize::Size(s) => (i, s)
        }
    }
}

#[derive(Clone, Debug)]
pub struct WeighedUnionFind<T: ComGroup>{
    data: Vec<PatentOrSize>,
    ws: Vec<T>
}

impl<T: ComGroup> WeighedUnionFind<T> {
    pub fn new(len: usize) -> Self {
        Self{
            data: vec![PatentOrSize::Size(1); len],
            ws: vec![T::zero(); len]
        }
    }

    pub fn find(&mut self, i: usize) -> usize {
        self._climb(i).0
    }

    pub fn size(&mut self, i: usize) -> usize {
        self._climb(i).1
    }

    pub fn potential(&mut self, i: usize) -> T {
        self._climb(i).2
    }

    /// unite(u, v, w) -> u <==w== v
    pub fn unite(&mut self, u: usize, v: usize, mut w: T) -> Result<(), ()> {
        let (mut u, su, wu) = self._climb(u);
        let (mut v, sv, wv) = self._climb(v);
        w += wv + -wu;
        if u == v { return Err(()); }
        if su < sv {
            std::mem::swap(&mut u, &mut v);
            w = -w;
        }
        self.data[u] = PatentOrSize::Size(su + sv);
        self.data[v] = PatentOrSize::Parent(u);
        self.ws[v] = w;
        Ok(())
    }

    pub fn is_same(&mut self, u: usize, v:usize) -> bool {
        self.find(u) == self.find(v)
    }

    /// _climb(index) -> (root index, group size, potential)
    fn _climb(&mut self, mut i: usize) -> (usize, usize, T) {
        let mut w = T::zero();
        while let PatentOrSize::Parent(p) = self.data[i] {
            match self.data[p] {
                PatentOrSize::Parent(pp) => {
                    self.ws[i] = self.ws[i].clone() + self.ws[p].clone();
                    self.data[i] = self.data[p];
                    w += self.ws[i].clone();
                    i = pp;
                },
                PatentOrSize::Size(_) => {
                    w += self.ws[i].clone();
                    i = p;
                }
            }
        }
        if let PatentOrSize::Size(s) = self.data[i] {
            (i, s, w + self.ws[i].clone())
        } else {
            (0, 0, T::zero())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::UnionFind;
    // * verified at https://judge.yosupo.jp/submission/26460
    #[test]
    fn tset_union_find() {
        let mut uf = UnionFind::new(6);
        assert_eq!(uf.size(2), 1);
        assert_eq!(uf.find(5), 5);
        uf.unite(0, 4).ok();
        uf.unite(2, 3).ok();
        uf.unite(1, 4).ok();
        uf.unite(1, 0).ok();
        assert_eq!(uf.find(5), 5);
        assert_eq!(uf.find(2), uf.find(3));
        assert_eq!(uf.size(0), 3);
        assert_ne!(uf.find(5), uf.find(0));
        assert!(uf.is_same(1, 4));
    }
}
