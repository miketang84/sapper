
use xxx::Router;

pub struct Biz;

impl Biz {
    // those handlers in module Biz
    fn xxx(req: Request) -> Result<Response, SError> {
        
        let mut response = Response::new();
        
        Ok(response)
    }
    
    fn yyy(req: Request) -> Result<Response, SError> {
        
        let mut response = Response::new();
        
        Ok(response)
    }
    
}

// set before, after middleware, and add routers
impl SModule for Biz {
    
    fn before(&self, &mut Request) -> Result<(), SError> {
        
    }
    
    fn after(&self, &mut Response) -> Result<(), SError> {
        
    }
    
    // here add routers ....
    fn router(&self) -> Router {
        // need to use Router struct here
        let router = Router::new();
        
        router.get("/xxx/bbb/eee", Biz::xxx);
        router.get("/yyy", Biz::yyy);
        
        router
    }
    
    
}

