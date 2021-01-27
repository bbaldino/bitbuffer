use crate::byte_buffer::ByteBuffer;
use crate::byte_buffer_slice::ByteBufferSlice;
use crate::readable_buf::ReadableBuf;

/// This enum allows methods like 'sub_buffer' to return a ReadableBuf without
/// using dynamic dispatch (which would require a heap allocation)--instead we return
/// an instance of this enum.  Combined with the Deref impl below, the enum can be
/// used as a ReadableBuf.
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
