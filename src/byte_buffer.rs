use crate::bit::Bit;
use crate::error::BitBufferError::BufferTooShort;
use crate::error::BitBufferResult;
use crate::helpers::{read_bit_as, read_byte, read_bytes};
use crate::readable_buf::ReadableBuf;
use std::ops::{AddAssign, Div, Rem};

#[derive(Clone)]
pub struct ByteBuffer<T> {
    inner: T,
    bit_offset: usize,
}

/// Constructors
impl<T> ByteBuffer<T> {
    pub fn new(inner: T) -> ByteBuffer<T> {
        ByteBuffer {
            inner,
            bit_offset: 0,
        }
    }
}

/// Private
impl<T> ByteBuffer<T> {
    /// Return the current byte offset into the buffer
    fn byte_offset(&self) -> usize {
        self.bit_offset.div(8)
    }

    /// Return the current bit position within the current byte
    fn bit_position(&self) -> usize {
        self.bit_offset.rem(8)
    }

    /// Move the current position forward by |num_bits| bits
    fn advance_bits(&mut self, num_bits: usize) {
        self.bit_offset.add_assign(num_bits);
    }

    /// Move the current position forward by |num_bytes| bytes
    fn advance_bytes(&mut self, num_bytes: usize) {
        self.bit_offset.add_assign(num_bytes * 8);
    }
}

impl<T> ReadableBuf for ByteBuffer<T>
where
    T: AsRef<[u8]>,
{
    fn bytes_remaining(&self) -> usize {
        self.inner.as_ref().len() - self.byte_offset()
    }

    fn read_bit(&mut self) -> BitBufferResult<Bit> {
        read_bit_as::<Bit>(self.inner.as_ref(), self.byte_offset(), self.bit_position()).map(|b| {
            self.advance_bits(1);
            b
        })
    }

    fn peek_u8(&self) -> BitBufferResult<u8> {
        read_byte(self.inner.as_ref(), self.byte_offset())
    }

    fn read_u8(&mut self) -> BitBufferResult<u8> {
        let byte = read_byte(self.inner.as_ref(), self.byte_offset())?;
        self.advance_bytes(1);
        Ok(byte)
    }

    fn read_bytes(&mut self, num_bytes: usize) -> BitBufferResult<&[u8]> {
        self.advance_bytes(num_bytes);
        let bytes = read_bytes(
            self.inner.as_ref(),
            self.byte_offset() - num_bytes,
            num_bytes,
        )?;
        Ok(bytes)
    }

    fn sub_buffer<'a, 'b>(&'a mut self, length: usize) -> BitBufferResult<ByteBuffer<&'b [u8]>>
    where
        'a: 'b,
    {
        if self.byte_offset() + length > self.inner.as_ref().len() {
            Err(BufferTooShort {
                start_pos: self.byte_offset(),
                num_bytes: length,
                buffer_size: self.inner.as_ref().len(),
            })
        } else {
            self.advance_bytes(length);
            let slice = &self.inner.as_ref()[(self.byte_offset() - length)..][..length];
            Ok(ByteBuffer::new(slice))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::readable_buf::ReadableBufExtra;

    #[test]
    fn test_bytes_remaining() {
        let data: Vec<u8> = vec![1, 2, 3];
        let mut bb = ByteBuffer::new(data);

        assert_eq!(bb.bytes_remaining(), 3);
        let _ = bb.read_bit();
        assert_eq!(bb.bytes_remaining(), 3);
        let _ = (&mut bb as &mut dyn ReadableBuf).read_bits_as::<u8>(7);
        assert_eq!(bb.bytes_remaining(), 2);

        let _ = bb.read_u8();
        assert_eq!(bb.bytes_remaining(), 1);
    }

    #[test]
    fn test_read_bit_as_bool() {
        let data: Vec<u8> = vec![0b11110000];
        let mut bb = ByteBuffer::new(data);

        assert_eq!(bb.read_bit_as_bool().unwrap(), true);
        assert_eq!(bb.read_bit_as_bool().unwrap(), true);
        assert_eq!(bb.read_bit_as_bool().unwrap(), true);
        assert_eq!(bb.read_bit_as_bool().unwrap(), true);
        assert_eq!(bb.read_bit_as_bool().unwrap(), false);
        assert_eq!(bb.read_bit_as_bool().unwrap(), false);
        assert_eq!(bb.read_bit_as_bool().unwrap(), false);
        assert_eq!(bb.read_bit_as_bool().unwrap(), false);
    }

    #[test]
    fn test_read_bit() {
        let data: Vec<u8> = vec![0b11110000];
        let mut bb = ByteBuffer::new(data);

        assert_eq!(bb.read_bit().unwrap(), Bit::One);
        assert_eq!(bb.read_bit().unwrap(), Bit::One);
        assert_eq!(bb.read_bit().unwrap(), Bit::One);
        assert_eq!(bb.read_bit().unwrap(), Bit::One);
        assert_eq!(bb.read_bit().unwrap(), Bit::Zero);
        assert_eq!(bb.read_bit().unwrap(), Bit::Zero);
        assert_eq!(bb.read_bit().unwrap(), Bit::Zero);
        assert_eq!(bb.read_bit().unwrap(), Bit::Zero);
    }

    #[test]
    fn test_read_u8() {
        let data: Vec<u8> = vec![1, 2, 3];
        let mut bb = ByteBuffer::new(data);

        assert_eq!(bb.read_u8().unwrap(), 1u8);
        assert_eq!(bb.read_u8().unwrap(), 2u8);
        assert_eq!(bb.read_u8().unwrap(), 3u8);
    }

    #[test]
    fn test_sub_buffer() {
        let data: Vec<u8> = vec![1, 2, 3];
        let mut bb = ByteBuffer::new(data);

        let mut sb = bb.sub_buffer(2).unwrap();
        assert_eq!(sb.read_u8().unwrap(), 1u8);
        assert_eq!(sb.read_u8().unwrap(), 2u8);

        let mut sb = bb.sub_buffer(1).unwrap();
        assert_eq!(sb.read_u8().unwrap(), 3u8);
    }
}
