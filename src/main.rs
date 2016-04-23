#![deny(warnings)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
extern crate hyper;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate route_recognizer as recognizer;
extern crate typemap;

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




// must impl it
// total entry and exitice
impl<T: SModule + Send + 'static> SAppWrapper for SApp<T> {
    fn before(req: &mut Request) -> Result<()> {
        
        Ok(())
    }
    
    fn after(req: &Request, res: &mut Response) -> Result<()> {
        
        Ok(())
    }
}



mod biz;
use biz::Biz;


fn main() {
    env_logger::init().unwrap();
    
    let server = Server::http(&"127.0.0.1:1337".parse().unwrap()).unwrap();
    let _guard = server.handle(|_| {
        
        let mut sapp = SApp::new();
        // register modules
        sapp.add_smodule(Biz);
        
        sapp
        
    });
    println!("Listening on http://127.0.0.1:1337");
}
