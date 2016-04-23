#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! `Router` provides fast and flexible routing for swiftrs.
// extern crate route_recognizer as recognizer;

pub use router::{Router, NoRoute};
pub use recognizer::Params;

mod router;
// mod macros;
