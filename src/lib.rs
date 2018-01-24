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
