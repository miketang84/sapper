#![allow(unused_variables)]

extern crate sapper;
extern crate env_logger;
#[macro_use]
extern crate log;

use sapper::SapperApp;



mod biz;
use biz::Biz;
mod foo;
use foo::Foo;



pub fn main() {
    env_logger::init().unwrap();
    
    let mut sapp = SapperApp::new();
    sapp.address("127.0.0.1")
        .port(1337)
        .add_module(Box::new(Biz))
        .add_module(Box::new(Foo));
    
    println!("Listening on http://127.0.0.1:1337");
    sapp.run_http();
    
}
