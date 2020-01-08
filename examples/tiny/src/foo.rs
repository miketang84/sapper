use sapper::{Module, Request, Response, Result as SapperResult, Router};

pub struct Foo;

impl Foo {
    // those handlers in module Foo
    fn index(_req: &mut Request) -> SapperResult<Response> {
        let mut response = Response::new();
        response.write_body("hello, foo!".to_string());

        Ok(response)
    }

    fn test(_req: &mut Request) -> SapperResult<Response> {
        let mut response = Response::new();
        response.write_body("hello, foo test!".to_string());

        Ok(response)
    }

    fn test_post(req: &mut Request) -> SapperResult<Response> {
        println!("in test_post, raw_body: {:?}", req.body());

        let mut response = Response::new();
        response.write_body("hello, foo post test!".to_string());

        Ok(response)
    }
}

// set before, after middleware, and add routers
impl Module for Foo {
    // here add routers ....
    fn router(&self, router: &mut Router) -> SapperResult<()> {
        router.get("/foo", Self::index);
        router.get("/foo/", Self::index);
        router.get("/foo/test", Self::test);
        router.post("/foo/test", Self::test_post);

        Ok(())
    }
}
