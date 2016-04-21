
use hyper::server::Response;
use hyper::status::StatusCode

pub struct SResponse {
    raw_response: Response,
    body: String,
    
}


impl SResponse {
    pub fn new() -> SResponse {
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