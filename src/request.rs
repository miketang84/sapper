use std::net::SocketAddr;
use std::io::Read;

use may_http::server::Request as RawRequest;
use may_http::server::RequestHeaders;
use http::Method;
use http::Version;
use http::header::HeaderMap;
use http::Uri;
use typemap::TypeMap;

pub struct SapperRequest {
    raw_req: RawRequest,
    ext: TypeMap,
}

impl SapperRequest {
    pub fn new(req: RawRequest) -> SapperRequest {
        SapperRequest {
            raw_req: req,
            ext: TypeMap::new(),
        }
    }

    // TODO:
    // pub fn remote_addr(&self) -> SocketAddr {
    //     self.raw_req.remote_addr
    // }

    pub fn method(&self) -> Method {
        self.raw_req.method()
    }

    pub fn version(&self) -> Version {
        self.raw_req.version()
    }

    pub fn headers(&self) -> RequestHeaders {
        self.raw_req.headers()
    }

    // TODO: optimize to (&str, Option<&str>)
    // uri() -> (path, query)
    pub fn uri(&self) -> (String, Option<String>) {
        unimplemented!()
        // match self.raw_req.uri {
        //     RequestUri::AbsolutePath(ref uri) => {

        //         let pathvec: Vec<&str> = uri[..].split('?').collect();
        //         let path = pathvec[0].to_owned();
        //         let mut query = None;

        //         // if has query_string
        //         if pathvec.len() > 1 {
        //             query = Some(pathvec[1].to_owned());
        //         }

        //         (path, query)
        //     },
        //     //_ => unreachable!()
        //     _ => {
        //         ("".to_owned(), None)
        //     }
        // }
    }

    //    pub fn query(&self) -> Option<&str> {
    //        self.raw_req.query()
    //    }

    // here, we read it all for simplify upper business logic
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

    pub fn ext(&self) -> &TypeMap {
        &self.ext
    }

    pub fn ext_mut(&mut self) -> &mut TypeMap {
        &mut self.ext
    }
}
