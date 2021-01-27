use crate::error::CursorError::BufferOverflow;
use crate::error::CursorResult;

/// Retrieve the |bit_offset| bit of the |byte_offset| byte from |buf| and return it as T
pub(crate) fn read_bit_as<T: From<u8>>(
    buf: &[u8],
    byte_offset: usize,
    bit_offset: usize,
) -> CursorResult<T> {
    if let Some(byte) = buf.get(byte_offset) {
        // Shift right until the bit pointed to by bit_offset is the LSB
        let shift_amount = 7 - bit_offset;
        Ok(((byte >> shift_amount) & 0x1).into())
    } else {
        Err(BufferOverflow(
            format!(
                "Buffer exhausted: cannot access byte {}, bit {}.  Buffer has length {}",
                byte_offset,
                bit_offset,
                buf.len()
            )
            .to_owned(),
        ))
    }
}

pub(crate) fn read_byte(buf: &[u8], byte_offset: usize) -> CursorResult<u8> {
    if let Some(b) = buf.get(byte_offset) {
        Ok(*b)
    } else {
        Err(BufferOverflow(format!(
            "Buffer exhausted: cannot access byte {}.  Buffer has length {}",
            byte_offset,
            buf.len()
        )))
    }
}

pub(crate) fn read_bytes(source: &[u8], start_pos: usize, num_bytes: usize) -> CursorResult<&[u8]> {
    let end_pos = start_pos + num_bytes;
    source.get(start_pos..end_pos).ok_or(BufferOverflow(format!(
        "Cannot read {} bytes starting at position {} from buffer with length {}",
        num_bytes,
        start_pos,
        source.len()
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_bit_as() {
        let buf = vec![0b00001111];
        let bit = read_bit_as(&buf, 0, 5).unwrap();
        assert_eq!(1, bit);
    }

    #[test]
    fn test_read_bit_as_error() {
        let buf = vec![0b00001111];
        assert!(read_bit_as::<u8>(&buf, 1, 5).err().is_some());
    }

    #[test]
    fn test_read_bytes() {
        let buf: Vec<u8> = vec![1, 2, 3, 4];

        let bytes = read_bytes(&buf, 1, 2).unwrap();
        assert_eq!([2, 3], bytes);

        assert!(read_bytes(&buf, 5, 1).err().is_some());
    }
}
