use crate::utils::algebraic_traits::Group;

// * verified at https://judge.yosupo.jp/submission/26460
// ------------ UnionFind start ------------
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
// ------------ UnionFind end ------------


// TODO: verify
// ------------ Potentialized UnionFind start ------------

#[derive(Clone, Debug)]
pub struct PotentializedUnionFind<T>{
    data: Vec<isize>,
    ws: Vec<T>
}

impl<T: Group> PotentializedUnionFind<T> {
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

    /// potential[v] - potential[u] = w
    /// keep potential[u] unchanged
    pub fn unite(&mut self, u: usize, v: usize, mut w: T) -> Result<(), ()> {
        let (u, su, wu) = self._climb(u);
        let (v, sv, wv) = self._climb(v);
		if u == v {
			return if w == -wu + wv { Ok(()) } else { Err(()) };
		}
        w = -self.ws[u].clone() + wu + w + self.ws[v].clone() + -wv;
		if su < sv {
            self.data[v] += self.data[u];
            self.data[u] = v as isize;
            self.ws[v] = self.ws[u].clone() + w.clone();
            self.ws[u] = -w.clone();
        } else {
            self.data[u] += self.data[v];
            self.data[v] = u as isize;
            self.ws[v] = w.clone();
        }
        Ok(())
    }

    pub fn is_same(&mut self, u: usize, v:usize) -> bool {
        self.find(u) == self.find(v)
    }

    /// potential[v] - potential[u]
    pub fn diff(&mut self, u: usize, v: usize) -> Option<T> {
        let (u, _, wu) = self._climb(u);
        let (v, _, wv) = self._climb(v);
        if u == v {
            Some(-wu + wv)
        } else {
            None
        }
    }

    pub fn weigh(&mut self, u: usize, w: T) {
        let p = self.find(u);
        self.ws[p] = self.ws[p].clone() + w;
    }

    /// _climb(i) -> (root, group size, potential)
    fn _climb(&mut self, i: usize) -> (usize, usize, T) {
        assert!(i < self.data.len());
        let mut v = i;
        let mut w = T::zero();
        while self.data[v] >= 0 {
			w = self.ws[v].clone() + w;
            let p = self.data[v] as usize;
            if self.data[p] >= 0 {
                self.data[v] = self.data[p];
                self.ws[v] = self.ws[p].clone() + self.ws[v].clone();
            }
            v = p;
        }
        w = self.ws[v].clone() + w;
        (v, -self.data[v] as usize, w)
    }
}
// ------------ Potentialized UnionFind end ------------


// TODO: verify
// ------------ Iterative UnionFind start ------------
#[derive(Clone, Debug)]
pub struct IterativeUnionFind(Vec<isize>, Vec<usize>);

impl IterativeUnionFind {
    pub fn new(len: usize) -> Self {
        Self(vec![-1; len], (0..len).collect())
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
        self.1.swap(u, v);
        Ok(())
    }

    pub fn is_same(&mut self, u: usize, v:usize) -> bool {
        self.find(u) == self.find(v)
    }

    pub fn iter_group(&mut self, u: usize) -> Vec<usize> {
        let mut res = Vec::with_capacity(self.size(u));
        res.push(u);
        let mut v = self.1[u];
        while v != u {
            res.push(v);
            v = self.1[v];
        }
        res
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
// ------------ Iterative UnionFind end ------------


#[cfg(test)]
mod tests {
    use super::*;

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


    #[test]
    fn tset_potentialized_union_find() {
        let mut puf = PotentializedUnionFind::<i32>::new(7);
        assert_eq!(puf.size(2), 1);
        assert_eq!(puf.find(5), 5);
        assert_eq!(puf.potential(5), 0);
        puf.weigh(5, 5);
        assert_eq!(puf.potential(5), 5);
        assert!(puf.unite(0, 1, 30).is_ok()); //pot[v] - pot[u] = w
        assert_eq!(puf.potential(0), 0);
        assert_eq!(puf.potential(1), 30);
        assert!(puf.unite(0, 2, -10).is_ok());
        assert_eq!(puf.potential(0), 0);
        assert_eq!(puf.potential(2), -10);
        assert_eq!(puf.diff(2, 1), Some(40)); // pot[v] - pot[u]
        puf.weigh(2, 50);
        assert_eq!(puf.potential(1), 80);
        assert_eq!(puf.potential(2), 40);
        assert_eq!(puf.diff(0, 1), Some(30));
        assert!(puf.unite(1, 2, -40).is_ok());
        assert_eq!(puf.potential(3), 0);
        assert_eq!(puf.potential(5), 5);
        puf.unite(3, 5, 0).ok();
        assert_eq!(puf.potential(3), 0);
        assert_eq!(puf.potential(5), 0);
        puf.unite(4, 5, 80).ok();
        assert_eq!(puf.potential(5), 80);
        puf.unite(5, 6, 10).ok();
        assert_eq!(puf.potential(0), 50);
        assert_eq!(puf.potential(6), 90);
        assert_eq!(puf.diff(0, 1), Some(30));
        puf.unite(6, 0, 20).ok();
        assert_eq!(puf.diff(0, 1), Some(30));
        assert_eq!(puf.potential(0), 110);
        assert_eq!(puf.potential(1), 140);
        assert_eq!(puf.potential(2), 100);
        assert_eq!(puf.potential(3), 80);
        assert_eq!(puf.potential(4), 0);
        assert_eq!(puf.potential(5), 80);
        assert_eq!(puf.potential(6), 90);
    }
}
