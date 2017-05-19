use std::str;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::clone::Clone;

use hyper;
use hyper::StatusCode;
use hyper::header::ContentLength;

use hyper::server::{Http, Service};
use hyper::server::Request as HyperRequest;
use hyper::server::Response as HyperResponse;

use futures;
use futures::Future;
use futures::future::{BoxFuture, FutureResult};

use mime_types::Types as MimeTypes;

pub use futures::future::{ok, err};
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
}


#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    NotFound(String),
    InvalidConfig,
    InvalidRouterConfig,
    FileNotExist,
    ShouldRedirect(String),
    Break(String),
    Fatal(String),
    Custom(String),
}

pub type StdResult<T> = ::std::result::Result<T, Error>; 
pub type Result<T> = FutureResult<T, Error>; 
pub type BoxFutureResult<T> = BoxFuture<T, Error>; 


pub trait SapperModule: Sync + Send {
    fn before(&self, req: &mut SapperRequest) -> Result<()> {
        ok(())
    }
    
    fn after(&self, req: &SapperRequest, res: &mut SapperResponse) -> Result<()> {
        ok(())
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
        sm.router(&mut router).wait().unwrap();
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
                        if c(req).wait().is_err() {
                            return err(Error::Break("Break in init closure.".to_owned()));
                        }
                    }
                    if let Some(ref shell) = shell {
                        //shell.before(req).wait().unwrap();
                        if shell.before(req).wait().is_err() {
                            return err(Error::Break("Break in shell before.".to_owned()));
                        }
                    }
                    if sm.before(req).wait().is_err() {
                        return err(Error::Break("Break in module before.".to_owned()));
                    }
                    let mut response: SapperResponse = match handler.handle(req).wait() {
                        Ok(res) => {
                            res
                        },
                        Err(_) => {
                            return err(Error::Break("Break in handler.".to_owned()));
                        }
                    };
                    
                    if sm.after(req, &mut response).wait().is_err() {
                        return err(Error::Break("Break in module after.".to_owned()));
                    }
                    if let Some(ref shell) = shell {
                        if shell.after(req, &mut response).wait().is_err() {
                            return err(Error::Break("Break in shell after.".to_owned()));
                        }
                    }
                    ok(response)
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
    type Future = BoxFuture<Self::Response, hyper::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {

        // make request from hyper request
//        let mut sreq = SapperRequest::new(&req);
        let mut sreq = SapperRequest::new(Box::new(req));

        // pass req to routers, execute matched biz handler
        self.routers.handle_method(&mut sreq).map(move |response_w| {
            response_w.map(move |sres| {
                
                
            
            });
        })
        
        
        let response_w = self.routers.handle_method(&mut sreq).wait().unwrap();
        if response_w.is_none() {
            let path = sreq.path();
            if self.static_service {
                match simple_file_get(path) {
                    Ok((file_u8vec, file_mime)) => {
                        let mut response = Self::Response::new()
                            .with_header(ContentLength(file_u8vec.len() as u64));
                            
                        response.headers_mut().set_raw("Content-Type", vec![file_mime.as_bytes().to_vec()]);
                        response.set_body(file_u8vec);
                        
                        return ok(response).boxed();
                    },
                    Err(_) => {
                        // return 404 NotFound now
                        let response = Self::Response::new()
                            .with_status(StatusCode::NotFound);
                            
                        return ok(response).boxed();
                    }
                }
            }
        
            // return 404 NotFound now
            let response = Self::Response::new()
                .with_status(StatusCode::NotFound);
                
            return ok(response).boxed();
        }
        
        let sres = response_w.unwrap();
        match sres.body() {
            &Some(ref vec) => {
                let mut response = Self::Response::new();
                
                for header in sres.headers().iter() {
                    response.headers_mut()
                        .set_raw(header.name().to_owned(), 
                            vec![header.value_string().as_bytes().to_vec()]);
                }
                
                response.headers_mut().set(ContentLength(vec.len() as u64));
                // TODO: need to optimize for live time problem
                let tvec = vec.clone();
                
                response.set_body(tvec);

                // here, if can not match any router, we need check static file service
                // or response NotFound
                
                ok(response).boxed()
            },
            &None => {
                let response = Self::Response::new()
                    .with_body("".as_bytes());

                // here, if can not match any router, we need check static file service
                // or response NotFound
                
                ok(response).boxed()
            }
        }
    }
}




// this is very expensive in time
// should make it as global 
lazy_static! {
    static ref MTYPES: MimeTypes = { MimeTypes::new().unwrap() };
}

fn simple_file_get(path: &str) -> StdResult<(Vec<u8>, String)> {
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


