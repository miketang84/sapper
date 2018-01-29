use std::sync::Arc;
use std::collections::HashMap;
use http::Method;

use handler::SapperHandler;

pub type InnerRouter = HashMap<Method, Vec<(&'static str, Arc<Box<SapperHandler>>)>>;

pub struct SapperRouter {
    router: InnerRouter,
}

impl SapperRouter {
    pub fn new() -> SapperRouter {
        SapperRouter {
            router: HashMap::new(),
        }
    }

    pub fn route<H>(&mut self, method: Method, glob: &'static str, handler: H) -> &mut SapperRouter
    where
        H: SapperHandler + 'static,
    {
        self.router
            .entry(method)
            .or_insert(Vec::new())
            .push((glob, Arc::new(Box::new(handler))));
        self
    }

    /// Like route, but specialized to the `Get` method.
    pub fn get<H: SapperHandler + 'static>(
        &mut self,
        glob: &'static str,
        handler: H,
    ) -> &mut SapperRouter {
        self.route(Method::GET, glob, handler)
    }

    /// Like route, but specialized to the `Post` method.
    pub fn post<H: SapperHandler + 'static>(
        &mut self,
        glob: &'static str,
        handler: H,
    ) -> &mut SapperRouter {
        self.route(Method::POST, glob, handler)
    }

    /// Like route, but specialized to the `Put` method.
    pub fn put<H: SapperHandler + 'static>(
        &mut self,
        glob: &'static str,
        handler: H,
    ) -> &mut SapperRouter {
        self.route(Method::PUT, glob, handler)
    }

    /// Like route, but specialized to the `Delete` method.
    pub fn delete<H: SapperHandler + 'static>(
        &mut self,
        glob: &'static str,
        handler: H,
    ) -> &mut SapperRouter {
        self.route(Method::DELETE, glob, handler)
    }

    /// Like route, but specialized to the `Head` method.
    pub fn head<H: SapperHandler + 'static>(
        &mut self,
        glob: &'static str,
        handler: H,
    ) -> &mut SapperRouter {
        self.route(Method::HEAD, glob, handler)
    }

    /// Like route, but specialized to the `Patch` method.
    pub fn patch<H: SapperHandler + 'static>(
        &mut self,
        glob: &'static str,
        handler: H,
    ) -> &mut SapperRouter {
        self.route(Method::PATCH, glob, handler)
    }

    /// Like route, but specialized to the `Options` method.
    pub fn options<H: SapperHandler + 'static>(
        &mut self,
        glob: &'static str,
        handler: H,
    ) -> &mut SapperRouter {
        self.route(Method::OPTIONS, glob, handler)
    }

    pub fn into_router(&self) -> &InnerRouter {
        &self.router
    }
}
