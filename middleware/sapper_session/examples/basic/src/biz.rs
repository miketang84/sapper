
use sapper::Result;
use sapper::SapperModule;
use sapper::Request;
use sapper::Response;
use sapper::SapperRouter;

#[derive(Clone)]
pub struct Biz;

use sapper_session::SessionVal;
use sapper_session::set_cookie;

impl Biz {
    // those handlers in module Biz
    fn index(_req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    
    fn test(req: &mut Request) -> Result<Response> {
        println!("{:?}", req.ext().get::<SessionVal>());
        
        let mut response = Response::new();
        response.write_body("hello, test!".to_string());
        
        let _ = set_cookie(&mut response, "TestSApp".to_string(), "99999999837343743432xxxyyyzzz".to_string(), None, None, None, None);
        
        Ok(response)
    }
    
    fn test_post(req: &mut Request) -> Result<Response> {
        
        println!("in test_post, raw_body: {:?}", req.body());
        
        let mut response = Response::new();
        response.write_body("hello, I'am post!".to_string());
        
        Ok(response)
    }
    
}

// set before, after middleware, and add routers
impl SapperModule for Biz {
    
    fn before(&self, _req: &mut Request) -> Result<()> {
        println!("{}", "in Biz before.");
        Ok(())
    }
    
    fn after(&self, _req: &Request, _res: &mut Response) -> Result<()> {
        println!("{}", "in Biz after.");
        
        Ok(())
    }
    
    // here add routers ....
    fn router(&self, router: &mut SapperRouter) -> Result<()> {
        // need to use Router struct here
        // XXX: here could not write as this, should record first, not parse it now
        
        
        router.get("/", Biz::index);
        router.get("/123", Biz::index);
        router.get("/test", Biz::test);
        router.post("/test", Biz::test_post);
        
        Ok(())
        
    }
    
    
}

