use std::fmt::Debug;

use crate::bit::Bit;
use crate::bit_buffer::BitBuffer;
use crate::error::BitBufferResult;

/// Defines methods to make reading fields from a buffer easier by providing methods for
/// reading bits, u8s, u32s, etc.
pub trait ReadableBuf: Debug {
    /// Return how many bytes are remaining in this buffer.  Note that this
    /// does not take into account a partially read byte (which
    /// is considered as a 'whole' byte)
    fn bytes_remaining(&self) -> usize;

    /// Consume the next bit and return it as a bool
    fn read_bit_as_bool(&mut self) -> BitBufferResult<bool> {
        self.read_bit().map(|b| b.into())
    }

    fn read_bit(&mut self) -> BitBufferResult<Bit>;

    fn read_bits_as_u8(&mut self, num_bits: usize) -> BitBufferResult<u8>;

    // Peek at the next byte without advancing the position
    fn peek_u8(&self) -> BitBufferResult<u8>;

    /// Consume and return the next byte as a u8
    fn read_u8(&mut self) -> BitBufferResult<u8>;

    /// Consume and return the next 2 bytes as a u16
    fn read_u16(&mut self) -> BitBufferResult<u16> {
        let mut value: u16 = 0;
        for _ in 0..2 {
            value <<= 8;
            value |= self.read_u8()? as u16;
        }

        Ok(value)
    }

    /// Consume and return the next 3 bytes as a u32
    fn read_u24(&mut self) -> BitBufferResult<u32> {
        let mut value: u32 = 0;
        for _ in 0..3 {
            value <<= 8;
            value |= self.read_u8()? as u32;
        }

        Ok(value)
    }

    /// Consume and return the next 4 bytes as a u32
    fn read_u32(&mut self) -> BitBufferResult<u32> {
        let mut value: u32 = 0;
        for _ in 0..4 {
            value <<= 8;
            value |= self.read_u8()? as u32;
        }

        Ok(value)
    }

    fn read_bytes(&mut self, num_bytes: usize) -> BitBufferResult<&[u8]>;

    /// Create a 'sub buffer' which starts at this ReadableBuf's current position
    /// and contains the next |length| bytes.
    /// TODO: it's not clear to me whether or not grabbing a sub-buffer should ALSO advance
    ///  the position of the parent buffer by the size of the sub-buffer.  I think I'll have
    ///  to see how it feels when using it and see which makes more sense.
    fn sub_buffer<'a, 'b>(&'a mut self, length: usize) -> BitBufferResult<BitBuffer<&'b [u8]>>
    where
        'a: 'b;
}
