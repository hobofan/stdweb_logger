## Crude logger implementation for the [log][crate_log] crate for use with [stdweb][crate_stdweb] - No long-term maintenance intended!

This crate allows you to use the usual log macros from the `log` crate (`info!`, `debug!`, etc.) when working on a project involving `stdweb`.

The logger will then call the Javascript equivalent in the browser. E.g. `info!("Hello World")` => `console.info("Hello World");`.

[crate_log]: https://crates.io/crates/log
[crate_stdweb]: https://crates.io/crates/stdweb

## Caveats

- Doesn't have module filtering (yet). If you turn the log level to `Debug` or `Trace` and your dependencies also log a lot, your console will be very full.
- Both `Debug` and `Trace` map to `console.debug()` since there is no `Trace` equivalent.

## Installation

Add this to your `Cargo.toml` dependencies:
```toml
hobofan_stdweb_logger = "0.1.1"
```

or run this if you have `cargo-edit` installed:
```bash
cargo add hobofan_stdweb_logger
```

## Usage

```rust
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate log;
extern crate hobofan_stdweb_logger as stdweb_logger;

fn main() {
  stdweb::initialize();
  stdweb_logger::Logger::init_with_level(::log::LevelFilter::Info);

  info!("Hello World!");
}
```

## License

Licensed under either of

  * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
  * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
