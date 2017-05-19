use std::collections::HashMap;
use std::sync::Arc;


use request::SapperRequest;
use response::SapperResponse;
use handler::SapperHandler;
use app::Result;
use app::Error;
use app::PathParams;
use app::Key;
use app::err;
use hyper::Method;

use recognizer::Router as Recognizer;
use recognizer::{Match, Params};

impl Key for PathParams { type Value = Params; }


/// `Router` provides an interface for creating complex routes as middleware
/// for the Iron framework.
pub struct Router {
    // The routers, specialized by method.
    routers: HashMap<Method, Recognizer<Arc<Box<SapperHandler>>>>,
    // Routes that accept any method.
    wildcard: Recognizer<Arc<Box<SapperHandler>>>
}

impl Router {
    pub fn new() -> Router {
        Router {
            routers: HashMap::new(),
            wildcard: Recognizer::new()
        }
    }

    pub fn route<S>(&mut self, method: Method,
                       glob: S, handler: Arc<Box<SapperHandler>>) -> &mut Router
    where S: AsRef<str> {
        self.routers.entry(method).or_insert(Recognizer::new())
                    .add(glob.as_ref(), handler);
        self
    }


    fn recognize(&self, method: &Method, path: &str)
                     -> Option<Match<&Arc<Box<SapperHandler>>>> {
        self.routers.get(method).and_then(|router| router.recognize(path).ok())
            .or(self.wildcard.recognize(path).ok())
    }

    // fn handle_options(&self, path: &str) -> Response {
    //     static METHODS: &'static [method::Method] =
    //         &[method::Get, method::Post, method::Post, method::Put,
    //           method::Delete, method::Head, method::Patch];

    //     // Get all the available methods and return them.
    //     let mut options = vec![];

    //     for method in METHODS.iter() {
    //         self.routers.get(method).map(|router| {
    //             if let Some(_) = router.recognize(path).ok() {
    //                 options.push(method.clone());
    //             }
    //         });
    //     }
    //     // If GET is there, HEAD is also there.
    //     if options.contains(&method::Get) && !options.contains(&method::Head) {
    //         options.push(method::Head);
    //     }

    //     let mut res = Response::with(status::StatusCode::Ok);
    //     res.headers.set(headers::Allow(options));
    //     res
    // }

    // Tests for a match by adding or removing a trailing slash.
    // fn redirect_slash(&self, req : &Request) -> Option<Error> {
    //     let mut url = req.url.clone();
    //     let mut path = url.path.join("/");

    //     if let Some(last_char) = path.chars().last() {
    //         if last_char == '/' {
    //             path.pop();
    //             url.path.pop();
    //         } else {
    //             path.push('/');
    //             url.path.push(String::new());
    //         }
    //     }

    //     self.recognize(&req.method(), &path).and(
    //         Some(Error::new(TrailingSlash,
    //                             (status::MovedPermanently, Redirect(url))))
    //     )
    // }

    pub fn handle_method(&self, req: &mut SapperRequest) -> Option<Result<SapperResponse>> {
        let path = req.path().to_owned();
        if let Some(matched) = self.recognize(req.method(), &path) {
            req.ext_mut().insert::<PathParams>(matched.params);
            Some(matched.handler.handle(req))
        } else { 
            // panic!("router not matched!");
            // self.redirect_slash(req).and_then(|redirect| Some(Err(redirect)))
            Some(err(Error::NotFound(path.to_owned()))) 
        }
    }
}

