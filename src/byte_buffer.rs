use crate::bit::Bit;
use crate::byte_buffer_slice::ByteBufferSlice;
use crate::error::CursorResult;
use crate::helpers::{read_byte, take_bit_as};
use crate::readable_buf::{ReadableBuf, ReadableBufExtra};
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
        take_bit_as::<Bit>(&self.buf, self.byte_offset(), self.bit_position()).map(|b| b.into())
    }

    fn read_bit(&self) -> CursorResult<Bit> {
        unimplemented!()
    }

    fn read_u8(&self) -> CursorResult<u8> {
        let byte = read_byte(&self.buf, self.byte_offset())?;
        self.advance_bytes(1);
        Ok(byte)
    }

    fn sub_buffer<'a, 'b>(&'a self, length: usize) -> CursorResult<Box<dyn ReadableBuf + 'b>>
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Sub;

    #[test]
    fn test() {
        let data: Vec<u8> = vec![1, 2, 3];
        let bb = ByteBuffer::from_vec(data);

        foo(&bb);
    }

    #[test]
    fn test2() {
        let data: Vec<u8> = vec![0b11110000, 2, 3];
        let bb = ByteBuffer::from_vec(data);

        println!("Got bool {}", bb.read_bit_as_bool().unwrap());
    }

    fn foo(buf: &dyn ReadableBuf) {
        let v = buf.read_u8().unwrap();
        println!("Got value {}", v);
        println!("{} bytes left in original buffer", buf.bytes_remaining());
        let sb = buf.sub_buffer(2).unwrap();

        let v = sb.read_u8().unwrap();
        println!("Got value {}", v);
        println!("{} bytes left in sub buffer", sb.bytes_remaining());

        let ssb = sb.sub_buffer(1).unwrap();
        let v = ssb.read_u8().unwrap();
        println!("Got value {}", v);
        println!("{} bytes left in sub buffer", ssb.bytes_remaining());
    }
}
