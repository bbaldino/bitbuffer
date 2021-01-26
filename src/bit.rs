// This helper makes the bit reading/writing functionality cleaner, since the caller can do an
// exhaustive match on the result and we can enforce which values are allowed.
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

impl Into<bool> for Bit {
    fn into(self) -> bool {
        match self {
            Bit::Zero => false,
            Bit::One => true,
        }
    }
}

impl Into<u8> for Bit {
    fn into(self) -> u8 {
        self as u8
    }
}
