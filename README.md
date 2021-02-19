# peter

[![Crates.io Version](https://img.shields.io/crates/v/peter.svg)](https://crates.io/crates/peter)
[![Docs.rs Latest](https://img.shields.io/badge/docs.rs-latest-blue.svg)](https://docs.rs/peter)

Peter builds on the [`ansi_term`] crate to allow styling of anything
implementing [`Display`] and makes colorizing text less verbose to use by
providing the [`Stylize`] trait.

[`ansi_term`]: https://github.com/ogham/rust-ansi-term
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`Stylize`]: https://docs.rs/peter/0.1/stride/trait.Stylize.html

## Getting started

```rust
use peter::Stylize;

println!("This is in red: {}", "a red string".red());

println!("How about some {}?", "bold and underline".bold().underline());
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
