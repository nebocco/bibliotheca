// * ngtkanaさんのac-adapterでの実装を大いに参考にしています
// * というかほぼそのままです
// * https://github.com/ngtkana/ac-adapter-rs/blob/master/crates/algolib/union_find/src/lib.rs

#[derive(Clone, Copy)]
enum PatentOrSize {
    Parent(usize),
    Size(usize),
}

#[derive(Clone, Debug)]
pub struct UnionFind(Vec<PatentOrSize>);

impl UnionFind {
    pub fn new(N: usize) -> Self {
        Self(vec![PatentOrSize::Size(1); N])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn find(&mut self, i: usize) -> usize {
        self._climb(i).0
    }

    pub fn size(&mut self, i: usize) -> usize {
        self._climb(i).1
    }

    pub fn unite(&mut self, u: usize, v: usize) -> Result<_, _> {
        let (mut u, su) = self._climb(u);
        let (mut v, sv) = self._climb(v);
        if u == v { return Err(); }
        if su < sv {
            std::mem::swap(&mut u, &mut v);
        }
        self.0[u] = PatentOrSize::Size(su + sv);
        self.0[v] = PatentOrSize::Parent(u);
        Ok()
    }

    pub fn same(&mut self, u: usize, v:usize) -> bool {
        self.find(u) == self.find(v)
    }

    fn _climb(&mut self, i: usize) -> (usize, usize) {
        assert!(i < self.len());
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

// TODO: create tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
