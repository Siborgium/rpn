
cfg_if::cfg_if! {
    if #[cfg(feature = "f32")] {
        pub type Type = f32;
        pub type Raw = u32;
        pub use std::f32::{ INFINITY, NAN };
    } else if #[cfg(feature = "f64")] {
        pub type Type = f64;
        pub type Raw = u64;
        pub use std::f64::{ INFINITY, NAN };
    } else if #[cfg(feature = "u32")] {
        pub type Type = u32;
        pub type Raw = u32;
        pub use std::u32::MAX;
    } else if #[cfg(feature = "u64")] {
        pub type Type = u64;
        pub type Raw = u64;
        pub use std::u64::MAX;
    }
}

#[cfg(any(feature = "f32", feature = "f64"))]
pub const DEFINES: [Type; 3] = [INFINITY, -INFINITY, NAN];

#[cfg(any(feature = "u32", feature = "u64"))]
pub const DEFINES: [Type; 3] = [MAX, MAX - 1, MAX - 2];

pub const STACK_SIZE: usize = 1024 * 1024 * 256; // 256MB
pub const CHUNK_SIZE: usize = 1024 * 32; // 32KB
pub const SIZE: usize = std::mem::size_of::<Type>();

#[inline]
pub fn norm(v: Type) -> Type {
    cfg_if::cfg_if! {
        if #[cfg(any(feature = "f32", feature = "f64"))] {
            v.abs()
        } else {
            v
        }
    }
}

#[inline]
pub fn to_bits(v: Type) -> Raw {
    cfg_if::cfg_if! {
        if #[cfg(any(feature = "f32", feature = "f64"))] {
            v.to_bits()
        } else {
            v
        }
    }
}

#[inline]
pub fn from_bits(v: Raw) -> Type {
    cfg_if::cfg_if! {
        if #[cfg(any(feature = "f32", feature = "f64"))] {
            Type::from_bits(v)
        } else {
            v
        }
    }
}
