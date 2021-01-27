use thiserror::Error;

#[derive(Error, Debug)]
pub enum CursorError {
    #[error(
        "Out of bounds: tried accessing position {attempted_index}, but buffer has size {buffer_size}"
    )]
    OutOfBounds {
        attempted_index: usize,
        buffer_size: usize,
    },
    #[error("Buffer too short: tried accessing {num_bytes} bytes starting at position {start_pos}, but buffer has size {buffer_size}")]
    BufferTooShort {
        start_pos: usize,
        num_bytes: usize,
        buffer_size: usize,
    },
}

pub type CursorResult<T> = Result<T, CursorError>;
