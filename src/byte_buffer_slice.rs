use crate::bit::Bit;
use crate::error::CursorResult;
use crate::readable_buf::ReadableBuf;
use std::cell::RefCell;

pub struct ByteBufferSlice<'a> {
    pub buf: &'a [u8],
    pub bit_offset: RefCell<usize>,
}

impl ByteBufferSlice<'_> {}

impl<'a> ReadableBuf<'a> for ByteBufferSlice<'a> {
    fn bytes_remaining(&self) -> usize {
        unimplemented!()
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
        unimplemented!()
    }
}
