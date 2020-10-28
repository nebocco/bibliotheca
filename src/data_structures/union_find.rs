use crate::utils::algebraic_traits::ComGroup;

/*

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

*/

#[derive(Clone, Debug)]
pub struct UnionFind(Vec<isize>);

impl UnionFind {
    pub fn new(len: usize) -> Self {
        Self(vec![-1; len])
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
        self.0[u] += self.0[v];
        self.0[v] = u as isize;
        Ok(())
    }

    pub fn is_same(&mut self, u: usize, v:usize) -> bool {
        self.find(u) == self.find(v)
    }

    fn _climb(&mut self, i: usize) -> (usize, usize) {
        assert!(i < self.0.len());
        let mut v = i;
        while self.0[v] >= 0 {
            let p = self.0[v] as usize;
            if self.0[p] >= 0 {
                self.0[v] = self.0[p];
                v = self.0[p] as usize;
            } else {
                v = p;
            }
        }
        (v, -self.0[v] as usize)
    }

}


#[derive(Clone, Debug)]
pub struct PotentializedUnionFind<T: ComGroup>{
    data: Vec<isize>,
    ws: Vec<T>
}

impl<T: ComGroup> PotentializedUnionFind<T> {
    pub fn new(len: usize) -> Self {
        Self{
            data: vec![-1; len],
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
        self.data[u] += self.data[v];
        self.data[v] = u as isize;
        self.ws[v] = w;
        Ok(())
    }

    pub fn is_same(&mut self, u: usize, v:usize) -> bool {
        self.find(u) == self.find(v)
    }

    pub fn diff(&mut self, u: usize, v: usize) -> Result<T, ()> {
        let (u, _, wu) = self._climb(u);
        let (v, _, wv) = self._climb(v);
        if u == v {
            Ok(wu + -wv)
        } else {
            Err(())
        }
    }

    pub fn weight(&mut self, u: usize, w: T) {
        let p = self.find(u);
        self.ws[p] += w;
    }

    /// _climb(index) -> (root index, group size, potential)
    fn _climb(&mut self, i: usize) -> (usize, usize, T) {
        assert!(i < self.data.len());
        let mut v = i;
        let mut w = T::zero();
        while self.data[v] >= 0 {
            let p = self.data[v] as usize;
            if self.data[p] >= 0 {
                self.data[v] = self.data[p];
                self.ws[v] = self.ws[v].clone() + self.ws[p].clone();
                w += self.ws[v].clone();
                v = self.data[p] as usize;
            } else {
                w += self.ws[v].clone();
                v = p;
            }
        }
        w += self.ws[v].clone();
        (v, -self.data[v] as usize, w)
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

    // TODO: add test
}
