
use sapper::Result;
use sapper::Request;
use sapper::Response;
use sapper::SapperModule;
use sapper::SapperRouter;

#[derive(Clone)]
pub struct Biz;

use FOOInt;
use FOOHashMap;
use FOOMutex;

impl Biz {
    // those handlers in module Biz
    fn index(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    
    fn test(req: &mut Request) -> Result<Response> {
        let a_global = req.ext().get::<FOOInt>();
        println!("in test, a_global is {:?}", a_global);
        let a_hash = req.ext().get::<FOOHashMap>();
        println!("in test, a_hash is {:?}", a_hash);
        let a_mutex = req.ext().get::<FOOMutex>();
        println!("in test, a_mutex is {:?}", a_mutex);
        {
            let a_mutex = a_mutex.unwrap();
            let mut data = a_mutex.lock().unwrap();
            data.insert("foo", "bar");
            
        }
        println!("in test, modified a_mutex is {:?}", a_mutex);
        
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
        // need to use Router struct here
        // XXX: here could not write as this, should record first, not parse it now
        
        
        router.get("/", Biz::index);
        router.get("/123", Biz::index);
        router.get("/test", Biz::test);
        router.post("/test", Biz::test_post);
        
        Ok(())
        
    }
    
    
}

