
use hyper::status::StatusCode;
use hyper::header::Headers;

#[derive(Default)]
pub struct Response {
    status: StatusCode,
    headers: Headers,
    body: Option<String>,
}


impl Response {
    pub fn new() -> Response {
        // create this object
        let res: Response = Default();
        
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
    
    pub fn headers_mut(&self) -> &mut Headers {
        &mut self.headers
    }
    
    pub fn write_body(&mut self, body: String) {
        self.body = Some(body)
    }
    
    
}

