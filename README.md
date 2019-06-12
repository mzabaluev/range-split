# Splitting Sequences with Range Syntax

This library crate provides a trait and support utilities adding
convenience methods for splitting sequences accordingly to a given range.

The primary use case for the `TakeRange` trait are data container libraries
such as `bytes`, where representations of data buffers can be efficiently
split into smaller parts, but the inherent methods for doing this tend to be
not very mnemonic.
Implementations of the trait parameterized with different range types
provide convenient polymorphism with the range syntax.

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `split-range` by you, shall be licensed as MIT, without any
additional terms or conditions.
