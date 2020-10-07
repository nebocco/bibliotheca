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
    use super::WaveletMatrix;
    #[test]
    fn it_works() {
        let _wm = WaveletMatrix::new();
        assert_eq!(2 + 2, 4);
    }
}
