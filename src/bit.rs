// This helper makes the bit reading/writing functionality cleaner, since the caller can do an
// exhaustive match on the result and we can enforce which values are allowed.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum Bit {
    Zero = 0,
    One = 1,
}

impl From<u8> for Bit {
    fn from(value: u8) -> Self {
        match value {
            0 => Bit::Zero,
            1 => Bit::One,
            _ => panic!("Invalid value {}", value),
        }
    }
}

impl From<Bit> for bool {
    fn from(bit: Bit) -> Self {
        match bit {
            Bit::Zero => false,
            Bit::One => true,
        }
    }
}

impl From<Bit> for u8 {
    fn from(bit: Bit) -> Self {
        bit as u8
    }
}
