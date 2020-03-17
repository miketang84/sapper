#![allow(unused_variables)]
#![allow(warnings)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate conduit_mime_types as mime_types;
extern crate hyper;
extern crate typemap;

mod app;
mod handler;
mod recognizer;
mod request;
mod response;
mod router;
mod router_m;

/// reexport hyper's Client to sapper level
pub use app::Client;
/// PathParams is the parameter type referring the parameters collected in url
pub use app::PathParams;
pub use app::SapperApp as App;
pub use app::SapperArmor as Armor;
pub use app::SapperHandler as Handler;
pub use app::SapperModule as Module;
pub use app::SapperRequest as Request;
pub use app::SapperResponse as Response;
pub use app::SapperRouter as Router;
pub use app::{header, mime, status};
pub use app::{Error, Key, Result};

pub use recognizer::Params;
