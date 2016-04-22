#![deny(warnings)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
extern crate hyper;
extern crate env_logger;
#[macro_use]
extern crate log;

mod biz;
use biz::Biz;
use sapp::SApp;
use sapp::SAppWrapper;

// must impl it
// total entry and exitice
impl SAppWrapper for SApp {
    fn before(&mut Request) -> Result<()> {
        
    }
    
    fn after(&Request, &mut Response) -> Result<()> {
        
    }
}


fn main() {
    env_logger::init().unwrap();
    
    let server = Server::http(&"127.0.0.1:1337".parse().unwrap()).unwrap();
    let _guard = server.handle(|_| {
        
        let mut sapp = SApp::new();
        // register modules
        sapp.add_smodule("/", Biz);
        
        sapp
        
    });
    println!("Listening on http://127.0.0.1:1337");
}
