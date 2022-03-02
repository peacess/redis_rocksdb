pub trait Bytes {
    fn as_ref(&self) -> &[u8];
}

/// Enum for the LEFT | RIGHT args used by some commands
pub enum Direction {
    Left,
    Right,
}

