//! Utilities for working with ranges of collections in memory.

use core::ops::{RangeTo, RangeToInclusive};

/// Converts a range with an inclusive end bound into the equivalent
/// range with the exclusive end bound.
///
/// This allows reusing the implementation of `TakeRange<RangeTo<usize>>`
/// to implement `TakeRange<RangeToInclusive<usize>>`. The exclusive end
/// bound value also corresponds directly to container lengths and
/// zero-based offsets, taking away the need to increment the value and
/// check for a possible integer overflow.
///
/// This conversion is valid for any valid range within a collection in memory,
/// since the entire length of the collection must fit within a `usize` value.
///
/// # Panics
///
/// Panics if the inclusive end bound is `std::usize::MAX`.
///
#[inline]
pub fn convert_inclusive_range(
    range: RangeToInclusive<usize>,
) -> RangeTo<usize> {
    ..range.end.checked_add(1).expect("integer overflow")
}
