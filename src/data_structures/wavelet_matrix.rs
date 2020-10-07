#![allow(dead_code)]

use crate::data_structures::bit_vector::BitVector;

struct WaveletMatrix {
    table: BitVector
}

impl WaveletMatrix {
    fn new() -> Self {
        WaveletMatrix{ table: BitVector::new(1) }
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