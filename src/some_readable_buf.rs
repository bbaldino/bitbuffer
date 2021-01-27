use crate::byte_buffer::ByteBuffer;
use crate::byte_buffer_slice::ByteBufferSlice;
use crate::readable_buf::ReadableBuf;
use std::ops::Deref;

/// This enum allows methods like 'sub_buffer' to return another ReadableBuf without
/// using dynamic dispatch (which would require a heap allocation).
pub enum SomeReadableBuf<'a> {
    ByteBuffer(ByteBuffer<'a>),
    ByteBufferSlice(ByteBufferSlice<'a>),
}

/// Implementing Deref for SomeReadableBuf allows it to be used directly as a
/// ReadableBuf trait object.
impl<'a> std::ops::Deref for SomeReadableBuf<'a> {
    type Target = dyn ReadableBuf + 'a;

    fn deref(&self) -> &Self::Target {
        match self {
            SomeReadableBuf::ByteBuffer(b) => b,
            SomeReadableBuf::ByteBufferSlice(b) => b,
        }
    }
}
