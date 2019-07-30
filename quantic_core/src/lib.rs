#![warn(missing_docs, rust_2018_idioms, rust_2018_compatibility)]

extern crate chrono;
extern crate futures;
#[macro_use]
extern crate quantic_derive;

mod time;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
