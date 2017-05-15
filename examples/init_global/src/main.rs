#![allow(unused_variables)]

extern crate sapper;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate typemap;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use sapper::{SapperApp, SapperAppShell, Request, Response, Result};
use typemap::Key;



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

pub struct FOO_Int;
impl Key for FOO_Int { type Value = Arc<Box<usize>>; }
pub struct FOO_HashMap;
impl Key for FOO_HashMap { type Value = HashMap<&'static str, &'static str>; }
pub struct FOO_Mutex;
impl Key for FOO_Mutex { type Value = Arc<Mutex<HashMap<&'static str, &'static str>>>; }


pub fn main() {
    env_logger::init().unwrap();
    
    let a_global = Arc::new(Box::new(1024));
    let mut a_hash = HashMap::new();
    a_hash.insert("it's", "me");
    
    let mut a_mutex = HashMap::new();
    a_mutex.insert("f1", "d1");
    let a_mutex = Arc::new(Mutex::new(a_mutex));
    
    
    let mut sapp = SapperApp::new();
    sapp.address("127.0.0.1")
        .port(1337)
        .init_global(Box::new(move |req: &mut Request| -> Result<()> {
            println!("in init_global {:?}", req.query());
            req.ext_mut().insert::<FOO_Int>(a_global.clone());
            req.ext_mut().insert::<FOO_HashMap>(a_hash.clone());
            req.ext_mut().insert::<FOO_Mutex>(a_mutex.clone());
            
            Ok(())
        }))
        .with_shell(Box::new(MyApp))
        .add_module(Box::new(Biz))
        .add_module(Box::new(Foo));
    
    println!("Listening on http://127.0.0.1:1337");
    sapp.run_http();
    
}

