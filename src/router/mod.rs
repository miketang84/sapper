#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! `Router` provides fast and flexible routing for swiftrs.
// extern crate route_recognizer as recognizer;
mod router;


pub use router::router::Router;
// pub use router::router::NoRoute;
pub use recognizer::Params;


// mod macros;
