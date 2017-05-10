use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;
use std::any::Any;
use std::sync::Arc;


use request::Request;
use response::Response;
use shandler::SHandler;
use sapp::Result;
use sapp::Error;
use sapp::PathParams;
use sapp::Key;
use hyper::{status, header};
use hyper::method::Method;


use recognizer::Router as Recognizer;
use recognizer::{Match, Params};

impl Key for PathParams { type Value = Params; }


/// `Router` provides an interface for creating complex routes as middleware
/// for the Iron framework.
pub struct Router {
    // The routers, specialized by method.
    routers: HashMap<Method, Recognizer<Arc<Box<SHandler>>>>,
    // Routes that accept any method.
    wildcard: Recognizer<Arc<Box<SHandler>>>
}

impl Router {
    /// Construct a new, empty `Router`.
    ///
    /// ```ignore
    /// # use router::Router;
    /// let router = Router::new();
    /// ```
    pub fn new() -> Router {
        Router {
            routers: HashMap::new(),
            wildcard: Recognizer::new()
        }
    }

    /// Add a new route to a `Router`, matching both a method and glob pattern.
    ///
    /// `route` supports glob patterns: `*` for a single wildcard segment and
    /// `:param` for matching storing that segment of the request url in the `Params`
    /// object, which is stored in the request `extensions`.
    ///
    /// For instance, to route `Get` requests on any route matching
    /// `/users/:userid/:friend` and store `userid` and `friend` in
    /// the exposed Params object:
    ///
    /// ```ignore
    /// let mut router = Router::new();
    /// router.route(method::Get, "/users/:userid/:friendid", controller);
    /// ```
    ///
    /// The controller provided to route can be any `SHandler`, which allows
    /// extreme flexibility when handling routes. For instance, you could provide
    /// a `Chain`, a `SHandler`, which contains an authorization middleware and
    /// a controller function, so that you can confirm that the request is
    /// authorized for this route before handling it.
    // pub fn route<H, S>(&mut self, method: Method,
    //                    glob: S, handler: H) -> &mut Router
    // where H: SHandler + 'static, S: AsRef<str> {
    //     self.routers.entry(method).or_insert(Recognizer::new())
    //                 // .add(glob.as_ref(), Box::new(handler));
    //                 .add(glob.as_ref(), handler);
    //     self
    // }
    pub fn route<S>(&mut self, method: Method,
                       glob: S, handler: Arc<Box<SHandler>>) -> &mut Router
    where S: AsRef<str> {
        self.routers.entry(method).or_insert(Recognizer::new())
                    .add(glob.as_ref(), handler);
        self
    }


    fn recognize(&self, method: &Method, path: &str)
                     -> Option<Match<&Arc<Box<SHandler>>>> {
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
        let path = req.path();
        if let Some(matched) = self.recognize(req.method(), path) {
            req.ext_mut().insert::<PathParams>(matched.params);
            Some(matched.handler.handle(req))
        } else { 
            // panic!("router not matched!");
            // self.redirect_slash(req).and_then(|redirect| Some(Err(redirect)))
            Some(Err(Error::NotFound(path.to_owned()))) 
        }
    }
}

