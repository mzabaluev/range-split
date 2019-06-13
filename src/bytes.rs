use crate::{mem, TakeRange};

use bytes::{Bytes, BytesMut};

use std::ops::{RangeFrom, RangeFull, RangeTo, RangeToInclusive};

macro_rules! take_range_method {
    {
        fn take_range(&mut $self:ident, $range:ident: $Range:ty)
        $body:block
    } => {
        #[inline]
        fn take_range(&mut $self, $range: $Range) -> Self::Output
        $body
    };
    {
        fn remove_range(&mut $self:ident, $range:ident: $Range:ty)
        $body:block
    } => {
        #[inline]
        fn remove_range(&mut $self, $range: $Range)
        $body
    };
}

macro_rules! impl_take_range {
    {
        $(
            <$Range:ty> for $T:ty {
                $(
                    fn $method:ident(&mut $self:ident, $range:ident)
                    $body:block
                )*
            }
        )*
    } => {
        $(
            impl TakeRange<$Range> for $T {
                type Output = $T;

                $(
                    take_range_method! {
                        fn $method(&mut $self, $range: $Range)
                        $body
                    }
                )*
            }
        )*
    };
}

impl_take_range! {
    <RangeFull> for Bytes {
        fn take_range(&mut self, _range) {
            self.split_off(0)
        }
        fn remove_range(&mut self, _range) {
            self.clear()
        }
    }
    <RangeFull> for BytesMut {
        fn take_range(&mut self, _range) {
            self.take()
        }
        fn remove_range(&mut self, _range) {
            self.clear()
        }
    }
    <RangeFrom<usize>> for Bytes {
        fn take_range(&mut self, range) {
            self.split_off(range.start)
        }
        fn remove_range(&mut self, range) {
            self.truncate(range.start)
        }
    }
    <RangeFrom<usize>> for BytesMut {
        fn take_range(&mut self, range) {
            self.split_off(range.start)
        }
        fn remove_range(&mut self, range) {
            self.truncate(range.start)
        }
    }
    <RangeTo<usize>> for Bytes {
        fn take_range(&mut self, range) {
            self.split_to(range.end)
        }
        fn remove_range(&mut self, range) {
            self.advance(range.end)
        }
    }
    <RangeTo<usize>> for BytesMut {
        fn take_range(&mut self, range) {
            self.split_to(range.end)
        }
        fn remove_range(&mut self, range) {
            self.advance(range.end)
        }
    }
    <RangeToInclusive<usize>> for Bytes {
        fn take_range(&mut self, range) {
            self.take_range(mem::convert_inclusive_range(range))
        }
        fn remove_range(&mut self, range) {
            self.remove_range(mem::convert_inclusive_range(range))
        }
    }
    <RangeToInclusive<usize>> for BytesMut {
        fn take_range(&mut self, range) {
            self.take_range(mem::convert_inclusive_range(range))
        }
        fn remove_range(&mut self, range) {
            self.remove_range(mem::convert_inclusive_range(range))
        }
    }
}
