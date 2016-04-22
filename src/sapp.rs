

use std::io::{self, Read, Write};

use hyper::{Get, Post, StatusCode, RequestUri, Decoder, Encoder, Next};
use hyper::header::ContentLength;
use hyper::net::HttpStream;
use hyper::server::Server;
use hyper::server::Handler as HyperHandler;
use hyper::server::Request as HyperRequest;
use hyper::server::Response as HyperResponse;



mod request;
use request::Request;
mod response;
use response::Response;



pub enum Error {
    BeforeError,
    HandlerError,
    AfterError
}

pub type Result<T> = ::std::result::Result<T, Error>; 




// all handler function in each module should fit this Handler trait
trait SHandler {
    fn handle(&self, req: &mut Request) -> Result<Response>;
}


trait SModule {
    fn before(&mut Request) -> Result<()>;
    
    fn after(&Request, &mut Response) -> Result<()>;
    
    // here add routers ....
    fn router() -> Router {
        // need to use Router struct here
        
    }
    
    // set the module prefix path
    // fn prefix(&mut self) -> &mut self;
    // add url router to this module, can chain
    // fn add_route(&mut self) -> &mut self;
}


// later will add more fields
pub struct SApp<T: SModule> {
    // got it or not
    pub route: Route,
    // response deliver
    pub response: Option<Response>,
    // module vector, no need
    // pub modules: Vec<T>,
    // routers, keep one big router table
    pub router: Router,
    
    // tmp for test
    pub res_str: String
}

impl<T> SApp<T: SModule> {
    pub fn new() -> SApp {
        SApp {
            route: Route::NotFound,
            response: None,
            router: Router::new(),
            // for test
            res_str: "".to_string()
        }
    }
    
    pub fn hello (&mut self) -> String {
        let res_str = "hello swift rs.";
        res_str.to_string()
    }
    
    // add methods of this smodule
    // prefix:  such as '/user'
    pub fn add_smodule(&mut self, prefix: &str,  sm: T) -> &mut self {
        
        // get the sm router
        sm.router(&mut self.router, prefix);
        // combile this router to global big router
        // create a new closure, containing 
        //      1. execute sm.before();
        //      2. execute a_router map pair value part function;
        //      3. execute sm.after();
        // assign this new closure to the router map pair  prefix + url part 
        
        self
        
    }
    
    
}


impl HyperHandler<HttpStream> for SApp {
    fn on_request(&mut self, req: HyperRequest) -> Next {
        match *req.uri() {
            RequestUri::AbsolutePath(ref path) => match req.method() {
                &Get => {
                    info!("GET Got");
                    self.route = Route::Got;
                    
                    // make swiftrs request from hyper request
                    let mut sreq = Request::new(req, &path[..]);
                    
                    // Need more work
                    self.router.handle_method(req, &path).unwrap_or_else(||
                        match req.method {
                            method::Options => Ok(self.handle_options(&path)),
                            // For HEAD, fall back to GET. Hyper ensures no response body is written.
                            method::Head => {
                                req.method = method::Get;
                                self.handle_method(req, &path).unwrap_or(
                                    Err(IronError::new(NoRoute, status::NotFound))
                                )
                            }
                            _ => Err(IronError::new(NoRoute, status::NotFound))
                        }
                    );
                    
                    // find target handler in router collection
                    
                    // if find  
                    // move this sreq to handler, executeit, return swiftrs Response 
                    // else 
                    // return NotFound
                    
                    // self.response = Some(response);
                    
                    // if request method is Get, no need to read req body, ready to response
                    Next::write()
                }
                
                // (&Post, "/echo") => {
                //     info!("POST Echo");
                //     let mut is_more = true;
                //     self.route = if let Some(len) = req.headers().get::<ContentLength>() {
                //         is_more = **len > 0;
                //         Route::Echo(Body::Len(**len))
                //     } else {
                //         Route::Echo(Body::Chunked)
                //     };
                //     if is_more {
                //         Next::read_and_write()
                //     } else {
                //         Next::write()
                //     }
                // }
                
                _ => Next::write(),
            },
            _ => Next::write()
        }
    }
    fn on_request_readable(&mut self, transport: &mut Decoder<HttpStream>) -> Next {
        // match self.route {
        //     // Route::Echo(ref body) => {
        //     //     if self.read_pos < self.buf.len() {
        //     //         match transport.read(&mut self.buf[self.read_pos..]) {
        //     //             Ok(0) => {
        //     //                 debug!("Read 0, eof");
        //     //                 self.eof = true;
        //     //                 Next::write()
        //     //             },
        //     //             Ok(n) => {
        //     //                 self.read_pos += n;
        //     //                 match *body {
        //     //                     Body::Len(max) if max <= self.read_pos as u64 => {
        //     //                         self.eof = true;
        //     //                         Next::write()
        //     //                     },
        //     //                     _ => Next::read_and_write()
        //     //                 }
        //     //             }
        //     //             Err(e) => match e.kind() {
        //     //                 io::ErrorKind::WouldBlock => Next::read_and_write(),
        //     //                 _ => {
        //     //                     println!("read error {:?}", e);
        //     //                     Next::end()
        //     //                 }
        //     //             }
        //     //         }
        //     //     } else {
        //     //         Next::write()
        //     //     }
        //     // }
        //     _ => unreachable!()
        // }
        
        Next::write()
    }

    fn on_response(&mut self, res: &mut HyperResponse) -> Next {
        match self.response {
            Some(response) => {
                // here, set hyper response status code, and headers
                
            },
            None => {
                // Inner Error
                // end
            }
        }
        
        // match self.route {
        //     Route::NotFound => {
        //         res.set_status(StatusCode::NotFound);
        //         Next::end()
        //     }
        //     Route::Got => {
        //         let res_str = self.hello();
        //         self.res_str = res_str;
        //         res.headers_mut().set(ContentLength(self.res_str.len() as u64));
        //         Next::write()
        //     }
        //     // Route::Echo(body) => {
        //     //     if let Body::Len(len) = body {
        //     //         res.headers_mut().set(ContentLength(len));
        //     //     }
        //     //     Next::read_and_write()
        //     // }
        // }
    }

    fn on_response_writable(&mut self, transport: &mut Encoder<HttpStream>) -> Next {
        match self.response {
            Some(response) => {
                // write response.body.unwrap() to transport
            },
            None => {
                // end
            }
        }
        
        // match self.route {
        //     Route::Got => {
        //         transport.write(self.res_str.as_bytes()).unwrap();
        //         Next::end()
        //     }
        //     // Route::Echo(..) => {
        //     //     if self.write_pos < self.read_pos {
        //     //         match transport.write(&self.buf[self.write_pos..self.read_pos]) {
        //     //             Ok(0) => panic!("write ZERO"),
        //     //             Ok(n) => {
        //     //                 self.write_pos += n;
        //     //                 Next::write()
        //     //             }
        //     //             Err(e) => match e.kind() {
        //     //                 io::ErrorKind::WouldBlock => Next::write(),
        //     //                 _ => {
        //     //                     println!("write error {:?}", e);
        //     //                     Next::end()
        //     //                 }
        //     //             }
        //     //         }
        //     //     } else if !self.eof {
        //     //         Next::read()
        //     //     } else {
        //     //         Next::end()
        //     //     }
        //     // }
        //     _ => unreachable!()
        // }
    }
}
