

use std::io::{self, Read, Write};

use hyper::{Get, Post, StatusCode, RequestUri, Decoder, Encoder, Next};
use hyper::header::ContentLength;
use hyper::net::HttpStream;

use hyper::server::Server;
use hyper::server::Handler as HyperHandler;
use hyper::server::Request as HyperRequest;
use hyper::server::Response as HyperResponse;

use std::result::Result as StdResult;
use std::error::Error as StdError;
use std::sync::Arc;
use std::marker::Reflect;
use std::clone::Clone;
use std::marker::PhantomData;

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
pub struct PathParams;

pub trait SModule {
    fn before(&self, &mut Request) -> Result<()>;
    
    fn after(&self, &mut Request, &mut Response) -> Result<()>;
    
    // here add routers ....
    fn router(&self, &mut SRouter) -> Result<()>;
    // fn router(&self, SRouter) -> Result<SRouter>;
    
}

pub trait SAppWrapper {
    fn before(&self, &mut Request) -> Result<()>;
    
    fn after(&self, &mut Request, &mut Response) -> Result<()>;
    
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
    pub fn add_smodule(&mut self, sm: T) -> &mut Self {
        
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
        
        // println!("router length: {}", router.into_router().len());
        // let wrapper = self.wrapper.clone();
        
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
                
                let path = &path[..];
                // make swiftrs request from hyper request
                let mut sreq = Request::new(
                    req.method().clone(),
                    req.version().clone(),
                    req.headers().clone(),
                    path);
                
                // XXX: Need more work
                // self.response = self.routers.handle_method(&mut sreq, &path).unwrap().ok();
                match self.sapp.routers.handle_method(&mut sreq, &path).unwrap() {
                    Ok(response) => self.response = Some(response),
                    Err(e) => {
                        if e == Error::NotFoundError {
                            self.response = None
                        }
                    }
                    
                    
                }
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
                
                Next::write()
            },
            _ => Next::write()
        }
    }
    fn on_request_readable(&mut self, transport: &mut Decoder<HttpStream>) -> Next {
        
        
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
