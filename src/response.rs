use std::default::Default;

use hyper::status::StatusCode;
use hyper::header::Headers;


#[derive(Default)]
pub struct Response {
    status: StatusCode,
    headers: Headers,
    // body: Option<String>,
    body: Option<Vec<u8>>,
}


impl Response {
    pub fn new() -> Response {
        // create this object
        let res: Response = Default::default();
        
        res
    }
    
    pub fn status(&self) -> StatusCode {
        self.status
    }
    
    pub fn set_status(&mut self, status: StatusCode) {
        self.status = status;
    }
    
    
    pub fn headers(&self) -> &Headers {
        &self.headers
    }
    
    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }
    
    
    pub fn body(&self) -> &Option<Vec<u8>>{
        &self.body
    }
    
    pub fn write_body(&mut self, body: String) {
        self.body = Some(body.as_bytes().to_vec())
    }
    
    pub fn write_raw_body(&mut self, body: Vec<u8>) {
        self.body = Some(body)
    }
    
}

