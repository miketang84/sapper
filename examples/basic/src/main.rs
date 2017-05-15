#![allow(unused_variables)]

extern crate sapper;
extern crate env_logger;
#[macro_use]
extern crate log;

use sapper::{SapperApp, SapperAppShell, Request, Response, Result};



mod biz;
use biz::Biz;
mod foo;
use foo::Foo;

#[derive(Clone)]
struct MyApp;
// must impl it
// total entry and exitice
impl SapperAppShell for MyApp {
    fn before(&self, req: &mut Request) -> Result<()> {
        println!("{}", "in SapperAppShell before.");
        
        Ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        println!("{}", "in SapperAppShell after.");
        
        Ok(())
    }
}



pub fn main() {
    env_logger::init().unwrap();
    
    // fn init_global(req: &mut Request) -> Result<()> {
    //     Ok(())
    // }
    
    let mut sapp = SapperApp::new();
    sapp.address("127.0.0.1")
        .port(1337)
        // .init_global(Box::new(init_global))
        .with_shell(Box::new(MyApp))
        .add_module(Box::new(Biz))
        .add_module(Box::new(Foo));
    
    println!("Listening on http://127.0.0.1:1337");
    sapp.run_http();
    
}

