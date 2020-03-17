use std::io::Read;
use std::net::SocketAddr;

use hyper::header::Headers;
use hyper::method::Method;
use hyper::server::request::Request as HyperRequest;
use hyper::uri::RequestUri;
use hyper::version::HttpVersion;
use typemap::TypeMap;

/// Sapper request struct
pub struct SapperRequest<'a, 'b: 'a> {
    raw_req: Box<HyperRequest<'a, 'b>>,
    ext: TypeMap,
}

impl<'a, 'b: 'a> SapperRequest<'a, 'b> {
    /// constructor
    pub fn new(req: Box<HyperRequest<'a, 'b>>) -> SapperRequest<'a, 'b> {
        SapperRequest {
            raw_req: req,
            ext: TypeMap::new(),
        }
    }

    /// get remote ip address
    pub fn remote_addr(&self) -> SocketAddr {
        self.raw_req.remote_addr
    }

    /// get http method
    pub fn method(&self) -> &Method {
        &self.raw_req.method
    }

    /// get http version
    pub fn version(&self) -> HttpVersion {
        self.raw_req.version
    }

    /// get http headers referrence
    pub fn headers(&self) -> &Headers {
        &self.raw_req.headers
    }

    /// get request path, and query parts
    // TODO: optimize to (&str, Option<&str>)
    // uri() -> (path, query)
    pub fn uri(&self) -> (String, Option<String>) {
        match self.raw_req.uri {
            RequestUri::AbsolutePath(ref uri) => {
                let pathvec: Vec<&str> = uri[..].split('?').collect();
                let path = pathvec[0].to_owned();
                let mut query = None;

                // if has query_string
                if pathvec.len() > 1 {
                    query = Some(pathvec[1].to_owned());
                }

                (path, query)
            }
            //_ => unreachable!()
            _ => ("".to_owned(), None),
        }
    }

    /// get the raw body vec of this request
    // here, we read it all for simplify upload biz
    pub fn body(&mut self) -> Option<Vec<u8>> {
        let mut body_vec: Vec<u8> = vec![];
        match self.raw_req.read_to_end(&mut body_vec) {
            Ok(n) => {
                if n > 0 {
                    Some(body_vec)
                } else {
                    None
                }
            }
            Err(_) => {
                println!("request body reading error!");
                None
            }
        }
    }

    /// get request struct ext ref
    pub fn ext(&self) -> &TypeMap {
        &self.ext
    }

    /// get request struct ext mut ref
    pub fn ext_mut(&mut self) -> &mut TypeMap {
        &mut self.ext
    }
}
