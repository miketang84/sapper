
use std::collections::HashMap;
use hyper::method::Method;

use shandler::SHandler;



pub struct SRouter {
    router: HashMap<Method, Vec<(&'static str, Box<SHandler>)>>
}


impl SRouter {
    pub fn new() -> SRouter {
        SRouter {
            router: HashMap::new()
        }
    }

    pub fn route<H, S>(&mut self, method: Method,
                       glob: &str, handler: H) -> &mut SRouter
    where H: SHandler {
        self.router.entry(method).or_insert(Vec::new())
                    .push((glob, Box::new(handler)));
        self
    }

    /// Like route, but specialized to the `Get` method.
    pub fn get<H: SHandler>(&mut self, glob: &str, handler: H) -> &mut SRouter {
        self.route(Method::Get, glob, handler)
    }

    /// Like route, but specialized to the `Post` method.
    pub fn post<H: SHandler>(&mut self, glob: &str, handler: H) -> &mut SRouter {
        self.route(Method::Post, glob, handler)
    }

    /// Like route, but specialized to the `Put` method.
    pub fn put<H: SHandler>(&mut self, glob: &str, handler: H) -> &mut SRouter {
        self.route(Method::Put, glob, handler)
    }

    /// Like route, but specialized to the `Delete` method.
    pub fn delete<H: SHandler>(&mut self, glob: &str, handler: H) -> &mut SRouter {
        self.route(Method::Delete, glob, handler)
    }

    /// Like route, but specialized to the `Head` method.
    pub fn head<H: SHandler>(&mut self, glob: &str, handler: H) -> &mut SRouter {
        self.route(Method::Head, glob, handler)
    }

    /// Like route, but specialized to the `Patch` method.
    pub fn patch<H: SHandler>(&mut self, glob: &str, handler: H) -> &mut SRouter {
        self.route(Method::Patch, glob, handler)
    }

    /// Like route, but specialized to the `Options` method.
    pub fn options<H: SHandler>(&mut self, glob: &str, handler: H) -> &mut SRouter {
        self.route(Method::Options, glob, handler)
    }
}

