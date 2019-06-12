pub trait TakeRange<R> {
    type Output;

    fn take_range(&mut self, range: R) -> Self::Output;

    fn remove_range(&mut self, range: R) {
        self.take_range(range);
    }
}
