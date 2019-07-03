use sapper::{
    Result as SapperResult,
    Module,
    Request,
    Response,
    Router
};

pub struct Bar;

impl Bar {
    // those handlers in module Bar
    fn index(_req: &mut Request) -> SapperResult<Response> {
        let mut response = Response::new();
        response.write_body("hello, bar!".to_string());
        
        Ok(response)
    }
    
    fn test(_req: &mut Request) -> SapperResult<Response> {
        let mut response = Response::new();
        response.write_body("hello, bar test!".to_string());
        
        Ok(response)
    }
    
    fn test_post(req: &mut Request) -> SapperResult<Response> {
        println!("in test_post, raw_body: {:?}", req.body());
        
        let mut response = Response::new();
        response.write_body("hello, bar post test!".to_string());
        
        Ok(response)
    }
    
}

// set before, after middleware, and add routers
impl Module for Bar {
    
    // here add routers ....
    fn router(&self, router: &mut Router) -> SapperResult<()> {
        router.get("/", Self::index);
        router.get("/123", Self::index);
        router.get("/test", Self::test);
        router.post("/test", Self::test_post);
        
        Ok(())
    }
}

