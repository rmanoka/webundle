#![feature(specialization, existential_type)]

#[macro_use] pub mod seq;
pub mod resolver;
pub mod source;
pub mod deps;
pub mod errors;

pub use seq::Seq;
pub use resolver::Request;
pub use source::{Id, Source};
pub use deps::{Compiled, Bundled};
pub use errors::{Error, ErrorKind};
