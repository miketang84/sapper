

use std::io::{self, Read, Write};

use hyper::{Get, Post, StatusCode, RequestUri, Decoder, Encoder, Next};
use hyper::header::ContentLength;
use hyper::net::HttpStream;
use hyper::server::{Server, Handler, Request, Response};



mod response;
use response::Response;


// struct Echo {
//     buf: Vec<u8>,
//     read_pos: usize,
//     write_pos: usize,
//     eof: bool,
//     route: Route,
// }

// enum Route {
//     NotFound,
//     Got,
//     // Echo(Body),
// }

// #[derive(Clone, Copy)]
// enum Body {
//     Len(u64),
//     Chunked
// }

static INDEX: &'static [u8] = b"Welcome to swift-rs.";

// impl Echo {
//     fn new() -> Echo {
//         Echo {
//             buf: vec![0; 4096],
//             read_pos: 0,
//             write_pos: 0,
//             eof: false,
//             route: Route::NotFound,
//         }
//     }
// }

impl Handler<HttpStream> for SApp {
    fn on_request(&mut self, req: Request) -> Next {
        match *req.uri() {
            RequestUri::AbsolutePath(ref path) => match (req.method(), &path[..]) {
                (&Get, "/") | (&Get, "/echo") => {
                    info!("GET Got");
                    self.route = Route::Got;
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

    fn on_response(&mut self, res: &mut Response) -> Next {
        match self.route {
            Route::NotFound => {
                res.set_status(StatusCode::NotFound);
                Next::end()
            }
            Route::Got => {
                let res_str = self.hello();
                self.res_str = res_str;
                res.headers_mut().set(ContentLength(self.res_str.len() as u64));
                Next::write()
            }
            // Route::Echo(body) => {
            //     if let Body::Len(len) = body {
            //         res.headers_mut().set(ContentLength(len));
            //     }
            //     Next::read_and_write()
            // }
        }
    }

    fn on_response_writable(&mut self, transport: &mut Encoder<HttpStream>) -> Next {
        match self.route {
            Route::Got => {
                transport.write(self.res_str.as_bytes()).unwrap();
                Next::end()
            }
            // Route::Echo(..) => {
            //     if self.write_pos < self.read_pos {
            //         match transport.write(&self.buf[self.write_pos..self.read_pos]) {
            //             Ok(0) => panic!("write ZERO"),
            //             Ok(n) => {
            //                 self.write_pos += n;
            //                 Next::write()
            //             }
            //             Err(e) => match e.kind() {
            //                 io::ErrorKind::WouldBlock => Next::write(),
            //                 _ => {
            //                     println!("write error {:?}", e);
            //                     Next::end()
            //                 }
            //             }
            //         }
            //     } else if !self.eof {
            //         Next::read()
            //     } else {
            //         Next::end()
            //     }
            // }
            _ => unreachable!()
        }
    }
}


pub enum Route {
    NotFound,
    Got,
}

trait SModule {
    fn before(&mut Request) -> Result<(), SError>;
    
    fn after(&mut Response) -> Result<(), SError>;
    
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
    pub routers: RouterTable,
    // tmp for test
    pub res_str: String
}

impl SApp {
    pub fn new() -> SApp {
        SApp {
            route: Route::NotFound,
            response: None,
            modules: vec![],
            res_str: "".to_string()
        }
    }
    
    pub fn hello (&mut self) -> String {
        let res_str = "hello swift rs.";
        res_str.to_string()
    }
    
    // add methods of this smodule
    pub fn add_smodule<T: SModule>(&mut self, sm: T) -> &mut self {
        
        // get the sm router
        let a_router = sm.router();
        // combile this router to global big router
        // create a new closure, containing 
        //      1. execute sm.before();
        //      2. execute a_router map pair value part function;
        //      3. execute sm.after();
        // assign this new closure to the router map pair url part
        
        
        
    }
    
    
}

