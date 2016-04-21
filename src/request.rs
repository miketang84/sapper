
use hyper::server::Request as HyperRequest;
use hyper::method::Method;
use hyper::header::Headers;
use hyper::version::HttpVersion;
use std::collections::HashMap;

pub struct Request {
    raw_request: HyperRequest,
    
    // only path part of this url
    path: String,
    // query string part of this url
    query_string: String,
    // if has body, keep it as raw here
    raw_body: Option<String>,
    // params pair parsed from url query string
    queries: HashMap<String, String>,
    // params pair parsed from body
    // if body is json, limit it to one level
    body_params: HashMap<String, String>,
    // combined params of queries and body_params
    full_params: HashMap<String, String>,
    
}

impl Request {
    pub fn new() -> Request {
        // here, we should fill those extra fields from raw_request
        
        
        
    }
    
    pub fn method(&self) -> &Method {
        self.raw_request.method()
    }
    
    pub fn version(&self) -> &HttpVersion {
        self.raw_request.version()
    }
    
    pub fn headers(&self) -> &Headers {
        self.raw_request.headers()
    }
    
    pub fn path(&self) -> &String {
        &self.path
    }
    
    pub fn query(&self) -> &HashMap<String, String> {
        &self.queries
    }
    
    pub fn body(&self) -> &HashMap<String, String> {
        &self.body_params
    }
    
    
    
    
}

