
use hyper::server::Request as HyperRequest;
use hyper::method::Method;
use hyper::header::Headers;
use hyper::version::HttpVersion;
use std::collections::HashMap;
use typemap::TypeMap;

#[derive(Default)]
pub struct Request {
    raw_request: HyperRequest,
    
    // only path part of this url
    path: String,
    // query string part of this url
    query_string: Option<String>,
    // if has body, keep it as raw here
    raw_body: Option<String>,
    // params pair parsed from url query string
    queries: Option<HashMap<String, String>>,
    // params pair parsed from body
    // if body is json, limit it to one level
    body_params: Option<HashMap<String, String>>,
    // combined params of queries and body_params
    full_params: Option<HashMap<String, String>>,
    // ext key value pair
    ext: TypeMap
    
} 

impl Request {
    pub fn new(raw_request: HyperRequest, pathstr: &str) -> Request {
        // seperate path and query_string
        let pathvec: Vec<&str> = path.split('?').collect();
        let path = pathvec[0].to_owned();
        let mut query_string = None;
        
        // if has query_string
        if pathvec.len() > 1 {
            query_string = Some(pathvec[1].to_owned());
        }
        
        Request {
            raw_request: raw_request,
            path: path,
            query_string: query_string,
            raw_body: None,
            queries: None,
            body_params: None,
            full_params: None,
            ext: TypeMap::new()
        }

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
    
    pub fn query_string(&self) -> &Option<String> {
        &self.query_string
    }
    
    pub fn raw_body(&self) -> &Option<String> {
        &self.raw_body
    }
    
    pub fn query(&self) -> &Option<HashMap<String, String>> {
        &self.queries
    }
    
    pub fn body(&self) -> &Option<HashMap<String, String>> {
        &self.body_params
    }
    
    pub fn params(&self) -> &Option<HashMap<String, String>> {
        &self.full_params
    }
    
    
}

