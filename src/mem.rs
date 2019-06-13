use core::ops::{RangeTo, RangeToInclusive};

#[inline]
pub fn convert_inclusive_range(
    range: RangeToInclusive<usize>,
) -> RangeTo<usize> {
    // Adding 1 shall succeed for a valid inclusive range
    // in a collection in memory.
    ..range.end.checked_add(1).expect("integer overflow")
}
