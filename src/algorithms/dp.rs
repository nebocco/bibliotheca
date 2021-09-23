use crate::utils::algebraic_traits::Element;

// pub mod knuth_yao;
// pub mod monotone_minima;
pub mod smawk;

pub trait Monge<T: Element>: Matrix<T> {}

pub trait TotallyMonotone<T: Element> {}
impl<T: Element, M: Monge<T>> TotallyMonotone<T> for M {}

pub trait Monotone<T: Element> {}
impl<T: Element, M: TotallyMonotone<T>> Monotone<T> for M {}

pub trait Matrix<T: Element> {
    fn size(&self) -> (usize, usize);
    fn index(&self, row: usize, col: usize) -> T;
}

impl<T: Element> Matrix<T> for Vec<Vec<T>> {
    fn size(&self) -> (usize, usize) {
        (self.len(), self[0].len())
    }

    fn index(&self, row: usize, col: usize) -> T {
        self[row][col].clone()
    }
}
