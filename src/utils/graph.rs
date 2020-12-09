use std::ops::{ Add, AddAssign, Sub, Neg };
use crate::utils::algebraic_traits::{ Element, One, Zero };

// ------------ Graph impl start ------------

pub trait Cost:
    Element
    + Clone + Copy + std::fmt::Display
    + Eq + Ord
    + Zero + One
    + Add<Output = Self> + AddAssign
    + Sub<Output = Self>
    + Neg<Output = Self>
{
    const MAX: Self;
}

#[derive(Copy, Clone)]
pub struct Edge<C = Void> {
    // pub from: usize,
    pub to: usize,
    pub cost: C,
    pub id: usize
}

pub struct UndirectedGraph<C>(pub Vec<Vec<Edge<C>>>, pub usize);
pub struct DirectedGraph<C>{
    pub forward: Vec<Vec<Edge<C>>>,
    pub backward: Vec<Vec<Edge<C>>>,
    pub count: usize,
}

pub trait Graph<C: Element> {
    fn new(size: usize) -> Self;
    fn size(&self) -> usize;
    fn add_edge(&mut self, u: usize, v: usize, cost: C);
    fn edges_from(&self, v: usize) -> std::slice::Iter<Edge<C>>;
}

impl<C: Element> Graph<C> for UndirectedGraph<C> {
    fn new(size: usize) -> Self {
        Self(vec![Vec::<Edge<C>>::new(); size], 0)
    }

    fn size(&self) -> usize {
        self.0.len()
    }

    fn add_edge(&mut self, u: usize, v: usize, cost: C) {
        self.0[u].push(Edge{ to: v, cost: cost.clone(), id: self.1 });
        self.0[v].push(Edge{ to: u, cost: cost.clone(), id: self.1 });
        self.1 += 1;
    }

    fn edges_from(&self, v: usize) -> std::slice::Iter<Edge<C>> {
        self.0[v].iter()
    }
}

impl<C: Element> Graph<C> for DirectedGraph<C> {
    fn new(size: usize) -> Self {
        Self {
            forward: vec![Vec::<Edge<C>>::new(); size],
            backward: vec![Vec::<Edge<C>>::new(); size],
            count: 0
        }
    }

    fn size(&self) -> usize {
        self.forward.len()
    }

    fn add_edge(&mut self, u: usize, v: usize, cost: C) {
        self.forward[u].push(Edge{ to: v, cost: cost.clone(), id: self.count });
        self.backward[v].push(Edge{ to: u, cost: cost.clone(), id: self.count });
        self.count += 1;
    }

    fn edges_from(&self, v: usize) -> std::slice::Iter<Edge<C>> {
        self.forward[v].iter()
    }
}

impl<C: Element> DirectedGraph<C> {
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
            impl Cost for $T { const MAX: Self = std::$T::MAX; }
        )*
    };
}

impl_cost! {
    i8, i16, i32, i64, i128, isize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Void();

impl std::fmt::Display for Void {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Zero for Void {
    fn zero() -> Self { Void() }
    fn is_zero(&self) -> bool { true }
}

impl One for Void {
    fn one() -> Self { Void() }
    fn is_one(&self) -> bool { true }
}

impl Add for Void {
    type Output = Self;
    fn add(self, _: Self) -> Self { Void() }
}

impl AddAssign for Void {
    fn add_assign(&mut self, _: Self) {}
}

impl Sub for Void {
    type Output = Self;
    fn sub(self, _: Self) -> Self { Void() }
}

impl Neg for Void {
    type Output = Self;
    fn neg(self) -> Self { Void() }
}

impl Cost for Void { const MAX: Self = Void(); }

// ------------ Graph impl end ------------