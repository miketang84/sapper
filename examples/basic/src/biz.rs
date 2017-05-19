
use sapper::Result;
use sapper::SapperModule;
use sapper::Request;
use sapper::Response;
use sapper::SapperRouter;
use sapper::Error;
use sapper::HyperError;

use std::str;
use sapper::Stream;
use sapper::Future;
use sapper::ok;

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
        
        let mut response = Response::new();
        response.write_body("hello, test!".to_string());
        
        Ok(response)
    }
    
    fn test_post(req: &mut Request) -> Result<Response> {
        //let (method, uri, _version, headers, body) = req.deconstruct();
        
        //let a = req.body().wait().unwrap();
        //println!("in test_post, raw_body: {:?}", a);
        
        for ref chunk in req.body().wait() {
            println!("in test_post, raw_body: {:?}", chunk);
        
        }
/*
        req.body()
            .fold(Vec::new(), |mut acc, chunk| {
                acc.extend_from_slice(&*chunk);
                Ok::<_, HyperError>(acc)
            })
            .map(move |body| {
                Ok::<_, HyperError>(Response::new()
                    .write_body(format!("Read {} bytes", body.len())))
            }).boxed()
*/
                
        //println!("in test_post, raw_body: {:?}", req.body());
        //match req.body() {
        //    Some(& ref body) => {
        //        println!("in test_post, body: {:?}", body);
                //body.for_each(|chunk| {
                //    println!("in body, {:?}", chunk);
                //    Ok(())
                //});
        //    },
        //    None => {
            
        //    }
        //}
        
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
        
        router.get("/", Biz::index);
        router.get("/123", Biz::index);
        router.get("/test", Biz::test);
        router.post("/test", Biz::test_post);
        
        Ok(())
        
    }
    
    
}

