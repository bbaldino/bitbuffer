use crate::bit::Bit;
use crate::error::CursorResult;
use std::ops::{BitOrAssign, ShlAssign};

pub trait ReadableBuf<'a> {
    /// Return how many bytes are remaining in this buffer
    fn bytes_remaining(&self) -> usize;

    /// Consume the next bit and return it as a bool
    fn read_bit_as_bool(&self) -> CursorResult<bool>;

    fn read_bit(&self) -> CursorResult<Bit>;

    /// Consume and return the next byte as a u8
    fn read_u8(&self) -> CursorResult<u8>;

    /// Consume and return the next 2 bytes as a u16
    fn read_u16(&self) -> CursorResult<u16> {
        let mut value: u16 = 0;
        for _ in 0..2 {
            value <<= 8;
            value |= self.read_u8()? as u16;
        }

        Ok(value)
    }

    /// Consume and return the next 3 bytes as a u32
    fn read_u24(&self) -> CursorResult<u32> {
        let mut value: u32 = 0;
        for _ in 0..3 {
            value <<= 8;
            value |= self.read_u8()? as u32;
        }

        Ok(value)
    }

    /// Consume and return the next 4 bytes as a u32
    fn read_u32(&self) -> CursorResult<u32> {
        let mut value: u32 = 0;
        for _ in 0..4 {
            value <<= 8;
            value |= self.read_u8()? as u32;
        }

        Ok(value)
    }

    /// Create a 'sub buffer' which starts at this ReadableBuf's current position
    /// and contains the next |length| bytes.
    fn sub_buffer<'b>(&'a self, length: usize) -> CursorResult<Box<dyn ReadableBuf<'b> + 'b>>
    where
        'a: 'b;
}

// In order to make ReadableBuf usable as a trait object (so I can do things like
// return Box<dyn ReadableBuf>), I needed to move the generic methods out of the
// ReadableBuf trait.  I can still achieve the same thing by moving _those_ methods
// into another trait, and then implementing that trait for ReadableBuf.  However,
// if I have a ByteBuffer, it looks like I'm able to call the ReadableBuf methods
// on it directly, but _can't_ call the methods defined in the secondary trait
// directly: I'd need to cast it like below.  I don't think this will be a big
// problem, though, as all the methods will use references to ReadableBuf, and that
// works fine with calling methods defined in the secondary trait.
pub trait ReadableBufExtra {
    /// Consume the next bit and return it as type T
    fn read_bit_as<T: From<u8>>(&self) -> CursorResult<T>;
}

impl ReadableBufExtra for dyn ReadableBuf<'_> {
    fn read_bit_as<T: From<u8>>(&self) -> CursorResult<T> {
        let bit_val: u8 = self.read_bit()?.into();
        Ok(bit_val.into())
    }
}

// Consume the next |num_bits| and return them as type T
// fn read_bits_as<T>(&self, num_bits: usize) -> CursorResult<T>
// where
//     T: From<u8> + Default + ShlAssign<u8> + BitOrAssign
// {
//     let mut value: T = Default::default();
//     for _ in 0..num_bits {
//         value <<= 1u8;
//         value |= self.read_bit_as()?;
//     }
//     Ok(value)
// }