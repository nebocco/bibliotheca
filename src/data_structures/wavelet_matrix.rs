use crate::data_structures::bit_vector::BitVector;

pub struct WaveletMatrix {
    _table: BitVector
}

impl WaveletMatrix {
    pub fn new() -> Self {
        WaveletMatrix{ _table: BitVector::new(1) }
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