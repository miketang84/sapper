
use sapper::Result;
use sapper::SapperModule;
use sapper::Request;
use sapper::Response;
use sapper::SapperRouter;

use sapper::{ok, err};

#[derive(Clone)]
pub struct Foo;

impl Foo {
    // those handlers in module Foo
    fn index(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        ok(response)
    }
    
    fn test(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, test!".to_string());
        
        ok(response)
    }
    
    fn test_post(req: &mut Request) -> Result<Response> {
        
        println!("in test_post, raw_body: {:?}", req.body_ref());
        
        let mut response = Response::new();
        response.write_body("hello, I'am !".to_string());
        
        ok(response)
    }
    
}

// set before, after middleware, and add routers
impl SapperModule for Foo {
    
    fn before(&self, req: &mut Request) -> Result<()> {
        println!("{}", "in Foo before.");
        ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        println!("{}", "in Foo after.");
        
        ok(())
    }
    
    // here add routers ....
    fn router(&self, router: &mut SapperRouter) -> Result<()> {
        // need to use Router struct here
        
        router.get("/foo", Foo::index);
        router.get("/foo/", Foo::index);
        router.get("/foo/test", Foo::test);
        router.post("/foo/test", Foo::test_post);
        
        ok(())
        
    }
    
    
}

