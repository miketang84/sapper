use hyper::status::StatusCode;
use hyper::header::Headers;

/// Sapper response struct
#[derive(Debug, Clone)]
pub struct SapperResponse {
    status: StatusCode,
    headers: Headers,
    body: Option<Vec<u8>>,
}

impl PartialEq for SapperResponse {
    fn eq(&self, other: &Self) -> bool {
        self.status() == other.status() && self.headers() == other.headers()
            && self.body() == other.body()
    }
}

impl SapperResponse {
    pub fn new() -> SapperResponse {
        SapperResponse {
            status: StatusCode::Ok,
            headers: Headers::new(),
            body: None
        }
    }
    
    /// get response status
    pub fn status(&self) -> StatusCode {
        self.status
    }
    
    /// set response status
    pub fn set_status(&mut self, status: StatusCode) {
        self.status = status;
    }
    
    
    /// get response headers ref
    pub fn headers(&self) -> &Headers {
        &self.headers
    }
    
    /// get response headers mut ref
    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }
    
    /// get response body mut ref
    pub fn body(&self) -> &Option<Vec<u8>>{
        &self.body
    }
    
    /// write string to body
    pub fn write_body(&mut self, body: String) {
        self.body = Some(body.as_bytes().to_vec())
    }
    
    /// write raw u8 vec to body
    pub fn write_raw_body(&mut self, body: Vec<u8>) {
        self.body = Some(body)
    }
    
}

