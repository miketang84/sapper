
use std::str;
use std::io::{self, Read, Write};
use std::fs::File;
use std::path::Path;

use hyper::{Get, Post, StatusCode, RequestUri, Decoder, Encoder, Next};
use hyper::header::{ContentLength, ContentType};
use hyper::net::HttpStream;

use hyper::server::Server;
use hyper::server::Handler as HyperHandler;
use hyper::server::Request as HyperRequest;
use hyper::server::Response as HyperResponse;
use hyper::method::Method;
use hyper::version::HttpVersion;

use std::result::Result as StdResult;
use std::error::Error as StdError;
use std::sync::Arc;
use std::marker::Reflect;
use std::clone::Clone;
use std::marker::PhantomData;


use mime_types::Types as MimeTypes;

pub use typemap::Key;
pub use hyper::header::Headers;
pub use hyper::header;
pub use hyper::mime;
pub use request::Request;
pub use response::Response;
pub use router::Router;
pub use srouter::SRouter;
pub use shandler::SHandler;


////////////////////////
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;

use futures::future::FutureResult;

use hyper::{Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};
////////////////////////


#[derive(Clone)]
pub struct PathParams;


/// Status Codes
pub mod status {
    pub use hyper::status::StatusCode as Status;
    pub use hyper::status::StatusCode::*;
    pub use hyper::status::StatusClass;
}


#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    NotFound(String),
    InvalidConfig,
    InvalidRouterConfig,
    FileNotExist,
    ShouldRedirect(String),
    Prompt(String),
    Warning(String),
    Fatal(String),
    Custom(String),
}

pub type Result<T> = ::std::result::Result<T, Error>; 



pub trait SapperModule: Sync + Send {
    fn before(&self, req: &mut Request) -> Result<()> {
        Ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        Ok(())
    }
    
    // here add routers ....
    fn router(&self, &mut SRouter) -> Result<()>;
    // fn router(&self, SRouter) -> Result<SRouter>;
    
}

pub trait SapperAppShell {
    fn before(&self, &mut Request) -> Result<()>;
    
    fn after(&self, &Request, &mut Response) -> Result<()>;
    
}

pub type GlobalInitClosure = Box<Fn(&mut Request) -> Result<()> + 'static + Send + Sync>;
pub type SapperAppShellType = Box<SapperAppShell + 'static + Send + Sync>;


#[derive(Clone, Copy)]
pub struct SapperApp {
    pub address: String,
    pub port:    u32,
    // for app entry, global middeware
    pub wrapper: Option<Arc<SAppWrapperType>>,
    // for actually use to recognize
    pub routers: Router,
    // do simple static file service
    pub static_service: bool,

    pub init_closure: Option<Arc<GlobalInitClosure>>
}



impl SapperApp {
    pub fn new() -> SApp {
        SApp {
            address: String::new(),
            port: 0,
            shell: None,
            routers: Router::new(),
            static_service: true,
            init_closure: None
        }
    }
    
    pub fn address(&mut self, address: &str) -> &mut Self {
        self.address = address.to_owned();
        self
    }
    
    pub fn port(&mut self, port: u32) -> &mut Self {
        self.port = port;
        self
    }
    
    pub fn static_service(&mut self, open: bool) -> &mut Self {
        self.static_service = open;
        self
    }

    pub fn with_shell(&mut self, w: SapperAppShellType) -> &mut Self {
        self.wrapper = Some(Arc::new(w));
        self
    }
    
    pub fn init_global(&mut self, clos: GlobalInitClosure) -> &mut Self {
        self.init_closure = Some(Arc::new(clos));
        self
    }
    
    // add methods of this sapper module
    pub fn add_module(&mut self, sm: Box<SModule>) -> &mut Self {
        
        let mut router = SRouter::new();
        // get the sm router
        sm.router(&mut router).unwrap();
        let sm = Arc::new(sm);
        
        for (method, handler_vec) in router.into_router() {
            // add to wrapped router
            for &(glob, ref handler) in handler_vec.iter() {
                let method = method.clone();
                let glob = glob.clone();
                let handler = handler.clone();
                let sm = sm.clone();
                // let sm = Box::new(sm);
                let wrapper = self.wrapper.clone();
                let init_closure = self.init_closure.clone();
                self.routers.route(method, glob, Arc::new(Box::new(move |req: &mut Request| -> Result<Response> {
                    // if init_closure.is_some() {
                    //     init_closure.unwrap()(req)?;
                    // }
                    if let Some(ref c) = init_closure {
                        c(req)?; 
                    }
                    if let Some(ref wrapper) = wrapper {
                        wrapper.before(req)?;
                    }
                    sm.before(req)?;
                    let mut response: Response = handler.handle(req)?;
                    sm.after(req, &mut response)?;
                    if let Some(ref wrapper) = wrapper {
                        wrapper.after(req, &mut response)?;
                    }
                    Ok(response)
                })));
            }
        }
        
        // self.modules.push(sm);
        
        self
    }
    
    
    pub fn run_http(self) {
        
        let addr = self.address.clone() + ":" + &self.port.to_string();
        //let arc_sapp = Arc::new(Box::new(self));
        
        let server = Http::new().bind(&addr, || Ok(self)).unwrap();
        server.run().unwrap();
        
    }
}



impl Service for SapperApp {
    type Request = HyperRequest;
    type Response = HyperResponse;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        
        // make request from hyper request
        let mut sreq = SapperRequest::new(&req);

        // pass req to routers, execute matched biz handler
        let response = self.sapp.routers.handle_method(&mut sreq).unwrap();
        
        // here, if can not match any router, we need check static file service
        // or response NotFound
        
        futures::future::ok(response)
        
    }
}




// this is very expensive in time
// should make it as global 
lazy_static! {
    static ref MTYPES: MimeTypes = { MimeTypes::new().unwrap() };
}

fn simple_file_get(path: &str) -> Result<(Vec<u8>, String)> {
    let new_path;
    if &path[(path.len()-1)..] == "/" {
        new_path = "static/".to_owned() + path + "index.html";
    }
    else {
        new_path = "static/".to_owned() + path;
    }
    //println!("file path: {}", new_path);
    match File::open(&new_path) {
        Ok(ref mut file) => {
            let mut s: Vec<u8> = vec![];
            file.read_to_end(&mut s).unwrap_or(0);
            
            let mt_str = MTYPES.mime_for_path(Path::new(&new_path));
            
            Ok((s, mt_str.to_owned()))
        },
        Err(_) => Err(Error::FileNotExist)
    }
}


