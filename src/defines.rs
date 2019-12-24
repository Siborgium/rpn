
#[cfg(feature = "f32")]
pub type Type = f32;

#[cfg(feature = "f32")]
pub use std::f32::{ INFINITY, NAN };

#[cfg(feature = "f64")]
pub type Type = f64;

#[cfg(feature = "f64")]
pub use std::f64::{ INFINITY, NAN };

#[cfg(feature = "u32")]
pub type Type = u32;

#[cfg(feature = "u32")]
pub use std::u32::MAX;

#[cfg(feature = "u64")]
pub type Type = u64;

#[cfg(feature = "u64")]
pub use std::u32::MAX;

#[cfg(any(feature = "f32", feature = "f64"))]
pub const DEFINES: [Type; 3] = [INFINITY, -INFINITY, NAN];

#[cfg(any(feature = "u32", feature = "u64"))]
pub const DEFINES: [Type; 3] = [MAX, MAX - 1, MAX - 2];

#[allow(dead_code)]
pub const STACK_SIZE: usize = 1024 * 1024 * 256; // 256MB
#[allow(dead_code)]
pub const CHUNK_SIZE: usize = 1024 * 4; // 4KB
#[allow(dead_code)]
pub const SIZE: usize = std::mem::size_of::<Type>();
