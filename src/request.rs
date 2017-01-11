
use hyper::server::Request as HyperRequest;
use hyper::method::Method;
use hyper::header::Headers;
use hyper::version::HttpVersion;
use std::collections::HashMap;
use typemap::TypeMap;

pub struct Request {
    hyper_request: &HyperRequest,
    ext: TypeMap
    
} 

impl Request {
    pub fn new(hyper_request: &HyperRequest) -> Request {
        Request {
            hyper_request: hyper_request,
            ext: TypeMap::new()
        }

    }
    
    pub fn method(&self) -> &Method {
        self.hyper_request.method()
    }
    
    pub fn version(&self) -> &HttpVersion {
        self.hyper_request.version
    }
    
    pub fn headers(&self) -> &Headers {
        self.hyper_request.headers
    }
    
    pub fn path(&self) -> Option<&str> {
        self.hyper_request.path()
    }
    
    pub fn query(&self) -> Option<&str> {
        self.hyper_request.query()
    }
    
//    pub fn raw_body(&self) -> &Option<Vec<u8>> {
//        &self.raw_body
//    }
//    
//    pub fn set_raw_body(&mut self, body: Vec<u8>) -> &mut Self {
//        self.raw_body = Some(body);
//        self
//    }
    
    pub fn ext(&self) -> &TypeMap {
        &self.ext
    }
    
    pub fn ext_mut(&mut self) -> &mut TypeMap {
        &mut self.ext
    }
}

