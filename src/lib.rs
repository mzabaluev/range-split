#![warn(rust_2018_idioms)]
#![warn(clippy::all)]

pub mod mem;
pub mod str;
mod take_range;

#[cfg(feature = "bytes")]
mod bytes;

pub use take_range::TakeRange;
