use std::collections::HashMap;
use std::sync::Arc;


use request::SapperRequest;
use response::SapperResponse;
use handler::SapperHandler;
use app::Result;
use app::Error;
use app::PathParams;
use app::Key;
use hyper::method::Method;

use recognizer::Router as Recognizer;
use recognizer::{Match, Params};

impl Key for PathParams { type Value = Params; }


/// `Router` provides an interface for creating complex routes as middleware
/// for the Iron framework.
pub struct Router {
    // The routers, specialized by method.
    routers: HashMap<Method, Recognizer<Arc<Box<dyn SapperHandler>>>>,
    // Routes that accept any method.
    wildcard: Recognizer<Arc<Box<dyn SapperHandler>>>
}

impl Router {
    pub fn new() -> Router {
	Router {
	    routers: HashMap::new(),
	    wildcard: Recognizer::new()
	}
    }

    pub fn route<S>(&mut self, method: Method,
		       glob: S, handler: Arc<Box<dyn SapperHandler>>) -> &mut Router
    where S: AsRef<str> {
	self.routers.entry(method).or_insert(Recognizer::new())
		    .add(glob.as_ref(), handler);
	self
    }


    fn recognize(&self, method: &Method, path: &str)
		     -> Option<Match<&Arc<Box<dyn SapperHandler>>>> {
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

    pub fn handle_method(&self, req: &mut SapperRequest, path: &str) -> Result<SapperResponse> {
	if let Some(matched) = self.recognize(req.method(), path) {
	    req.ext_mut().insert::<PathParams>(matched.params);
	    matched.handler.handle(req)
	} else {
	    // panic!("router not matched!");
	    // self.redirect_slash(req).and_then(|redirect| Some(Err(redirect)))
	    Err(Error::NotFound)
	}
    }
}
