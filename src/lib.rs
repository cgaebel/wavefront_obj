//! Parsers for wavefront's `.obj` and `.mtl` file format for loading meshes.
#![crate_type = "lib"]
#![deny(unsafe_blocks)]
#![deny(warnings)]
#![deny(missing_docs)]
#![feature(collections, core)]

pub use lex::ParseError;

mod lex;
mod util;

pub mod mtl;
pub mod obj;
