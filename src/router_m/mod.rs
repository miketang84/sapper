#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

mod router;


pub use router::router::Router;
// pub use router::router::NoRoute;
pub use recognizer::Params;

