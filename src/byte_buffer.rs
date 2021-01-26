use crate::bit::Bit;
use crate::byte_buffer_slice::ByteBufferSlice;
use crate::error::CursorResult;
use crate::readable_buf::ReadableBuf;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::{AddAssign, Div, Rem};

pub struct ByteBuffer<'a> {
    buf: Vec<u8>,
    bit_offset: RefCell<usize>,
    marker: PhantomData<&'a ()>,
}

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

impl<'a> ReadableBuf<'a> for ByteBuffer<'a> {
    fn bytes_remaining(&self) -> usize {
        self.bytes_remaining()
    }

    fn read_bit_as_bool(&self) -> CursorResult<bool> {
        unimplemented!()
    }

    fn read_bit(&self) -> CursorResult<Bit> {
        unimplemented!()
    }

    fn read_u8(&self) -> CursorResult<u8> {
        unimplemented!()
    }

    fn sub_buffer<'b>(&'a self, length: usize) -> CursorResult<Box<dyn ReadableBuf<'b> + 'b>>
    where
        'a: 'b,
    {
        let b = ByteBufferSlice {
            buf: &(self.buf[self.byte_offset()..][..length]),
            bit_offset: RefCell::new(0),
        };
        Ok(Box::new(b))
    }
}
