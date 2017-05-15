use std::str;
use std::io::{self, Read, Write};
use std::fs::File;
use std::path::Path;
use std::result::Result as StdResult;
use std::error::Error as StdError;
use std::sync::Arc;
use std::clone::Clone;

use hyper;
use hyper::{Get, Post, StatusCode};
use hyper::header::{ContentLength, ContentType};

use hyper::server::{Http, Service};
use hyper::server::Request as HyperRequest;
use hyper::server::Response as HyperResponse;
use hyper::Method;
use hyper::HttpVersion;

use futures;
use futures::future::FutureResult;
use mime_types::Types as MimeTypes;


pub use typemap::Key;
pub use hyper::header::Headers;
pub use hyper::header;
pub use hyper::mime;
pub use request::SapperRequest;
pub use response::SapperResponse;
pub use router_m::Router;
pub use router::SapperRouter;
pub use handler::SapperHandler;


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
    Fatal(String),
    Custom(String),
}
pub type Result<T> = ::std::result::Result<T, Error>; 



pub trait SapperModule: Sync + Send {
    fn before(&self, req: &mut SapperRequest) -> Result<()> {
        Ok(())
    }
    
    fn after(&self, req: &SapperRequest, res: &mut SapperResponse) -> Result<()> {
        Ok(())
    }
    
    fn router(&self, &mut SapperRouter) -> Result<()>;
    
}

pub trait SapperAppShell {
    fn before(&self, &mut SapperRequest) -> Result<()>;
    fn after(&self, &SapperRequest, &mut SapperResponse) -> Result<()>;
}

pub type GlobalInitClosure = Box<Fn(&mut SapperRequest) -> Result<()> + 'static + Send + Sync>;
pub type SapperAppShellType = Box<SapperAppShell + 'static + Send + Sync>;


pub struct SapperApp {
    pub address:        String,
    pub port:           u32,
    // for app entry, global middeware
    pub shell:          Option<Arc<SapperAppShellType>>,
    // for actually use to recognize
    pub routers:        Router,
    // do simple static file service
    pub static_service: bool,

    pub init_closure:   Option<Arc<GlobalInitClosure>>
}



impl SapperApp {
    pub fn new() -> SapperApp {
        SapperApp {
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
        self.shell = Some(Arc::new(w));
        self
    }
    
    pub fn init_global(&mut self, clos: GlobalInitClosure) -> &mut Self {
        self.init_closure = Some(Arc::new(clos));
        self
    }
    
    // add methods of this sapper module
    pub fn add_module(&mut self, sm: Box<SapperModule>) -> &mut Self {
        
        let mut router = SapperRouter::new();
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
                let shell = self.shell.clone();
                let init_closure = self.init_closure.clone();
                
                self.routers.route(method, glob, Arc::new(Box::new(move |req: &mut SapperRequest| -> Result<SapperResponse> {
                    if let Some(ref c) = init_closure {
                        c(req)?; 
                    }
                    if let Some(ref shell) = shell {
                        shell.before(req)?;
                    }
                    sm.before(req)?;
                    let mut response: SapperResponse = handler.handle(req)?;
                    sm.after(req, &mut response)?;
                    if let Some(ref shell) = shell {
                        shell.after(req, &mut response)?;
                    }
                    Ok(response)
                })));
            }
        }

        self
    }
    
    pub fn run_http(self) {
        
        let addr = self.address.clone() + ":" + &self.port.to_string();
        //let arc_sapp = Arc::new(Box::new(self));
        
        let self_box = Arc::new(Box::new(self));
        
        let server = Http::new().bind(
            &addr.parse().unwrap(), 
            move || Ok(self_box.clone())
        ).unwrap();
        server.run().unwrap();
    }
}


impl Service for SapperApp {
    type Request = HyperRequest;
    type Response = HyperResponse;
    type Error = hyper::Error;
    type Future = FutureResult<Self::Response, hyper::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {

        // make request from hyper request
//        let mut sreq = SapperRequest::new(&req);
        let mut sreq = SapperRequest::new(Box::new(req));

        // pass req to routers, execute matched biz handler
        let response_w = self.routers.handle_method(&mut sreq).unwrap();
        if response_w.is_err() {
            let path = sreq.path();
            if self.static_service {
                match simple_file_get(path) {
                    Ok((file_u8vec, file_mime)) => {
                        let mut response = Self::Response::new()
                            .with_header(ContentLength(file_u8vec.len() as u64));
                            
                        response.headers_mut().set_raw("Content-Type", vec![file_mime.as_bytes().to_vec()]);
                        response.set_body(file_u8vec);
                        
                        return futures::future::ok(response);
                    },
                    Err(_) => {
                        // return 404 NotFound now
                        let response = Self::Response::new()
                            .with_status(StatusCode::NotFound);
                            
                        return futures::future::ok(response);
                    }
                }
            }
        
            // return 404 NotFound now
            let response = Self::Response::new()
                .with_status(StatusCode::NotFound);
                
            return futures::future::ok(response);
        }
        
        let sres = response_w.unwrap();
        match sres.body_ref() {
            &Some(ref vec) => {
                // TODO: need to optimize for live time problem
                let tvec = vec.clone();
                // TODO: need copy all headers from SapperResponse
                let response = Self::Response::new()
                    .with_header(ContentLength(vec.len() as u64))
                    .with_body(tvec);

                // here, if can not match any router, we need check static file service
                // or response NotFound
                
                futures::future::ok(response)
            },
            &None => {
                let response = Self::Response::new()
                    .with_body("".as_bytes());

                // here, if can not match any router, we need check static file service
                // or response NotFound
                
                futures::future::ok(response)
            }
        }
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


