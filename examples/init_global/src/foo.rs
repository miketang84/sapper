
use sapper::Result;
use sapper::SModule;
use sapper::Request;
use sapper::Response;
use sapper::SRouter;

#[derive(Clone)]
pub struct Foo;

impl Foo {
    // those handlers in module Foo
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
    
    fn test_post(req: &mut Request) -> Result<Response> {
        
        println!("in test_post, raw_body: {:?}", req.raw_body());
        
        let mut response = Response::new();
        response.write_body("hello, I'am !".to_string());
        
        Ok(response)
    }
    
}

// set before, after middleware, and add routers
impl SModule for Foo {
    
    fn before(&self, req: &mut Request) -> Result<()> {
        println!("{}", "in Foo before.");
        Ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        println!("{}", "in Foo after.");
        
        Ok(())
    }
    
    // here add routers ....
    fn router(&self, router: &mut SRouter) -> Result<()> {
        // need to use Router struct here
        // XXX: here could not write as this, should record first, not parse it now
        
        
        router.get("/foo", Foo::index);
        router.get("/foo/", Foo::index);
        router.get("/foo/test", Foo::test);
        router.post("/foo/test", Foo::test_post);
        
        Ok(())
        
    }
    
    
}

