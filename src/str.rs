//! Utilities for validating ranges on UTF-8 strings.

use core::ops::{Bound, RangeBounds};

/// Asserts that the given range is valid for the given string slice.
///
/// The first parameter shall be of a type implementing `AsRef<str>`.
/// The second parameter shall be of a type implementing
/// the standard library trait `RangeBounds<usize>`.
///
/// The range is valid if it fits within the slice and its bounds are
/// on UTF-8 code point boundaries. If either of these checks fails,
/// `panic!` is invoked with a description of the failure.
///
/// # Examples
///
/// ```
/// # use range_split::assert_str_range;
/// let s = "Hello";
/// assert_str_range!(s, ..0);
/// assert_str_range!(s, 5..);
///
/// let r = (..=2);
/// assert_str_range!(s, r);
/// let (head, tail) = s.as_bytes().split_at(r.end + 1);
/// ```
///
/// ```should_panic
/// # use range_split::assert_str_range;
/// let s = "Привет".to_string();
/// assert_str_range!(s, ..1); // fails due to splitting a UTF-8 sequence
/// ```
#[macro_export]
macro_rules! assert_str_range {
    ($s:expr, $r:expr) => {{
        let s = &$s;
        let r = &$r;
        if !$crate::str::is_valid_range(s, r) {
            $crate::str::range_fail(s, r)
        }
    }};
}

/// Checks that `range` is valid for splitting the string slice `s`.
///
/// The range is valid if it fits within the slice and its bounds are
/// on UTF-8 code point boundaries.
pub fn is_valid_range<S, R>(s: S, range: &R) -> bool
where
    S: AsRef<str>,
    R: RangeBounds<usize>,
{
    let s = s.as_ref();
    validate_start_bound(s, range.start_bound()).is_ok()
        && validate_end_bound(s, range.end_bound()).is_ok()
}

enum InvalidBound {
    OutOfBuffer,
    NotCharBoundary,
}

#[inline]
fn validate_start_bound(
    s: &str,
    bound: Bound<&usize>,
) -> Result<(), InvalidBound> {
    use Bound::*;

    match bound {
        Unbounded => Ok(()),
        Included(index) => validate_index(s, *index),
        Excluded(index) => validate_next_index(s, *index),
    }
}

#[inline]
fn validate_end_bound(
    s: &str,
    bound: Bound<&usize>,
) -> Result<(), InvalidBound> {
    use Bound::*;

    match bound {
        Unbounded => Ok(()),
        Excluded(index) => validate_index(s, *index),
        Included(index) => validate_next_index(s, *index),
    }
}

#[inline]
fn validate_index(s: &str, index: usize) -> Result<(), InvalidBound> {
    use InvalidBound::*;

    // .is_char_boundary() fails on OOB as well, but we check it as
    // the fast path first and discern the failure cause later.
    if s.is_char_boundary(index) {
        Ok(())
    } else if index > s.len() {
        Err(OutOfBuffer)
    } else {
        Err(NotCharBoundary)
    }
}

#[inline]
fn validate_next_index(s: &str, index: usize) -> Result<(), InvalidBound> {
    use InvalidBound::*;

    // The check for OOB also rules out integer overflow in index + 1
    if index >= s.len() {
        #[cold]
        Err(OutOfBuffer)
    } else if s.is_char_boundary(index + 1) {
        Ok(())
    } else {
        Err(NotCharBoundary)
    }
}

#[doc(hidden)]
#[cold]
pub fn range_fail<S, R>(s: S, range: &R) -> !
where
    S: AsRef<str>,
    R: RangeBounds<usize>,
{
    range_fail_internal(s.as_ref(), range.start_bound(), range.end_bound())
}

fn range_fail_internal(
    s: &str,
    start_bound: Bound<&usize>,
    end_bound: Bound<&usize>,
) -> ! {
    use InvalidBound::*;

    let start_validity = validate_start_bound(s, start_bound);
    let end_validity = validate_end_bound(s, end_bound);
    let r = (start_bound, end_bound);
    match (start_validity, end_validity) {
        (Err(OutOfBuffer), _) | (_, Err(OutOfBuffer)) => {
            panic!("range {:?} is out of bounds", r)
        }
        (Err(NotCharBoundary), _) | (_, Err(NotCharBoundary)) => {
            panic!("range {:?} does not split on a UTF-8 boundary", r)
        }
        (Ok(()), Ok(())) => unreachable!("there was no problem with the range"),
    }
}
