//! Quantic is a free and open source quantitative finance library written in idiomatic [Rust][rs].
//! Quantic provides an assortment of basic building blocks that can be composed, chained and extended
//! to model a wide variety of financial instruments and markets.
//! 
//! This project is based heavily on the excellent [QuantLib][ql].
//! 
//! [rs]: https://www.rust-lang.org
//! [ql]: https://github.com/lballabio/QuantLib
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms, rust_2018_compatibility)]
#![warn(clippy::all)]
#![allow(clippy::new_without_default)]

extern crate quantic_core;
#[macro_use]
extern crate quantic_derive;

use quantic_core as core;
use quantic_derive as derive;

pub use crate::core::*;
pub use crate::derive::*;