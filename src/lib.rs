//! Utilities for splitting sequences with range parameters.
//!
//! The `TakeRange` trait provides polymorphic, easily memorizable methods
//! for splitting indexed sequences with a parameter given in range syntax.
//!
//! ```
//! # #[cfg(features = "bytes")]
//! # fn main() {
//! # use bytes::Bytes;
//! use range_split::TakeRange;
//!
//! let mut buf = Bytes::from("Hello, world");
//! let p = buf.take_range(..5);
//! buf.remove_range(2..);
//! assert_eq!(p, "Hello");
//! assert_eq!(buf, ", ");
//! # }
//! # #[cfg(not(features = "bytes"))]
//! # fn main() {}
//! ```
//!
//! is equivalent to
//!
//! ```
//! # #[cfg(features = "bytes")]
//! # fn main() {
//! # use bytes::Bytes;
//! #
//! let mut buf = Bytes::from("Hello, world");
//! let p = buf.split_to(5);
//! buf.truncate(2);
//! # }
//! # #[cfg(not(features = "bytes"))]
//! # fn main() {}
//! ```
//!
//! Implementations of `TakeRange` are provided for `Bytes` and `BytesMut`
//! from the crate `bytes` if the `bytes` compile-time feature is enabled.

#![warn(rust_2018_idioms)]
#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod mem;
pub mod str;
mod take_range;

#[macro_use]
mod impl_macro;

#[cfg(feature = "bytes")]
mod bytes;

pub use take_range::TakeRange;
