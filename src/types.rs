use core::{mem, ptr};
use std::{error, fmt};

pub trait Bytes {
    fn as_ref(&self) -> &[u8];
}

/// Enum for the LEFT | RIGHT args used by some commands
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    message: String,
}

impl Error {
    fn new(message: String) -> Error {
        Error { message }
    }

    pub fn into_string(self) -> String {
        self.into()
    }
}

impl AsRef<str> for Error {
    fn as_ref(&self) -> &str {
        &self.message
    }
}

impl From<Error> for String {
    fn from(e: Error) -> String {
        e.message
    }
}

impl From<rocksdb::Error> for Error {
    fn from(e: rocksdb::Error) -> Self {
        Error {
            message: e.into_string(),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.message.fmt(formatter)
    }
}


/// [see](https://github.com/google/flatbuffers/blob/master/rust/flatbuffers/src/endian_scalar.rs)
/// Trait for values that must be stored in little-endian byte order, but
/// might be represented in memory as big-endian. Every type that implements
/// EndianScalar is a valid FlatBuffers scalar value.
///
/// The Rust stdlib does not provide a trait to represent scalars, so this trait
/// serves that purpose, too.
///
/// Note that we do not use the num-traits crate for this, because it provides
/// "too much". For example, num-traits provides i128 support, but that is an
/// invalid FlatBuffers type.
pub trait EndianScalar: Sized + PartialEq + Copy + Clone {
    fn to_little_endian(self) -> Self;
    fn from_little_endian(self) -> Self;
}


/// Macro for implementing an endian conversion using the stdlib `to_le` and
/// `from_le` functions. This is used for integer types. It is not used for
/// floats, because the `to_le` and `from_le` are not implemented for them in
/// the stdlib.
macro_rules! impl_endian_scalar_stdlib_le_conversion {
    ($ty:ident) => {
        impl EndianScalar for $ty {
            #[inline]
            fn to_little_endian(self) -> Self {
                Self::to_le(self)
            }
            #[inline]
            fn from_little_endian(self) -> Self {
                Self::from_le(self)
            }
        }
    };
}

impl_endian_scalar_stdlib_le_conversion!(u16);
impl_endian_scalar_stdlib_le_conversion!(u32);
impl_endian_scalar_stdlib_le_conversion!(u64);
impl_endian_scalar_stdlib_le_conversion!(i16);
impl_endian_scalar_stdlib_le_conversion!(i32);
impl_endian_scalar_stdlib_le_conversion!(i64);


type KeySized = [u8; 10];

pub struct MetaKey(KeySized);


impl From<KeySized> for MetaKey {
    fn from(key: KeySized) -> Self {
        MetaKey(key)
    }
}

impl AsRef<[u8]> for MetaKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl MetaKey {
    const len_seq: usize = mem::size_of::<i16>();
    const len_meta_key: usize = mem::size_of::<MetaKey>();
    const len_key: usize = MetaKey::len_meta_key - MetaKey::len_seq;

    pub fn new() -> Self {
        MetaKey([0; MetaKey::len_meta_key])
    }

    pub fn key(&self) -> &[u8] {
        &self.0[..MetaKey::len_key]
    }

    pub fn set_key(&mut self, key: KeySized) {
        self.0[0..MetaKey::len_key].copy_from_slice(&key);
    }

    pub fn sep(&self) -> i16 {
        let mut mem = mem::MaybeUninit::<i16>::uninit();
        unsafe {
            ptr::copy_nonoverlapping(
                self.0[MetaKey::len_key..].as_ptr(),
                mem.as_mut_ptr() as *mut u8,
                mem::size_of::<i16>(),
            );
            mem.assume_init()
        }.from_little_endian()
    }

    pub fn set_sep(&mut self, sep: i16) {
        let x_le = sep.to_little_endian();

        unsafe {
            core::ptr::copy_nonoverlapping(
                &x_le as *const i16 as *const u8,
                self.0[MetaKey::len_key..].as_mut_ptr(),
                mem::size_of::<i16>(),
            );
        }
    }
}


