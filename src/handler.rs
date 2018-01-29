use std::any::Any;

use request::SapperRequest as Request;
use response::SapperResponse as Response;
use app::Result;

// all handler function in each module should fit this Handler trait
pub trait SapperHandler: Send + Sync + Any {
    fn handle(&self, &mut Request) -> Result<Response>;
}

impl<F> SapperHandler for F
where
    F: Send + Sync + Any + Fn(&mut Request) -> Result<Response>,
{
    fn handle(&self, req: &mut Request) -> Result<Response> {
        (*self)(req)
    }
}

impl SapperHandler for Box<SapperHandler> {
    fn handle(&self, req: &mut Request) -> Result<Response> {
        (**self).handle(req)
    }
}
