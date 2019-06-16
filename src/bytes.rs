use crate::mem;

use bytes::{Bytes, BytesMut};

use std::ops::{RangeFrom, RangeFull, RangeTo, RangeToInclusive};

impl_take_range! {
    <RangeFull> for Bytes {
        #[inline]
        fn take_range(&mut self, _range) {
            self.split_off(0)
        }
        #[inline]
        fn remove_range(&mut self, _range) {
            self.clear()
        }
    }
    <RangeFull> for BytesMut {
        #[inline]
        fn take_range(&mut self, _range) {
            self.take()
        }
        #[inline]
        fn remove_range(&mut self, _range) {
            self.clear()
        }
    }
    <RangeFrom<usize>> for Bytes {
        #[inline]
        fn take_range(&mut self, range) {
            self.split_off(range.start)
        }
        #[inline]
        fn remove_range(&mut self, range) {
            self.truncate(range.start)
        }
    }
    <RangeFrom<usize>> for BytesMut {
        #[inline]
        fn take_range(&mut self, range) {
            self.split_off(range.start)
        }
        #[inline]
        fn remove_range(&mut self, range) {
            self.truncate(range.start)
        }
    }
    <RangeTo<usize>> for Bytes {
        #[inline]
        fn take_range(&mut self, range) {
            self.split_to(range.end)
        }
        #[inline]
        fn remove_range(&mut self, range) {
            self.advance(range.end)
        }
    }
    <RangeTo<usize>> for BytesMut {
        #[inline]
        fn take_range(&mut self, range) {
            self.split_to(range.end)
        }
        #[inline]
        fn remove_range(&mut self, range) {
            self.advance(range.end)
        }
    }
    <RangeToInclusive<usize>> for Bytes {
        #[inline]
        fn take_range(&mut self, range) {
            self.take_range(mem::convert_inclusive_range(range))
        }
        #[inline]
        fn remove_range(&mut self, range) {
            self.remove_range(mem::convert_inclusive_range(range))
        }
    }
    <RangeToInclusive<usize>> for BytesMut {
        #[inline]
        fn take_range(&mut self, range) {
            self.take_range(mem::convert_inclusive_range(range))
        }
        #[inline]
        fn remove_range(&mut self, range) {
            self.remove_range(mem::convert_inclusive_range(range))
        }
    }
}
