# iron-try

> A handler for trying handlers in succession.

## Overview

For any tuple of handlers used in `Try`, `Try`s `Handler`
implementation will call each handler's `handle` method until
one returns an error-free `Response`.

The same procedure is done for `Try<Vec<Box<Handler>>>`.

## Usage

Use the crates.io repository; add this to your `Cargo.toml` along
with the rest of your dependencies:

```toml
[dependencies]
iron-try = "*"
```

## Author

[Jonathan Reem](https://medium.com/@jreem) is the primary author and
maintainer of iron-try.

## License

MIT

