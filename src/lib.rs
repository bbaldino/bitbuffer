pub mod bit;
pub mod byte_buffer;
mod byte_buffer_slice;
mod error;
mod helpers;
pub mod readable_buf;
pub mod some_readable_buf;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
