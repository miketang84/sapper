
use hyper::server::Response as HyperResponse;
use hyper::status::StatusCode;
use hyper::header::Headers;

pub struct Response {
    status: StatusCode,
    headers: Headers,
    body: Option<String>,
}


impl Response {
    pub fn new() -> Response {
        // create this object
        
    }
    
    pub fn status(&self) -> StatusCode {
        
    }
    
    pub fn set_status(&mut self, status: StatusCode) {
        self.raw_response.set_status(status);
    }
    
    pub fn version(&self) -> &HttpVersion {
        self.raw_response.version()
    }
    
    pub fn headers(&self) -> &Headers {
        self.raw_response.headers()
    }
    
    pub fn headers_mut(&self) -> &mut Headers {
        self.raw_response.headers_mut()
    }
    
    pub fn write_body(&mut self, body: String) {
        
    }
    
    
}

