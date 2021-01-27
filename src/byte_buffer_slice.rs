use crate::bit::Bit;
use crate::error::CursorResult;
use crate::helpers::read_byte;
use crate::readable_buf::ReadableBuf;
use crate::some_readable_buf::SomeReadableBuf;
use std::cell::RefCell;
use std::ops::{AddAssign, Div};

pub struct ByteBufferSlice<'a> {
    pub buf: &'a [u8],
    pub bit_offset: RefCell<usize>,
}

/// Private
impl ByteBufferSlice<'_> {
    /// Return the current byte offset into the buffer
    fn byte_offset(&self) -> usize {
        self.bit_offset.borrow().div(8)
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

    fn read_bit_as_bool(&self) -> CursorResult<bool> {
        unimplemented!()
    }

    fn read_bit(&self) -> CursorResult<Bit> {
        unimplemented!()
    }

    fn read_u8(&self) -> CursorResult<u8> {
        let byte = read_byte(self.buf, self.byte_offset())?;
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
