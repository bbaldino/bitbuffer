use std::ops::{BitOrAssign, ShlAssign};

use crate::error::BitBufferError::{self, BufferTooShort, OutOfBounds};
use crate::error::BitBufferResult;

/// Retrieve the |bit_offset| bit of the |byte_offset| byte from |buf| and return it as T
pub(crate) fn read_bit_as<T: From<u8>>(
    buf: &[u8],
    byte_offset: usize,
    bit_offset: usize,
) -> BitBufferResult<T> {
    if let Some(byte) = buf.get(byte_offset) {
        // Shift right until the bit pointed to by bit_offset is the LSB
        let shift_amount = 7 - bit_offset;
        Ok(((byte >> shift_amount) & 0x1).into())
    } else {
        Err(OutOfBounds {
            attempted_index: byte_offset,
            buffer_size: buf.len(),
        })
    }
}

fn get_u8_mask(start_position: usize, num_bits: usize) -> Option<u8> {
    if start_position + num_bits > 8 {
        return None;
    }
    let mut mask = match num_bits {
        0 => 0u8,
        1 => 0b1,
        2 => 0b11,
        3 => 0b111,
        4 => 0b1111,
        5 => 0b11111,
        6 => 0b111111,
        7 => 0b1111111,
        8 => 0b11111111,
        _ => return None,
    };

    mask <<= 8 - start_position - num_bits;
    Some(mask)
}

/// Note that this function does NOT (currently) support reading bits across
/// byte boundaries
pub(crate) fn read_bits_as<T>(
    buf: &[u8],
    byte_offset: usize,
    bit_offset: usize,
    num_bits: usize,
) -> BitBufferResult<T>
where
    T: From<u8> + Default + ShlAssign<u8> + BitOrAssign,
{
    if bit_offset + num_bits > 8 {
        // TODO: better error
        return Err(BitBufferError::OutOfBounds {
            attempted_index: num_bits,
            buffer_size: (7 - bit_offset),
        });
    }
    let mask = get_u8_mask(bit_offset, num_bits).ok_or(BitBufferError::OutOfBounds {
        attempted_index: num_bits,
        buffer_size: (7 - bit_offset),
    })?;
    let mut result = buf[byte_offset] & mask;

    // Now shift the result back so the masked values are all the way to the right
    result >>= 8 - num_bits - bit_offset;

    Ok(result.into())
}

pub(crate) fn read_byte(buf: &[u8], byte_offset: usize) -> BitBufferResult<u8> {
    if let Some(b) = buf.get(byte_offset) {
        Ok(*b)
    } else {
        Err(OutOfBounds {
            attempted_index: byte_offset,
            buffer_size: buf.len(),
        })
    }
}

pub(crate) fn read_bytes(
    source: &[u8],
    start_pos: usize,
    num_bytes: usize,
) -> BitBufferResult<&[u8]> {
    let end_pos = start_pos + num_bytes;
    source.get(start_pos..end_pos).ok_or(BufferTooShort {
        start_pos,
        num_bytes,
        buffer_size: source.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mask() {
        assert_eq!(0b00011100, get_u8_mask(3, 3).unwrap());
        assert_eq!(0b11100000, get_u8_mask(0, 3).unwrap());
        assert_eq!(0b00000111, get_u8_mask(5, 3).unwrap());
        assert_eq!(0b11111111, get_u8_mask(0, 8).unwrap());
        assert_eq!(0b00000000, get_u8_mask(7, 0).unwrap());
        assert_eq!(0b00000001, get_u8_mask(7, 1).unwrap());
        assert_eq!(None, get_u8_mask(7, 3));
    }

    #[test]
    fn test_read_bits_as() {
        let data = vec![0b00110011, 0b00001111];
        assert_eq!(0b0011u8, read_bits_as::<u8>(&data, 0, 0, 4).unwrap());
        assert_eq!(0b0011u16, read_bits_as::<u16>(&data, 0, 0, 4).unwrap());
        assert_eq!(0b0011u8, read_bits_as::<u8>(&data, 0, 4, 4).unwrap());
        assert_eq!(0b0000u8, read_bits_as::<u8>(&data, 1, 0, 4).unwrap());
        assert_eq!(0b1111u8, read_bits_as::<u8>(&data, 1, 4, 4).unwrap());
        assert_eq!(0b0110011u8, read_bits_as::<u8>(&data, 0, 1, 7).unwrap());
    }

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
