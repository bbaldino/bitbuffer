use crate::bit::Bit;
use crate::byte_buffer_slice::ByteBufferSlice;
use crate::error::CursorResult;
use crate::helpers::{read_byte, take_bit_as};
use crate::readable_buf::{ReadableBuf, ReadableBufExtra};
use crate::some_readable_buf::SomeReadableBuf;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::{AddAssign, Div, Rem};

pub struct ByteBuffer<'a> {
    buf: Vec<u8>,
    bit_offset: RefCell<usize>,
    marker: PhantomData<&'a ()>,
}

/// Ctors
impl<'a> ByteBuffer<'a> {
    pub fn from_vec(buf: Vec<u8>) -> ByteBuffer<'a> {
        ByteBuffer {
            buf,
            bit_offset: RefCell::new(0),
            marker: PhantomData,
        }
    }
}

/// Private
impl ByteBuffer<'_> {
    /// Return the current byte offset into the buffer
    fn byte_offset(&self) -> usize {
        self.bit_offset.borrow().div(8)
    }

    /// Return the current bit position within the current byte
    fn bit_position(&self) -> usize {
        self.bit_offset.borrow().rem(8)
    }

    /// Move the current position forward by |num_bits| bits
    fn advance_bits(&self, num_bits: usize) {
        self.bit_offset.borrow_mut().add_assign(num_bits);
    }

    /// Move the current position forward by |num_bytes| bytes
    fn advance_bytes(&self, num_bytes: usize) {
        self.bit_offset.borrow_mut().add_assign(num_bytes * 8);
    }
}

/// Public
impl ByteBuffer<'_> {
    /// Return how many bytes are remaining in this buffer.  Note that this
    /// does not take into account a partially consumed/written byte (which
    /// is considered as a 'whole' byte)
    pub fn bytes_remaining(&self) -> usize {
        self.buf.len() - self.byte_offset()
    }
}

impl ReadableBuf for ByteBuffer<'_> {
    fn bytes_remaining(&self) -> usize {
        self.bytes_remaining()
    }

    fn read_bit_as_bool(&self) -> CursorResult<bool> {
        take_bit_as::<Bit>(&self.buf, self.byte_offset(), self.bit_position()).map(|b| {
            self.advance_bits(1);
            b.into()
        })
    }

    fn read_bit(&self) -> CursorResult<Bit> {
        take_bit_as::<Bit>(&self.buf, self.byte_offset(), self.bit_position()).map(|b| {
            self.advance_bits(1);
            b
        })
    }

    fn read_u8(&self) -> CursorResult<u8> {
        let byte = read_byte(&self.buf, self.byte_offset())?;
        self.advance_bytes(1);
        Ok(byte)
    }

    fn sub_buffer<'a>(&'a self, length: usize) -> CursorResult<SomeReadableBuf<'a>> {
        let b = ByteBufferSlice {
            buf: &(self.buf[self.byte_offset()..][..length]),
            bit_offset: RefCell::new(0),
        };
        Ok(SomeReadableBuf::ByteBufferSlice(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_remaining() {
        let data: Vec<u8> = vec![1, 2, 3];
        let bb = ByteBuffer::from_vec(data);

        assert_eq!(bb.bytes_remaining(), 3);
        let _ = bb.read_bit();
        assert_eq!(bb.bytes_remaining(), 3);
        let _ = (&bb as &dyn ReadableBuf).read_bits_as::<u8>(7);
        assert_eq!(bb.bytes_remaining(), 2);

        let _ = bb.read_u8();
        assert_eq!(bb.bytes_remaining(), 1);
    }

    #[test]
    fn test_read_bit_as_bool() {
        let data: Vec<u8> = vec![0b11110000];
        let bb = ByteBuffer::from_vec(data);

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
        let bb = ByteBuffer::from_vec(data);

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
        let bb = ByteBuffer::from_vec(data);

        assert_eq!(bb.read_u8().unwrap(), 1u8);
        assert_eq!(bb.read_u8().unwrap(), 2u8);
        assert_eq!(bb.read_u8().unwrap(), 3u8);
    }

    #[test]
    fn test_sub_buffer() {
        let data: Vec<u8> = vec![1, 2, 3];
        let bb = ByteBuffer::from_vec(data);

        let sb = bb.sub_buffer(2).unwrap();
        assert_eq!(sb.read_u8().unwrap(), 1u8);
        assert_eq!(sb.read_u8().unwrap(), 2u8);
    }
}
