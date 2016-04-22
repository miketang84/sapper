use sapp::Result;
use router::Router;
use request::Request;
use response::Response;

pub struct Biz;

impl Biz {
    // those handlers in module Biz
    fn index(req: Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    
    fn test(req: Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    
}

// set before, after middleware, and add routers
impl SModule for Biz {
    
    fn before(&self, &mut Request) -> Result<()> {
        
        Ok(())
    }
    
    fn after(&self, &mut Response) -> Result<()> {
        
        Ok(())
    }
    
    // here add routers ....
    fn router(&self, router: &mut Router, prefix: &str) {
        // need to use Router struct here
        
        router.get("/", Biz::index);
        router.get("/test", Biz::test);
        
    }
    
    
}

