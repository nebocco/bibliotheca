pub mod chromatic_number;
pub mod cycle;
pub mod dijkstra;
pub mod is_biparate;
pub mod kruskal;
pub mod scc;
pub mod tree_dfs;
pub mod two_edge_connected;

use std::ops::{Add, AddAssign, Neg, Sub};

// ------------ Graph impl start ------------

pub trait Cost:
    Clone
    + Copy
    + std::fmt::Display
    + Eq
    + Ord
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + Neg<Output = Self>
{
    const ZERO: Self;
    const MAX: Self;
}

#[derive(Copy, Clone)]
pub struct Edge<C = Void> {
    // pub from: usize,
    pub to: usize,
    pub cost: C,
    pub id: usize,
}

pub struct UndirectedGraph<C>(pub Vec<Vec<Edge<C>>>, pub usize);
pub struct DirectedGraph<C> {
    pub forward: Vec<Vec<Edge<C>>>,
    pub backward: Vec<Vec<Edge<C>>>,
    pub count: usize,
}

pub trait Graph<C> {
    fn new(size: usize) -> Self;
    fn size(&self) -> usize;
    fn add_edge(&mut self, u: usize, v: usize, cost: C);
    fn edges_from(&self, v: usize) -> std::slice::Iter<Edge<C>>;
}

impl<C: Clone> Graph<C> for UndirectedGraph<C> {
    fn new(size: usize) -> Self {
        Self(vec![Vec::<Edge<C>>::new(); size], 0)
    }

    fn size(&self) -> usize {
        self.0.len()
    }

    fn add_edge(&mut self, u: usize, v: usize, cost: C) {
        self.0[u].push(Edge {
            to: v,
            cost: cost.clone(),
            id: self.1,
        });
        self.0[v].push(Edge {
            to: u,
            cost,
            id: self.1,
        });
        self.1 += 1;
    }

    fn edges_from(&self, v: usize) -> std::slice::Iter<Edge<C>> {
        self.0[v].iter()
    }
}

impl<C: Clone> Graph<C> for DirectedGraph<C> {
    fn new(size: usize) -> Self {
        Self {
            forward: vec![Vec::<Edge<C>>::new(); size],
            backward: vec![Vec::<Edge<C>>::new(); size],
            count: 0,
        }
    }

    fn size(&self) -> usize {
        self.forward.len()
    }

    fn add_edge(&mut self, u: usize, v: usize, cost: C) {
        self.forward[u].push(Edge {
            to: v,
            cost: cost.clone(),
            id: self.count,
        });
        self.backward[v].push(Edge {
            to: u,
            cost,
            id: self.count,
        });
        self.count += 1;
    }

    fn edges_from(&self, v: usize) -> std::slice::Iter<Edge<C>> {
        self.forward[v].iter()
    }
}

impl<C: Clone> DirectedGraph<C> {
    pub fn edges_to(&self, u: usize) -> std::slice::Iter<Edge<C>> {
        self.backward[u].iter()
    }

    pub fn reverse(&self) -> Self {
        Self {
            forward: self.backward.clone(),
            backward: self.forward.clone(),
            count: self.count,
        }
    }
}

macro_rules! impl_cost {
    ($($T:ident,)*) => {
        $(
            impl Cost for $T {
                const ZERO: Self = 0;
                const MAX: Self = std::$T::MAX;
            }
        )*
    };
}

impl_cost! {
    i8, i16, i32, i64, i128, isize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Void;

impl std::fmt::Display for Void {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Void")
    }
}

impl Add for Void {
    type Output = Self;
    fn add(self, _: Self) -> Self {
        self
    }
}

impl AddAssign for Void {
    fn add_assign(&mut self, _: Self) {}
}

impl Sub for Void {
    type Output = Self;
    fn sub(self, _: Self) -> Self {
        self
    }
}

impl Neg for Void {
    type Output = Self;
    fn neg(self) -> Self {
        self
    }
}

impl Cost for Void {
    const ZERO: Self = Void;
    const MAX: Self = Void;
}

// ------------ Graph impl end ------------
