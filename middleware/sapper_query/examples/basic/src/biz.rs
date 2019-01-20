
use sapper::Result;
use sapper::SapperModule;
use sapper::Request;
use sapper::Response;
use sapper::SapperRouter;

#[derive(Clone)]
pub struct Biz;

use sapper_query::QueryParams;

impl Biz {
    // those handlers in module Biz
    fn index(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    
    fn test(req: &mut Request) -> Result<Response> {
        // GET http://localhost:1337/test?a=1&b=2&c=3&a=4
        // Some({"a": ["1", "4"], "b": ["2"], "c": ["3"]})
        println!("{:?}", req.ext().get::<QueryParams>());
        
        // queries is now an Option<HashMap<String, Vec<String>>>
        let queries = req.ext().get::<QueryParams>();
        if queries.is_some() {
            // do something
            // let a = queries.get("a");
            // println!("{}", a);
        }
        
        let mut response = Response::new();
        response.write_body("hello, test!".to_string());
        
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
        router.get("/", Biz::index);
        router.get("/123", Biz::index);
        router.get("/test", Biz::test);
        router.post("/test", Biz::test_post);
        
        Ok(())
        
    }
    
    
}

