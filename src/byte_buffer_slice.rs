use crate::bit::Bit;
use crate::error::CursorError::BufferTooShort;
use crate::error::CursorResult;
use crate::helpers::{read_bit_as, read_byte, read_bytes};
use crate::readable_buf::ReadableBuf;
use std::cell::RefCell;
use std::ops::{AddAssign, Div, Rem};

pub struct ByteBufferSlice<'a> {
    pub buf: &'a [u8],
    pub bit_offset: RefCell<usize>,
}

/// Constructors
impl ByteBufferSlice<'_> {
    pub fn from_slice(slice: &[u8], start_pos: usize, length: usize) -> ByteBufferSlice<'_> {
        ByteBufferSlice {
            buf: &slice[start_pos..][..length],
            bit_offset: RefCell::new(0),
        }
    }
}

/// Private
impl ByteBufferSlice<'_> {
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
impl ByteBufferSlice<'_> {}

impl ReadableBuf for ByteBufferSlice<'_> {
    fn bytes_remaining(&self) -> usize {
        self.buf.len() - self.byte_offset()
    }

    fn read_bit(&self) -> CursorResult<Bit> {
        read_bit_as::<Bit>(&self.buf, self.byte_offset(), self.bit_position()).map(|b| {
            self.advance_bits(1);
            b
        })
    }

    fn peek_u8(&self) -> CursorResult<u8> {
        read_byte(&self.buf, self.byte_offset())
    }

    fn read_u8(&self) -> CursorResult<u8> {
        let byte = read_byte(self.buf, self.byte_offset())?;
        self.advance_bytes(1);
        Ok(byte)
    }

    fn read_bytes(&self, num_bytes: usize) -> CursorResult<&[u8]> {
        read_bytes(&self.buf, self.byte_offset(), num_bytes).and_then(|bytes| {
            self.advance_bytes(num_bytes);
            Ok(bytes)
        })
    }

    fn sub_buffer<'a, 'b>(&'a self, length: usize) -> CursorResult<ByteBufferSlice<'b>>
    where
        'a: 'b,
    {
        if self.byte_offset() + length > self.buf.len() {
            Err(BufferTooShort {
                start_pos: self.byte_offset(),
                num_bytes: length,
                buffer_size: self.buf.len(),
            })
        } else {
            let slice = ByteBufferSlice::from_slice(&self.buf, self.byte_offset(), length);
            self.advance_bytes(length);
            Ok(slice)
        }
    }
}
