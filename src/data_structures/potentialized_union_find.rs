use crate::utils::algebraic_traits::ComGroup;

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
    // TODO: make tests
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}