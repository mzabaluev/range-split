/// Methods for splitting out part of a collection with a given range.
pub trait TakeRange<R> {
    /// The value returned by the `take_range` method, representing
    /// the extracted part of the collection.
    type Output;

    /// Splits off and returns part of the collection designated by
    /// the given range. The remaining part is left in `self` with indices
    /// adjusted after the removal.
    ///
    /// The range parameter typically has one of the standard range types
    /// constructed with [range expression][range-expr] syntax.
    ///
    /// [range-expr]: https://doc.rust-lang.org/reference/expressions/range-expr.html
    ///
    /// # Panics
    ///
    /// The implementation can panic if the range is not valid for the
    /// operation.
    fn take_range(&mut self, range: R) -> Self::Output;

    /// Removes items from the the collection as designated by
    /// the given range. The remaining part is left in `self` with indices
    /// adjusted after the removal.
    ///
    /// The range parameter typically has one of the standard range types
    /// constructed with [range expression][range-expr] syntax.
    ///
    /// The default implementation of this method calls `take_range` and
    /// drops the returned value. Implementors of the trait should consider
    /// a more efficient implementation, avoiding construction of an
    /// intermediate container.
    ///
    /// [range-expr]: https://doc.rust-lang.org/reference/expressions/range-expr.html
    ///
    /// # Panics
    ///
    /// The implementation can panic if the range is not valid for the
    /// operation.
    fn remove_range(&mut self, range: R) {
        self.take_range(range);
    }
}
