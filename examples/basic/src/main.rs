
extern crate sapper;
extern crate env_logger;
#[macro_use]
extern crate log;

use sapper::{SApp, SAppWrapper, Request, Response, Result, SModule};



mod biz;
use biz::Biz;
mod foo;
use foo::Foo;

#[derive(Clone)]
struct MyApp;
// must impl it
// total entry and exitice
impl SAppWrapper for MyApp {
    fn before(&self, req: &mut Request) -> Result<()> {
        println!("{}", "in SAppWrapper before.");
        
        Ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        println!("{}", "in SAppWrapper after.");
        
        Ok(())
    }
}



pub fn main() {
    env_logger::init().unwrap();
    
    let mut sapp = SApp::new();
    sapp.address("127.0.0.1")
        .port(1337)
        .with_wrapper(MyApp)
        .add_module(Box::new(Biz))
        .add_module(Box::new(Foo));
    
    println!("Listening on http://127.0.0.1:1337");
    sapp.run();
    
}
