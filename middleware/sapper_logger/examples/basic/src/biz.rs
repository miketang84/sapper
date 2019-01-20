
use sapper::Result;
use sapper::SapperModule;
use sapper::Request;
use sapper::Response;
use sapper::SapperRouter;

#[derive(Clone)]
pub struct Biz;


impl Biz {
    // those handlers in module Biz
    fn index(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    

    
}

// set before, after middleware, and add routers
impl SapperModule for Biz {
    
    fn before(&self, req: &mut Request) -> Result<()> {
        println!("{}", "in Biz before.");
        Ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        println!("{}", "in Biz after.");
        
        Ok(())
    }
    
    // here add routers ....
    fn router(&self, router: &mut SapperRouter) -> Result<()> {
        // need to use Router struct here
        // XXX: here could not write as this, should record first, not parse it now
        
        router.get("/", Biz::index);
        router.get("/123", Biz::index);
        
        Ok(())
        
    }
    
    
}

