#![allow(unused_variables)]

#[allow(warnings)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate hyper;
extern crate typemap;
extern crate route_recognizer as recognizer;
extern crate conduit_mime_types as mime_types;
extern crate futures;

mod request;
mod response;
mod handler;
mod router_m;
mod router;
mod app;

pub use app::SapperApp;
pub use app::SapperAppShell;
pub use app::SapperRequest as Request;
pub use app::SapperResponse as Response;
pub use app::SapperModule;
pub use app::SapperHandler;
pub use app::SapperRouter;
pub use app::{Result, Error, Key};
pub use app::{header, status, mime};
pub use app::PathParams;

pub use hyper::Body;
pub use futures::{Future, Stream};
pub use futures::future::{ok, err};
pub use hyper::Error as HyperError;

