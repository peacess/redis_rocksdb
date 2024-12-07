use core::mem;
use std::{
    fmt,
    fmt::{Display, Formatter},
};

pub trait Bytes: AsRef<[u8]> {}

impl Bytes for &[u8] {}

impl Bytes for Vec<u8> {}

/// Enum for the LEFT | RIGHT args used by some commands
pub enum Direction {
    Left,
    Right,
}

/// 类型：表示成员或字段的总个数
pub type LenType = u64;
/// 类型：表示成员或字段名字的长度bytes
pub type BytesType = i64;

pub const BYTES_LEN_TYPE: usize = mem::size_of::<LenType>();

#[inline]
pub fn read_len_type(bytes: &[u8]) -> LenType {
    read_int(bytes)
}

#[inline]
pub fn write_len_type(bytes: &mut [u8], len: LenType) {
    write_int(bytes, len)
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

pub fn read_int<T: EndianScalar>(bytes: &[u8]) -> T {
    let mut mem = core::mem::MaybeUninit::<T>::uninit();
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), mem.as_mut_ptr() as *mut u8, core::mem::size_of::<T>());
        mem.assume_init()
    }
    .from_little_endian()
}

pub fn read_int_ptr<T: EndianScalar>(p: *const u8) -> T {
    let mut mem = core::mem::MaybeUninit::<T>::uninit();
    unsafe {
        core::ptr::copy_nonoverlapping(p, mem.as_mut_ptr() as *mut u8, core::mem::size_of::<T>());
        mem.assume_init()
    }
    .from_little_endian()
}

pub fn write_int<T: EndianScalar>(bytes: &mut [u8], value: T) {
    let x_le = value.to_little_endian();
    unsafe {
        core::ptr::copy_nonoverlapping(&x_le as *const T as *const u8, bytes.as_mut_ptr(), core::mem::size_of::<T>());
    }
}

pub fn write_int_ptr<T: EndianScalar>(p: *mut u8, value: T) {
    let x_le = value.to_little_endian();
    unsafe {
        core::ptr::copy_nonoverlapping(&x_le as *const T as *const u8, p, core::mem::size_of::<T>());
    }
}

pub fn to_little_endian_array<T: EndianScalar>(value: T) -> Vec<u8> {
    let mut temp = Vec::<u8>::with_capacity(mem::size_of::<T>());
    write_int(&mut temp, value);
    temp
}

///
/// ```rust
/// struct  MetaKey{
///     hash: u64,
///     sep: u16,
/// }
/// ```
type MetaKeyArray = [u8; 10];

#[derive(Clone, Debug)]
pub struct MetaKey(MetaKeyArray);

impl MetaKey {
    const LEN_SEQ: usize = mem::size_of::<u16>();
    const LEN_META_KEY: usize = mem::size_of::<MetaKey>();
    const LEN_KEY: usize = MetaKey::LEN_META_KEY - MetaKey::LEN_SEQ;

    #[inline]
    pub fn read(bytes: &[u8]) -> Option<&MetaKey> {
        let t = &bytes[..MetaKey::LEN_META_KEY];
        if t.eq(&[0u8; MetaKey::LEN_META_KEY]) {
            None
        } else {
            Some(unsafe { &*(t.as_ptr() as *const MetaKey) })
        }
    }

    #[inline]
    pub fn read_mut(bytes: &[u8]) -> Option<&mut MetaKey> {
        let t = &bytes[..MetaKey::LEN_META_KEY];
        if t.eq(&[0u8; MetaKey::LEN_META_KEY]) {
            None
        } else {
            Some(unsafe { &mut *(t.as_ptr() as *mut MetaKey) })
        }
    }

    #[inline]
    pub fn write(bytes: &mut [u8], value: &Option<&MetaKey>) {
        match value {
            None => unsafe {
                bytes.as_mut_ptr().write_bytes(0u8, MetaKey::LEN_META_KEY);
            },
            Some(m) => {
                bytes[..MetaKey::LEN_META_KEY].copy_from_slice(m.as_ref());
            }
        }
    }

    pub fn new() -> Self {
        MetaKey([0; MetaKey::LEN_META_KEY])
    }

    pub fn new_add(&self) -> Self {
        let mut n = MetaKey(self.0);
        n.add_sep(1);
        n
    }

    pub fn key(&self) -> u64 {
        read_int(&self.0)
    }

    pub fn set_key(&mut self, key: u64) {
        write_int(&mut self.0, key)
    }

    pub fn sep(&self) -> u16 {
        read_int(&self.0[MetaKey::LEN_KEY..])
    }

    pub fn add_sep(&mut self, diff: u16) {
        let old: u16 = read_int(&self.0[MetaKey::LEN_KEY..]);
        write_int(&mut self.0[MetaKey::LEN_KEY..], old + diff);
    }

    pub fn set_sep(&mut self, sep: u16) {
        write_int(&mut self.0[MetaKey::LEN_KEY..], sep)
    }
}

impl From<MetaKeyArray> for MetaKey {
    fn from(key: MetaKeyArray) -> Self {
        MetaKey(key)
    }
}

impl AsRef<[u8]> for MetaKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Display for MetaKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.key(), self.sep())
    }
}

#[cfg(test)]
mod test {
    use crate::{read_int_ptr, write_int_ptr};

    #[test]
    fn test_write_int_ptr() {
        let mut data = Vec::with_capacity(16);
        data.resize(data.capacity(), 0 as u8);

        let b = 256 as i64;
        write_int_ptr(data.as_mut_ptr(), b);
        let b2: i64 = read_int_ptr(data.as_ptr());
        assert_eq!(b, b2)
    }
}
