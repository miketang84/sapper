
use hyper::server::Request as HyperRequest;
use hyper::method::Method;
use hyper::header::Headers;
use hyper::version::HttpVersion;
use std::collections::HashMap;
use typemap::TypeMap;

pub struct Request {
    method: Method,
    version: HttpVersion,
    headers: Headers,
    // only path part of this url
    path: String,
    // query string part of this url
    query_string: Option<String>,
    // if has body, keep it as raw here
    raw_body: Option<String>,
    // // params pair parsed from url query string
    // queries: Option<HashMap<String, String>>,
    // // params pair parsed from body
    // // if body is json, limit it to one level
    // body_params: Option<HashMap<String, String>>,
    // // combined params of queries and body_params
    // full_params: Option<HashMap<String, String>>,
    // ext key value pair
    ext: TypeMap
    
} 

impl Request {
    pub fn new(method: Method, version: HttpVersion, headers: Headers, pathstr: &str) -> Request {
        // seperate path and query_string
        let pathvec: Vec<&str> = pathstr.split('?').collect();
        let path = pathvec[0].to_owned();
        let mut query_string = None;
        
        // if has query_string
        if pathvec.len() > 1 {
            query_string = Some(pathvec[1].to_owned());
        }
        
        Request {
            method: method,
            version: version,
            headers: headers,
            
            path: path,
            query_string: query_string,
            raw_body: None,
            // queries: None,
            // body_params: None,
            // full_params: None,
            ext: TypeMap::new()
        }

    }
    
    pub fn method(&self) -> &Method {
        &self.method
    }
    
    pub fn version(&self) -> &HttpVersion {
        &self.version
    }
    
    pub fn headers(&self) -> &Headers {
        &self.headers
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
    
    pub fn set_raw_body(&mut self, body: String) -> &mut Self {
        self.raw_body = Some(body);
        self
    }
    
    // pub fn query(&self) -> &Option<HashMap<String, String>> {
    //     &self.queries
    // }
    
    // pub fn body(&self) -> &Option<HashMap<String, String>> {
    //     &self.body_params
    // }
    
    // pub fn params(&self) -> &Option<HashMap<String, String>> {
    //     &self.full_params
    // }
    
    pub fn get_ext(&mut self) -> &TypeMap {
        &self.ext
    }
    
    pub fn get_ext_mut(&mut self) -> &mut TypeMap {
        &mut self.ext
    }
}

