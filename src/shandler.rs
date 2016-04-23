use request::Request;
use response::Response;


// all handler function in each module should fit this Handler trait
pub trait SHandler {
    fn handle(&self, req: &mut Request) -> Result<Response>;
}

