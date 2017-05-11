use std::collections::HashMap;
use std::net::SocketAddr;

use hyper::server::Request as HyperRequest;
use hyper::Method;
use hyper::HttpVersion;
use hyper::header::Headers;
use hyper::Body;
use typemap::TypeMap;

pub struct SapperRequest {
    raw_req: &HyperRequest,
    ext: TypeMap
} 

impl SapperRequest {
    pub fn new(req: &HyperRequest) -> SapperRequest {

        SapperRequest {
            raw_req: req,
            ext: TypeMap::new()
        }
    }
    
    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.raw_req.remote_addr()
    }
    
    pub fn method(&self) -> &Method {
        self.raw_req.method()
    }
    
    pub fn version(&self) -> HttpVersion {
        self.raw_req.version()
    }
    
    pub fn headers(&self) -> &Headers {
        self.raw_req.headers()
    }
    
    pub fn path(&self) -> &str {
        self.raw_req.path()
    }
    
    pub fn query(&self) -> Option<&str> {
        self.raw_req.query()
    }
    
    pub fn body_ref(&self) -> Option<&Body> {
        self.raw_req.body_ref()
    }
    
    pub fn ext(&self) -> &TypeMap {
        &self.ext
    }
    
    pub fn ext_mut(&mut self) -> &mut TypeMap {
        &mut self.ext
    }
}

