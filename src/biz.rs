use std::result::Result as StdResult;
use sapp::Result;
use sapp::SModule;
use request::Request;
use response::Response;
use srouter::SRouter;
use sapp::SHandler;

#[derive(Clone)]
pub struct Biz;

// unsafe impl Send for Biz {}
// unsafe impl Sync for Biz {}


impl Biz {
    // those handlers in module Biz
    fn index(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    
    fn test(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, tang gang gang!".to_string());
        
        Ok(response)
    }
    
}

// set before, after middleware, and add routers
impl SModule for Biz {
    
    fn before(&self, req: &mut Request) -> Result<()> {
        println!("{}", "in Biz before.");
        Ok(())
    }
    
    fn after(&self, req: &mut Request, res: &mut Response) -> Result<()> {
        println!("{}", "in Biz after.");
        
        Ok(())
    }
    
    // here add routers ....
    fn router(&self, router: &mut SRouter) -> Result<()> {
        // need to use Router struct here
        // XXX: here could not write as this, should record first, not parse it now
        
        
        router.get("/", Biz::index);
        router.get("/123", Biz::index);
        router.get("/test", Biz::test);
        
        Ok(())
        
    }
    
    
}

