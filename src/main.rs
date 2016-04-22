#![deny(warnings)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
extern crate hyper;
extern crate env_logger;
#[macro_use]
extern crate log;



fn main() {
    env_logger::init().unwrap();
    
    // mod biz;
    // use biz::Biz;
    // let m_biz = Biz;
    // sapp.add_smodule(m_biz);
    
    
    
    let server = Server::http(&"127.0.0.1:1337".parse().unwrap()).unwrap();
    let _guard = server.handle(|_| SApp::new());
    println!("Listening on http://127.0.0.1:1337");
}
