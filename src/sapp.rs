

use std::io::{self, Read, Write};

use hyper::{Get, Post, StatusCode, RequestUri, Decoder, Encoder, Next};
use hyper::header::ContentLength;
use hyper::net::HttpStream;

use hyper::server::Handler as HyperHandler;
use hyper::server::Request as HyperRequest;
use hyper::server::Response as HyperResponse;

use std::result::Result as StdResult;
use std::error::Error as StdError;


mod request;
use request::Request;
mod response;
use response::Response;
use router::Router;
use srouter::SRouter;


pub enum Error {
    BeforeError,
    HandlerError,
    AfterError,
    RouterConfigError
}

pub type Result<T> = ::std::result::Result<T, Error>; 


pub trait SModule {
    fn before(&self, &mut Request) -> Result<()>;
    
    fn after(&self, &Request, &mut Response) -> Result<()>;
    
    // here add routers ....
    fn router(&self, &mut SRouter) -> Result<()>;
    
}

pub trait SAppWrapper {
    fn before(&mut Request) -> Result<()>;
    
    fn after(&Request, &mut Response) -> Result<()>;
    
}

// later will add more fields
#[derive(Debug, Clone)]
pub struct SApp {
    // router, keep the original handler function
    pub router: SRouter,
    // wrapped router, keep the wrapped handler function
    // for actually use to recognize
    pub router_wrap: Router,
    // response deliver
    pub response: Option<Response>,
}

impl<T> SApp<T: SModule> {
    pub fn new() -> SApp {
        SApp {
            router: SRouter::new(),
            router_wrap: Router::new(),
            response: None,
        }
    }
    
    // add methods of this smodule
    // prefix:  such as '/user'
    pub fn add_smodule(&mut self, sm: T) -> &mut self {
        
        // get the sm router
        // pass self.router in
        sm.router(&mut self.router);
        // combile this router to global big router
        // create a new closure, containing 
        //      0. execute sapp.before();
        //      1. execute sm.before();
        //      2. execute a_router map pair value part function;
        //      3. execute sm.after();
        //      4. execute sapp.after();
        // fill the self.router_wrap finally
        // assign this new closure to the router_wrap router map pair  prefix + url part 
        
        for method, handler_vec in &self.router {
            // add to wrapped router
            handler_vec.map(|(&glob, &handler)|{
                self.router_wrap.route(*method, *glob, *handler);
            })
        }
        
        self
    }
}


impl HyperHandler<HttpStream> for SApp {
    fn on_request(&mut self, req: HyperRequest) -> Next {
        match *req.uri() {
            RequestUri::AbsolutePath(ref path) =>  {
                
                let path = &path[..];
                // make swiftrs request from hyper request
                let mut sreq = Request::new(req, path);
                
                // XXX: Need more work
                self.response = self.router_wrap.handle_method(sreq, &path).unwrap().ok();
                // self.router_wrap.handle_method(sreq, &path).unwrap_or_else(||
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
                );
                
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
            Some(response) => {
                // here, set hyper response status code, and headers
                res.headers_mut().set(ContentLength("it do.".len() as u64));
                Next::write()
            },
            None => {
                // Inner Error
                // end
                Next::end()
            }
        }
        
        
    }

    fn on_response_writable(&mut self, transport: &mut Encoder<HttpStream>) -> Next {
        match self.response {
            Some(response) => {
                // write response.body.unwrap() to transport
                transport.write("it do.")
                Next::end()
            },
            None => {
                // end
                Next::end()
            }
        }
       
    }
}
