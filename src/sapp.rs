
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

#[derive(Clone)]
pub struct ReqPathParams;

pub trait SModule: Sync + Send {
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
pub struct SApp<W: SAppWrapper + Send + 'static> {
    pub address: String,
    pub port:    u32,
    // for app entry, global middeware
    pub wrapper: Option<W>,
    // for actually use to recognize
    pub routers: Router,
    // do simple static file service?
    pub static_service: bool,
    // marker for type T
    // pub _marker: PhantomData<T>,
}



impl<W> SApp<W>
    where
            W: SAppWrapper + Send + Sync + Reflect + Clone 
    {
    pub fn new() -> SApp<W> {
        SApp {
            address: String::new(),
            port: 0,
            wrapper: None,
            routers: Router::new(),
            static_service: true,
            // _marker: PhantomData
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
    
    pub fn static_service(&mut self, open: bool) -> &mut Self {
        self.static_service = open;
        self
    }
    
    
    // add methods of this smodule
    // prefix:  such as '/user'
    pub fn add_module<'a>(&mut self, sm: Box<SModule>) -> &mut Self {
        
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
        
        let sm = Arc::new(sm);
        
        for (method, handler_vec) in router.into_router() {
            // add to wrapped router
            for &(glob, ref handler) in handler_vec.iter() {
                let method = method.clone();
                let glob = glob.clone();
                let handler = handler.clone();
                let sm = sm.clone();
                // let sm = Box::new(sm);
                let wrapper = self.wrapper.clone().unwrap();
                self.routers.route(method, glob, Arc::new(Box::new(move |req: &mut Request| -> Result<Response> {
                    wrapper.before(req)?;
                    sm.before(req)?;
                    let mut response: Response = handler.handle(req)?;
                    sm.after(req, &mut response)?;
                    wrapper.after(req, &mut response)?;
                    Ok(response)
                })));
            }
        }
        
        // self.modules.push(sm);
        
        self
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


pub struct RequestHandler<
        // T: SModule + Send + Sync + Reflect + Clone + 'static, 
        W: SAppWrapper + Send + Sync + Reflect + Clone + 'static> {
    // router, keep the original handler function
    // pub router: SRouter,
    // wrapped router, keep the wrapped handler function
    // for actually use to recognize
    pub sapp: Arc<Box<SApp<W>>>,
    pub path: String,
    pub method: Method,
    pub version: HttpVersion,
    pub headers: Headers,
    pub buf: Vec<u8>,
    pub body: String,
    pub has_body: bool,
    pub write_pos: usize,
    // response deliver
    pub response: Result<Response>,
    pub static_file: Option<Vec<u8>>,
}

impl<W> RequestHandler<W> 
where    
        W: SAppWrapper + Send + Sync + Reflect + Clone + 'static
{
    pub fn new(sapp: Arc<Box<SApp<W>>>) -> RequestHandler<W> {
        RequestHandler {
            sapp: sapp,
            path: String::new(),
            method: Default::default(),
            version: Default::default(),
            headers: Default::default(),
            buf: vec![0; 2048],
            body: String::new(),
            has_body: false,
            write_pos: 0,
            response: Err(Error::NotFound("/".to_owned())),
            static_file: None,
        }
    }
    
}


impl<W> HyperHandler<HttpStream> for RequestHandler<W>
where  
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

                    self.response = self.sapp.routers.handle_method(&mut sreq, &path).unwrap();
                        
                    // match self.sapp.routers.handle_method(&mut sreq, &path).unwrap() {
                    //     Ok(response) => self.response = Some(response),
                    //     Err(e) => {
                    //         if e == Error::NotFoundError {
                    //             self.response = None
                    //         }
                    //     }
                    // }
                    
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
                        
                    // TODO: optimize this memory copy
                    sreq.set_raw_body(self.body.clone());
                        
                    self.response = self.sapp.routers.handle_method(&mut sreq, &path).unwrap();
                    
                    // match self.sapp.routers.handle_method(&mut sreq, &path).unwrap() {
                    //     Ok(response) => self.response = Some(response),
                    //     Err(e) => {
                    //         if e == Error::NotFoundError {
                    //             self.response = None
                    //         }
                    //     }
                    // }
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
            Ok(ref response) => {
                if let &Some(ref body) = response.body() {
                    // default set content type as html
                    res.headers_mut().set(ContentType::html());
                    
                    // update top level headers to low level headers
                    for header in response.headers().iter() {
                        res.headers_mut()
                            .set_raw(header.name().to_owned(), 
                                vec![header.value_string().as_bytes().to_vec()]);
                    }
                    
                    // here, set hyper response status code, and headers
                    res.headers_mut().set(ContentLength(body.len() as u64));
                }

                Next::write()
            },
            Err(ref e) => {
                // Inner Error
                // end
                match e {
                    &Error::NotFound(ref path) => {

                        if self.sapp.static_service {
                            match simple_file_get(path) {
                                Ok((avec, mt_str)) => {
                                    println!("serve file: {}", path);
                                    let body_len = avec.len() as u64;
                                    self.static_file = Some(avec);
                                    // TODO: need jude file mime type according to path
                                    // and set the header
                                    // let mt_str = MTYPES.mime_for_path(Path::new(path));
                                    res.headers_mut().set_raw("Content-Type", vec![mt_str.as_bytes().to_vec()]);
                                    res.headers_mut().set(ContentLength(body_len));
                                },
                                Err(_) => {
                                    println!("NotFound: {}", path);
                                    res.set_status(StatusCode::NotFound);
                                    res.headers_mut().set(ContentLength("404 Not Found".len() as u64));
                                }
                            }
                        }
                        else {
                            println!("NotFound: {}", path);
                            res.set_status(StatusCode::NotFound);
                            res.headers_mut().set(ContentLength("404 Not Found".len() as u64));
                        }
                    },
                    &Error::Fatal(ref astr) => {
                        println!("fatal error: {}", astr);
                        res.set_status(StatusCode::InternalServerError);
                        return Next::end();
                    },
                    _ => {
                        
                    }
                    
                }
                
                Next::write()
            }
        }
        
        
        
        
        
    }

    fn on_response_writable(&mut self, transport: &mut Encoder<HttpStream>) -> Next {
        
        match self.response {
            Ok(ref response) => {
                if let &Some(ref body) = response.body() {
                    // match transport.write(body) 
                    // write response.body.unwrap() to transport
                    match transport.write(&body[self.write_pos..]) {
                        Ok(0) => {
                            // println!("why write zero byte?");
                            Next::end()
                        },
                        Ok(n) => {
                            self.write_pos += n;
                            Next::write()
                        },
                        Err(e) => match e.kind() {
                            io::ErrorKind::WouldBlock => Next::write(),
                            _ => {
                                println!("write error {:?}", e);
                                Next::end()
                            }
                        }
                    }
                }
                else {
                    Next::end()
                }

                
            },
            Err(ref e) => {
                match e {
                    &Error::NotFound(ref path) => {
                        if self.sapp.static_service {
                            match self.static_file {
                                Some(ref avec) => {
                                    // transport.write(avec).unwrap();
                                    match transport.write(&avec[self.write_pos..]) {
                                        Ok(0) => {
                                            // println!("why write zero byte?");
                                            Next::end()
                                        },
                                        Ok(n) => {
                                            self.write_pos += n;
                                            Next::write()
                                        },
                                        Err(e) => match e.kind() {
                                            io::ErrorKind::WouldBlock => Next::write(),
                                            _ => {
                                                println!("write error {:?}", e);
                                                Next::end()
                                            }
                                        }
                                    }
                                },
                                None => {
                                    transport.write("404 Not Found".as_bytes()).unwrap();
                                    // end
                                    Next::end()
                                }
                            }
                        }
                        else {
                            transport.write("404 Not Found".as_bytes()).unwrap();
                            Next::end()
                        }
                    },
                    _ => {
                        Next::end()
                    }
                }
            }
        }
       
    }
}
