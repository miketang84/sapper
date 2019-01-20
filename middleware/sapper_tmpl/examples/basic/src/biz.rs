
use sapper::Result;
use sapper::SapperModule;
use sapper::Request;
use sapper::Response;
use sapper::SapperRouter;

use sapper_tmpl::{Context, render};


#[derive(Clone)]
pub struct Biz;


impl Biz {
    // those handlers in module Biz
    fn index(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    
    fn test(req: &mut Request) -> Result<Response> {
        
        let mut c = Context::new();
        c.add("title", &"This is a title.");
        c.add("header", &"This is a header.");
        
        let res_str = render("variables.html", &c);
        
        let mut response = Response::new();
        response.write_body(res_str);
        
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
        router.get("/test", Biz::test);
        router.post("/test", Biz::test_post);
        
        Ok(())
        
    }
    
    
}

