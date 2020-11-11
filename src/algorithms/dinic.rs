#![allow(dead_code)]
use num_traits::{Num, Unsigned};

pub trait FlowTrait: Num + Unsigned {}
impl<T: Num + Unsigned> FlowTrait for T {}

#[derive(Debug, Clone)]
struct FlowEdge<Value: FlowTrait> {
	from: usize,
	to: usize,
	cap: Value
}

#[derive(Debug, Clone)]
struct DoubleEdge {
    from: usize,
    from_idx: usize,
    to: usize,
    to_idx: usize,
}

/// Dinic 法で最大流を求めます。
pub struct MaxFlow<Value: FlowTrait> {
    /// 最大流のアルゴリズムを適用済みかどうかです。
    source: usize,
    sink: usize,
    network: Vec<Vec<FlowEdge<Value>>>,
}

impl<Value: FlowTrait> MaxFlow<Value> {
    fn len(&self) -> usize {
        self.network.len()
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
