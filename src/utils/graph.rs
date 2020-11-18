use std::ops::{ Add, AddAssign, Sub, Neg };
use crate::utils::algebraic_traits::{ Element, One, Zero };
use std::fmt::Display;

pub trait Cost:
    Element
    + Display
    + Copy
    + Eq
    + Ord
    + Zero
    + One
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + Neg<Output = Self>
{
    const MAX: Self;
}

#[derive(Copy, Clone)]
pub struct Edge<C: Cost> {
    // pub from: usize,
    pub to: usize,
    pub cost: C,
    // pub id: usize
}

pub struct UndirectedGraph<C: Cost>(pub Vec<Vec<Edge<C>>>);
pub struct DirectedGraph<C: Cost>{
    pub forward: Vec<Vec<Edge<C>>>,
    pub backward: Vec<Vec<Edge<C>>>
}

pub trait Graph<C: Cost> {
    fn new(size: usize) -> Self;
    fn size(&self) -> usize;
    fn add_edge(&mut self, u: usize, v: usize, cost: C);
    fn edges_from(&self, v: usize) -> std::slice::Iter<Edge<C>>;
}

impl<C: Cost> Graph<C> for UndirectedGraph<C> {
    fn new(size: usize) -> Self {
        Self(vec![Vec::<Edge<C>>::new(); size])
    }

    fn size(&self) -> usize {
        self.0.len()
    }

    fn add_edge(&mut self, u: usize, v: usize, cost: C) {
        self.0[u].push(Edge{ to: v, cost });
        self.0[v].push(Edge{ to: u, cost });
    }

    fn edges_from(&self, v: usize) -> std::slice::Iter<Edge<C>> {
        self.0[v].iter()
    }
}

impl<C: Cost> Graph<C> for DirectedGraph<C> {
    fn new(size: usize) -> Self {
        Self {
            forward: vec![Vec::<Edge<C>>::new(); size],
            backward: vec![Vec::<Edge<C>>::new(); size],
        }
    }

    fn size(&self) -> usize {
        self.forward.len()
    }

    fn add_edge(&mut self, u: usize, v: usize, cost: C) {
        self.forward[u].push(Edge{ to: v, cost });
        self.backward[v].push(Edge{ to: u, cost });
    }

    fn edges_from(&self, v: usize) -> std::slice::Iter<Edge<C>> {
        self.forward[v].iter()
    }
}

impl<C: Cost> DirectedGraph<C> {
    pub fn reverse(&self) -> Self {
        Self {
            forward: self.backward.clone(),
            backward: self.forward.clone(),
        }
    }
}

macro_rules! impl_cost {
    ($($T:ident,)*) => {
        $(
            impl Cost for $T {
                const MAX: Self = std::$T::MAX;
            }
        )*
    };
}

impl_cost! {
    i8, i16, i32, i64, i128, isize,
}