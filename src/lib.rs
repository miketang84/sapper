#![deny(warnings)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(reflect_marker)]
#![feature(question_mark)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate hyper;
extern crate route_recognizer as recognizer;
extern crate typemap;
extern crate conduit_mime_types as mime_types;

mod request;
mod response;
mod shandler;
mod router;
mod srouter;
mod sapp;


pub use sapp::SApp;
pub use sapp::SAppWrapper;
pub use sapp::Request;
pub use sapp::Response;
pub use sapp::SModule;
pub use sapp::SHandler;
pub use sapp::SRouter;
pub use sapp::RequestHandler;
pub use sapp::Result;
pub use sapp::Error;
pub use sapp::Key;
pub use sapp::header;
pub use sapp::status;
pub use sapp::mime;
