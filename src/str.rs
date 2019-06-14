//! Utilities for validating ranges on UTF-8 strings.

use core::fmt::Debug;
use core::ops::{Bound, RangeBounds};

/// Asserts that the given range is valid for the given string slice.
///
/// The first parameter shall coerce to an `&str` reference.
/// The second parameter shall be a reference to a value implementing
/// the standard library traits `RangeBounds<usize>` and `Debug`.
///
/// The range is valid if it fits within the slice and its bounds are
/// on UTF-8 code point boundaries. If either of these checks fails,
/// `panic!` is invoked with a description of the failure.
///
/// # Examples
///
/// ```
/// # use range_split::assert_str_range;
/// let s = &"Hello";
/// assert_str_range!(s, &(..=2));
/// assert_str_range!(s, &(..0));
/// assert_str_range!(s, &(5..));
/// ```
///
/// ```should_panic
/// # use range_split::assert_str_range;
/// let s = "Привет".to_string();
/// assert_str_range!(&s, &(..1)); // fails due to splitting a UTF-8 sequence
/// ```
#[macro_export]
macro_rules! assert_str_range {
    ($s:expr, $r:expr) => {
        if !$crate::str::is_valid_range($s, $r) {
            $crate::str::range_fail($s, $r)
        }
    };
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
    validate_start_bound(s, range.start_bound()).is_valid()
        && validate_end_bound(s, range.end_bound()).is_valid()
}

enum BoundValidity {
    Valid,
    OutOfBuffer,
    NotCharBoundary,
}

impl BoundValidity {
    #[inline]
    fn is_valid(&self) -> bool {
        use BoundValidity::*;

        match self {
            Valid => true,
            _ => false,
        }
    }
}

#[inline]
fn validate_start_bound(s: &str, bound: Bound<&usize>) -> BoundValidity {
    use Bound::*;
    use BoundValidity::*;

    match bound {
        Unbounded => Valid,
        Included(index) => validate_index(s, *index),
        Excluded(index) => validate_next_index(s, *index),
    }
}

#[inline]
fn validate_end_bound(s: &str, bound: Bound<&usize>) -> BoundValidity {
    use Bound::*;
    use BoundValidity::*;

    match bound {
        Unbounded => Valid,
        Excluded(index) => validate_index(s, *index),
        Included(index) => validate_next_index(s, *index),
    }
}

#[inline]
fn validate_index(s: &str, index: usize) -> BoundValidity {
    use BoundValidity::*;

    // .is_char_boundary() fails on OOB as well, but we check it as
    // the fast path first and discern the failure cause later.
    if s.is_char_boundary(index) {
        Valid
    } else if index > s.len() {
        OutOfBuffer
    } else {
        NotCharBoundary
    }
}

#[inline]
fn validate_next_index(s: &str, index: usize) -> BoundValidity {
    use BoundValidity::*;

    // The check for OOB also rules out integer overflow in index + 1
    if index >= s.len() {
        #[cold]
        OutOfBuffer
    } else if s.is_char_boundary(index + 1) {
        Valid
    } else {
        NotCharBoundary
    }
}

#[doc(hidden)]
#[cold]
#[inline(never)]
pub fn range_fail<R>(s: &str, range: &R) -> !
where
    R: RangeBounds<usize> + Debug,
{
    use BoundValidity::*;

    let start_validity = validate_start_bound(s, range.start_bound());
    let end_validity = validate_end_bound(s, range.end_bound());
    match (start_validity, end_validity) {
        (OutOfBuffer, _) | (_, OutOfBuffer) => {
            panic!("range {:?} is out of bounds of the string buffer", range)
        }
        (NotCharBoundary, _) | (_, NotCharBoundary) => {
            panic!("range {:?} does not split on a UTF-8 boundary", range)
        }
        (Valid, Valid) => unreachable!("there was no problem with the range"),
    }
}
