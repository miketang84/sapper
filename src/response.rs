use http::StatusCode;
use http::HeaderMap;

pub struct SapperResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: Option<Vec<u8>>,
}

// TODO: should only use may_http response instead
// no need to keep the internal buffer
impl SapperResponse {
    pub fn new() -> SapperResponse {
        SapperResponse {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: None,
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn set_status(&mut self, status: StatusCode) {
        self.status = status;
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    pub fn body(&self) -> &Option<Vec<u8>> {
        &self.body
    }

    pub fn write_body(&mut self, body: String) {
        self.body = Some(body.as_bytes().to_vec())
    }

    pub fn write_raw_body(&mut self, body: Vec<u8>) {
        self.body = Some(body)
    }
}
