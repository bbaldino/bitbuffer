use std::ops::Add;

struct Offset {
    byte_position: usize,
    bit_position: usize,
}

impl Default for Offset {
    fn default() -> Self {
        Offset {
            byte_position: 0,
            bit_position: 0,
        }
    }
}

impl Offset {
    fn advance_bits(&mut self, num_bits: usize) {}
}
