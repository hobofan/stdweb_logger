//! ## Usage
//!
//! ```rust,norun
//! #[macro_use]
//! extern crate stdweb;
//! #[macro_use]
//! extern crate log;
//! extern crate hobofan_stdweb_logger as stdweb_logger;
//!
//! fn main() {
//!   stdweb::initialize();
//!   stdweb_logger::Logger::init_with_level(::log::LevelFilter::Info);
//!
//!   info!("Hello World!");
//! }
//! ```
#[macro_use]
extern crate stdweb;
extern crate log;

mod logger;

pub use logger::Logger;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
