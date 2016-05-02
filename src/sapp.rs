
use std::str;
use std::io::{self, Read, Write};

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


pub use typemap::Key;
pub use hyper::header::Headers;
pub use request::Request;
pub use response::Response;
pub use router::Router;
pub use srouter::SRouter;
pub use shandler::SHandler;

#[derive(Debug, PartialEq)]
pub enum Error {
    BeforeError,
    HandlerError,
    AfterError,
    RouterConfigError,
    RedirectError,
    NotFoundError,
}

pub type Result<T> = ::std::result::Result<T, Error>; 

#[derive(Clone)]
pub struct ReqPathParams;

pub trait SModule {
    fn before(&self, &mut Request) -> Result<()>;
    
    fn after(&self, &Request, &mut Response) -> Result<()>;
    
    // here add routers ....
    fn router(&self, &mut SRouter) -> Result<()>;
    // fn router(&self, SRouter) -> Result<SRouter>;
    
}

pub trait SAppWrapper {
    fn before(&self, &mut Request) -> Result<()>;
    
    fn after(&self, &Request, &mut Response) -> Result<()>;
    
}

// later will add more fields
pub struct SApp<T: SModule + Send + 'static, W: SAppWrapper + Send + 'static> {
    pub address: String,
    pub port:    u32,
    // for app entry, global middeware
    pub wrapper: Option<W>,
    // for actually use to recognize
    pub routers: Router,
    // marker for type T
    pub _marker: PhantomData<T>,
}



impl<T, W> SApp<T, W>
    where   T: SModule + Send + Sync + Reflect + Clone + 'static, 
            W: SAppWrapper + Send + Sync + Reflect + Clone + 'static 
    {
    pub fn new() -> SApp<T, W> {
        SApp {
            address: String::new(),
            port: 0,
            wrapper: None,
            routers: Router::new(),
            _marker: PhantomData
        }
    }
    
    pub fn run(self) {
        
        let listen_addr = self.address.clone() + ":" + &self.port.to_string();
        let arc_sapp = Arc::new(Box::new(self));
        
        let server = Server::http(&listen_addr.parse().unwrap()).unwrap();
        let _guard = server.handle(move |_| {
            RequestHandler::new(arc_sapp.clone())
        });
    }
    
    pub fn with_wrapper(&mut self, w: W) -> &mut Self {
        self.wrapper = Some(w);
        self
    }
    
    pub fn address(&mut self, address: &str) -> &mut Self {
        self.address = address.to_owned();
        self
    }
    
    pub fn port(&mut self, port: u32) -> &mut Self {
        self.port = port;
        self
    }
    
    
    // add methods of this smodule
    // prefix:  such as '/user'
    pub fn add_module(&mut self, sm: T) -> &mut Self {
        
        let mut router = SRouter::new();
        // get the sm router
        // pass self.router in
        sm.router(&mut router).unwrap();
        // combile this router to global big router
        // create a new closure, containing 
        //      0. execute sapp.before();
        //      1. execute sm.before();
        //      2. execute a_router map pair value part function;
        //      3. execute sm.after();
        //      4. execute sapp.after();
        // fill the self.routers finally
        // assign this new closure to the routers router map pair  prefix + url part 
        
        for (method, handler_vec) in router.into_router() {
            // add to wrapped router
            for &(glob, ref handler) in handler_vec.iter() {
                let method = method.clone();
                let glob = glob.clone();
                let handler = handler.clone();
                let sm = sm.clone();
                let wrapper = self.wrapper.clone().unwrap();
                self.routers.route(method, glob, Arc::new(Box::new(move |req: &mut Request| -> Result<Response> {
                    wrapper.before(req).unwrap();
                    sm.before(req).unwrap();
                    let res: Result<Response> = handler.handle(req);
                    let mut response = res.unwrap();
                    sm.after(req, &mut response).unwrap();
                    wrapper.after(req, &mut response).unwrap();
                    Ok(response)
                })));
            }
        }
        
        // self.modules.push(sm);
        
        self
    }
}


pub struct RequestHandler<
        T: SModule + Send + Sync + Reflect + Clone + 'static, 
        W: SAppWrapper + Send + Sync + Reflect + Clone + 'static> {
    // router, keep the original handler function
    // pub router: SRouter,
    // wrapped router, keep the wrapped handler function
    // for actually use to recognize
    pub sapp: Arc<Box<SApp<T, W>>>,
    pub path: String,
    pub method: Method,
    pub version: HttpVersion,
    pub headers: Headers,
    pub buf: Vec<u8>,
    pub body: String,
    pub has_body: bool,
    // response deliver
    pub response: Option<Response>,
}

impl<T, W> RequestHandler<T, W> 
where   T: SModule + Send + Sync + Reflect + Clone + 'static, 
        W: SAppWrapper + Send + Sync + Reflect + Clone + 'static
{
    pub fn new(sapp: Arc<Box<SApp<T, W>>>) -> RequestHandler<T, W> {
        RequestHandler {
            sapp: sapp,
            path: String::new(),
            method: Default::default(),
            version: Default::default(),
            headers: Default::default(),
            buf: vec![0; 2048],
            body: String::new(),
            has_body: false,
            response: None
        }
    }
    
}


impl<T, W> HyperHandler<HttpStream> for RequestHandler<T, W>
where   T: SModule + Send + Sync + Reflect + Clone + 'static, 
        W: SAppWrapper + Send + Sync + Reflect + Clone + 'static
 {
    fn on_request(&mut self, req: HyperRequest) -> Next {
        match *req.uri() {
            RequestUri::AbsolutePath(ref path) =>  {
                // if has_body
                if req.headers().get::<ContentLength>().is_some()
                    || req.headers().get::<ContentType>().is_some() 
                 {
                    self.path = path.to_owned();
                    self.method = req.method().clone();
                    self.version = req.version().clone();
                    self.headers = req.headers().clone();
                    self.has_body = true;
                    Next::read_and_write()
                } 
                else {
                    // if no body
                    let pathstr = &path[..];
                    let pathvec: Vec<&str> = pathstr.split('?').collect();
                    let path = pathvec[0].to_owned();
                    let mut query_string = None;
                    
                    // if has query_string
                    if pathvec.len() > 1 {
                        query_string = Some(pathvec[1].to_owned());
                    }
                    
                    // make swiftrs request from hyper request
                    let mut sreq = Request::new(
                        req.method().clone(),
                        req.version().clone(),
                        req.headers().clone(),
                        path.clone(),
                        query_string);
                        
                    match self.sapp.routers.handle_method(&mut sreq, &path).unwrap() {
                        Ok(response) => self.response = Some(response),
                        Err(e) => {
                            if e == Error::NotFoundError {
                                self.response = None
                            }
                        }
                    }
                    
                    Next::write()
                } 
                
                // XXX: Need more work
                // self.response = self.routers.handle_method(&mut sreq, &path).unwrap().ok();
                
                // TODO: complete it later
                // .unwrap_or_else(||
                    // match req.method {
                    //     method::Options => Ok(self.handle_options(&path)),
                    //     // For HEAD, fall back to GET. Hyper ensures no response body is written.
                    //     method::Head => {
                    //         req.method = method::Get;
                    //         self.handle_method(req, &path).unwrap_or(
                    //             Err(IronError::new(NoRoute, status::NotFound))
                    //         )
                    //     }
                    //     _ => Err(IronError::new(NoRoute, status::NotFound))
                    // }
                // );
                // currently
                
                
                // if is_more {
                //     Next::read_and_write()
                // } else {
                //     Next::write()
                // }
            
                // Next::read_and_write()
                // Next::write()
            },
            _ => Next::write()
        }
    }
    fn on_request_readable(&mut self, transport: &mut Decoder<HttpStream>) -> Next {
        if self.has_body {
            match transport.read(&mut self.buf) {
                Ok(0) => {
                    debug!("Read 0, eof");
                    
                    // TODO: need optimize
                    let pathstr = &self.path[..];
                    let pathvec: Vec<&str> = pathstr.split('?').collect();
                    let path = pathvec[0].to_owned();
                    let mut query_string = None;
                    
                    // if has query_string
                    if pathvec.len() > 1 {
                        query_string = Some(pathvec[1].to_owned());
                    }
                    let mut sreq = Request::new(
                        self.method.clone(),
                        self.version.clone(),
                        self.headers.clone(),
                        path.clone(),
                        query_string);
                        
                    sreq.set_raw_body(self.body.clone());
                        
                    match self.sapp.routers.handle_method(&mut sreq, &path).unwrap() {
                        Ok(response) => self.response = Some(response),
                        Err(e) => {
                            if e == Error::NotFoundError {
                                self.response = None
                            }
                        }
                    }
                    // 
                    return Next::write()
                },
                Ok(n) => {
                    self.body.push_str(str::from_utf8(&self.buf[0..n]).unwrap());
                    return Next::read_and_write()
                }
                Err(e) => match e.kind() {
                    io::ErrorKind::WouldBlock => return Next::read_and_write(),
                    _ => {
                        println!("read error {:?}", e);
                        return Next::end()
                    }
                }
            }
        }
        
        Next::write()
    }

    fn on_response(&mut self, res: &mut HyperResponse) -> Next {
        match self.response {
            Some(ref response) => {
                if let &Some(ref body) = response.body() {
                    // here, set hyper response status code, and headers
                    res.headers_mut().set(ContentLength(body.len() as u64));
                }
                Next::write()
            },
            None => {
                // Inner Error
                // end
                res.set_status(StatusCode::NotFound);
                res.headers_mut().set(ContentLength("404 Not Found".len() as u64));
                Next::write()
            }
        }
        
        
    }

    fn on_response_writable(&mut self, transport: &mut Encoder<HttpStream>) -> Next {
        match self.response {
            Some(ref response) => {
                if let &Some(ref body) = response.body() {
                    // write response.body.unwrap() to transport
                    transport.write(body.as_bytes()).unwrap();
                }
                Next::end()
            },
            None => {
                transport.write("404 Not Found".as_bytes()).unwrap();
                // end
                Next::end()
            }
        }
       
    }
}
