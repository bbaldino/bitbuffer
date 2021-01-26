
#[derive(Debug)]
pub enum CursorError {
    BufferOverflow(String)
}

pub type CursorResult<T> = Result<T, CursorError>;
