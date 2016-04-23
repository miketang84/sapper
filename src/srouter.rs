
use std::collections::HashMap;
use hyper::method;

use shandler::SHandler;



pub type SRouter = HashMap<Method, Vec<(&str, Box<SHandler>)>>;


impl SRouter {
    pub fn new() -> Router {
        HashMap::new()
    }

    pub fn route<H, S>(&mut self, method: method::Method,
                       glob: &str, handler: H) -> &mut Router
    where H: SHandler {
        self.router.entry(method).or_insert(Vec::new())
                    .push(glob, Box::new(handler));
        self
    }

    /// Like route, but specialized to the `Get` method.
    pub fn get<H: SHandler>(&mut self, glob: &str, handler: H) -> &mut Router {
        self.route(method::Get, glob, handler)
    }

    /// Like route, but specialized to the `Post` method.
    pub fn post<H: SHandler>(&mut self, glob: &str, handler: H) -> &mut Router {
        self.route(method::Post, glob, handler)
    }

    /// Like route, but specialized to the `Put` method.
    pub fn put<H: SHandler>(&mut self, glob: &str, handler: H) -> &mut Router {
        self.route(method::Put, glob, handler)
    }

    /// Like route, but specialized to the `Delete` method.
    pub fn delete<H: SHandler>(&mut self, glob: &str, handler: H) -> &mut Router {
        self.route(method::Delete, glob, handler)
    }

    /// Like route, but specialized to the `Head` method.
    pub fn head<H: SHandler>(&mut self, glob: &str, handler: H) -> &mut Router {
        self.route(method::Head, glob, handler)
    }

    /// Like route, but specialized to the `Patch` method.
    pub fn patch<H: SHandler>(&mut self, glob: &str, handler: H) -> &mut Router {
        self.route(method::Patch, glob, handler)
    }

    /// Like route, but specialized to the `Options` method.
    pub fn options<H: SHandler>(&mut self, glob: &str, handler: H) -> &mut Router {
        self.route(method::Options, glob, handler)
    }
}

