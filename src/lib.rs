#![allow(unused_variables)]
#![allow(warnings)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate hyper;
extern crate typemap;
extern crate conduit_mime_types as mime_types;

mod request;
mod response;
mod handler;
mod recognizer;
mod router_m;
mod router;
mod app;

pub use app::SapperApp as App;
pub use app::SapperArmor as Armor;
pub use app::SapperRequest as Request;
pub use app::SapperResponse as Response;
pub use app::SapperModule as Module;
pub use app::SapperHandler as Handler;
pub use app::SapperRouter as Router;
pub use app::{Result, Error, Key};
pub use app::{header, status, mime};
/// PathParams is the parameter type referring the parameters collected in url
pub use app::PathParams;
/// reexport hyper's Client to sapper level
pub use app::Client;

pub use recognizer::Params;
