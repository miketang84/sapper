use std::net::SocketAddr;
use std::io::Read;

use hyper::server::request::Request as HyperRequest;
use hyper::method::Method;
use hyper::version::HttpVersion;
use hyper::header::Headers;
use hyper::uri::RequestUri;
use typemap::TypeMap;


pub struct SapperRequest<'a, 'b: 'a> {
    raw_req: Box<HyperRequest<'a, 'b>>,
    ext: TypeMap
} 

impl<'a, 'b: 'a> SapperRequest<'a, 'b> {
    pub fn new(req: Box<HyperRequest<'a, 'b>>) -> SapperRequest<'a, 'b> {

        SapperRequest {
            raw_req: req,
            ext: TypeMap::new()
        }
    }
    
    pub fn remote_addr(&self) -> SocketAddr {
        self.raw_req.remote_addr
    }
    
    pub fn method(&self) -> &Method {
        &self.raw_req.method
    }
    
    pub fn version(&self) -> HttpVersion {
        self.raw_req.version
    }
    
    pub fn headers(&self) -> &Headers {
        &self.raw_req.headers
    }
    
    // TODO: optimize to (&str, Option<&str>)
    // uri() -> (path, query)
    pub fn uri(&self) -> (String, Option<String>) {
        match self.raw_req.uri {
            RequestUri::AbsolutePath(ref uri) => {
                
                let pathvec: Vec<&str> = uri[..].split('?').collect();
                let path = pathvec[0].to_owned();
                let mut query = None;

                // if has query_string
                if pathvec.len() > 1 {
                    query = Some(pathvec[1].to_owned());
                }
                
                (path, query)
            },
            _ => unreachable!()
        
        }
    }
    
//    pub fn query(&self) -> Option<&str> {
//        self.raw_req.query()
//    }
    
    // here, we read it all for simplify upper business logic
    pub fn body(&mut self) -> Option<Vec<u8>> {
        let mut body_vec: Vec<u8> = vec![];
        match self.raw_req.read_to_end(&mut body_vec) {
            Ok(n) => {
                if n > 0 {
                    Some(body_vec)
                }
                else {
                    None
                }
            },
            Err(_) => {
                println!("request body reading error!");
                None
            }
        }
    }
    
    pub fn ext(&self) -> &TypeMap {
        &self.ext
    }
    
    pub fn ext_mut(&mut self) -> &mut TypeMap {
        &mut self.ext
    }
}

