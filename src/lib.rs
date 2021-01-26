use std::marker::PhantomData;

mod bit;
mod byte_buffer;
mod byte_buffer_slice;
mod error;
mod helpers;
mod readable_buf;

struct Foo<'a> {
    buf: Vec<u8>,
    marker: PhantomData<&'a ()>,
}

struct Bar<'a> {
    buf: &'a [u8],
}

impl<'a> Foo<'a> {
    fn new(buf: Vec<u8>) -> Foo<'a> {
        Foo {
            buf,
            marker: PhantomData,
        }
    }

    fn sub<'b>(&'a self) -> Bar<'b>
    where
        'a: 'b,
    {
        Bar { buf: &self.buf }
    }
}

#[cfg(test)]
mod tests {
    use crate::byte_buffer::ByteBuffer;
    use crate::readable_buf::{ReadableBuf, ReadableBufExtra};
    use crate::Foo;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn foo() {
        let data: Vec<u8> = vec![1, 2, 3];

        let foo = Foo::new(data);

        let bar = foo.sub();

        println!("bar: {}", bar.buf[1]);
    }

    fn blah() {
        let data: Vec<u8> = vec![1, 2, 3];
        let bb = ByteBuffer::from_vec(data);

        // In order to make ReadableBuf usable as a trait object (so I can do things like
        // return Box<dyn ReadableBuf>), I needed to move the generic methods out of the
        // ReadableBuf trait.  I can still achieve the same thing by moving _those_ methods
        // into another trait, and then implementing that trait for ReadableBuf.  However,
        // if I have a ByteBuffer, it looks like I'm able to call the ReadableBuf methods
        // on it directly, but _can't_ call the methods defined in the secondary trait
        // directly: I'd need to cast it like below.  I don't think this will be a big
        // problem, though, as all the methods will use references to ReadableBuf, and that
        // works fine with calling methods defined in the secondary trait.
        let x: u8 = (&bb as &dyn ReadableBuf).read_bit_as().unwrap();
    }
}
