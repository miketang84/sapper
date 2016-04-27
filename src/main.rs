#![deny(warnings)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(reflect_marker)]
extern crate hyper;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate route_recognizer as recognizer;
extern crate typemap;


use std::sync::Arc;
use hyper::server::Server;

mod request;
mod response;
mod shandler;
mod router;
mod srouter;
mod sapp;


use sapp::SApp;
use sapp::SAppWrapper;
use sapp::Request;
use sapp::Response;
use sapp::Result;
use sapp::SModule;
use sapp::SHandler;
use sapp::RequestHandler;


mod biz;
use biz::Biz;


#[derive(Clone)]
struct MyApp;
// must impl it
// total entry and exitice
impl SAppWrapper for MyApp {
    fn before(&self, req: &mut Request) -> Result<()> {
        println!("{}", "in SAppWrapper before.");
        
        Ok(())
    }
    
    fn after(&self, req: &mut Request, res: &mut Response) -> Result<()> {
        println!("{}", "in SAppWrapper after.");
        
        Ok(())
    }
}



fn main() {
    env_logger::init().unwrap();
    
    let mut sapp = SApp::new();
    sapp.with_wrapper(MyApp)
        .add_smodule(Biz);
    
    let arc_sapp = Arc::new(Box::new(sapp));
    
    let server = Server::http(&"127.0.0.1:1337".parse().unwrap()).unwrap();
    let _guard = server.handle(move |_| {
        RequestHandler::new(arc_sapp.clone())
    });
    println!("Listening on http://127.0.0.1:1337");
}
