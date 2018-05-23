Three-valued logics
===================

[![ptal on Travis CI][travis-image]][travis]

[travis-image]: https://travis-ci.org/ptal/trilean.png
[travis]: https://travis-ci.org/ptal/trilean

This library provides the strong Kleene's three-valued logic which contains the boolean values `true` and `false` and an additional `unknown` variant.
The semantics of `unknown` usually means that the current value is neither true or false yet, but might evolve to one or the other later.
In particular, we have `false && unknown` equals to `true`, although the second value is `unknown`, we can evaluate the full expression.

Truth tables are available in the [documentation](https://docs.rs/trilean/).

This library compiles on stable Rust.

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.

## References

M. Fitting, “[Kleene’s three valued logics and their children](https://www.researchgate.net/publication/220444085_Kleene's_Three_Valued_Logics_and_Their_Children),” Fundamenta informaticae, vol. 20, no. 1, 2, 3, pp. 113–131, 1994.
