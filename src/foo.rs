use crate::helpers::read_byte;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::Deref;

enum SomeReadableBuffer<'a> {
    ByteBufferSlice(NewByteBufferSlice<'a>),
}

impl<'a> Deref for SomeReadableBuffer<'a> {
    type Target = dyn NewTrait + 'a;

    fn deref(&self) -> &Self::Target {
        match self {
            SomeReadableBuffer::ByteBufferSlice(b) => b,
        }
    }
}

struct NewByteBuffer<'a> {
    buf: Vec<u8>,
    bit_offset: RefCell<usize>,
    marker: PhantomData<&'a ()>,
}

struct NewByteBufferSlice<'a> {
    pub buf: &'a [u8],
    pub bit_offset: RefCell<usize>,
}

trait NewTrait {
    fn read_u8(&self) -> u8;
    fn sub_buffer<'a, 'b>(&'a self, length: usize) -> SomeReadableBuffer;
}

trait NewTraitExtra {
    fn read_bit_as<T: From<u8>>(&self) -> T;
}

impl NewTraitExtra for dyn NewTrait {
    fn read_bit_as<T: From<u8>>(&self) -> T {
        unimplemented!()
    }
}

impl NewTrait for NewByteBuffer<'_> {
    fn read_u8(&self) -> u8 {
        unimplemented!()
    }

    fn sub_buffer<'a, 'b>(&'a self, length: usize) -> SomeReadableBuffer {
        let b = NewByteBufferSlice {
            // buf: &(self.buf[self.byte_offset()..][..length]),
            buf: &(self.buf[..length]),
            bit_offset: RefCell::new(0),
        };
        SomeReadableBuffer::ByteBufferSlice(b)
    }
}

impl NewTrait for NewByteBufferSlice<'_> {
    fn read_u8(&self) -> u8 {
        let byte = read_byte(self.buf, 0).unwrap();
        //self.advance_bytes(1);
        byte
    }

    fn sub_buffer<'a, 'b>(&'a self, length: usize) -> SomeReadableBuffer {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let bb = NewByteBuffer {
            buf: vec![1, 2, 3],
            bit_offset: RefCell::new(0),
            marker: Default::default(),
        };

        let sb = bb.sub_buffer(2);
        println!("num: {}", sb.read_u8());
    }
}
