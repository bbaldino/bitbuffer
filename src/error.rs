use thiserror::Error;

#[derive(Error, Debug)]
pub enum BitBufferError {
    #[error(
        "Buffer index out of bounds: tried accessing position {attempted_index}, but buffer has size {buffer_size}"
    )]
    OutOfBounds {
        attempted_index: usize,
        buffer_size: usize,
    },
    // 'BufferTooShort' is used when reading multiple bytes and some amount of it was within
    // bounds, but the full extent would go beyond the end of the buffer
    #[error("Buffer too short: tried accessing {num_bytes} bytes starting at position {start_pos}, but buffer only has size {buffer_size}")]
    BufferTooShort {
        start_pos: usize,
        num_bytes: usize,
        buffer_size: usize,
    },
}

pub type BitBufferResult<T> = Result<T, Box<dyn std::error::Error>>;
