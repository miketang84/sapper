## Tutorial

### Preface

Sapper is a lightweight web mvc framework, easy for use.  Like python falcon/flask.

### Introduction

Sapper is based on [Hyper 0.10.13](https://github.com/hyperium/hyper), now using sync net mode, when async/await are ready, Sapper will follow it to walk to async framework.

Sapper consists of:

- [Sapper](https://github.com/sappworks/sapper): Main repository, exported some important traits and structs. It is used to router, parse parameters, error handling and so on. It also supplies a simple static file server. This repository can work without following helper repositories, but not very easy;
- [Sapper_std](https://github.com/sappworks/sapper_std): Wrappers for some basic components, such as parsing Query/Form/JsonBody, supplying some convenient macros to help rapid business coding; 

Generally speaking, in project, it is workable only using above two creates, but if you want to custom your flow, you can use following seperately:

The following crates are all dependant by `Sapper_std` and been exported.
- [Sapper_session](https://github.com/sappworks/sapper_session) offers cookie setting and session parsing function; 

- [Sapper_logger](https://github.com/sappworks/sapper_logger) offers a basic log function, format as:

```
[2017-12-09 12:20:12]  GET /ip/view Some("limit=25&offset=0") -> 200 OK (0.700839 ms)
```
- [Sapper_tmpl](https://github.com/sappworks/sapper_tmpl) using [tera](https://github.com/Keats/tera)  to render page;

- [Sapper_query](https://github.com/sappworks/sapper_query) parsing url query string;

- [Sapper_body](https://github.com/sappworks/sapper_body) parsing http body url encoded form data, or json data; 

### Characteristics

The biggest feature of Sapper is that: it divide business logic things into three levels - global, module, and in handler. You can define global shared variables and global shell (middlewares) and module router and middlewares, and handler middlewares.

### Start 

I will try my best to contain all features of sapper into this demo, but not all. [Sapper demo](https://github.com/sappworks/sapper_examples/tree/master/sapper_demo).

#### Create Project

```
$ cargo new sapper_demo
```
add dependencies to cargo.toml.

```toml
[dependencies]
 sapper = "^0.1"
 sapper_std = "^0.1"
 serde="*"
 serde_json="*"
 serde_derive="*"

```

A full Sapper project has these directories. Static files are all in static/, such as js/css/images, all html page templates are in views/. 
```
|-- src
|   |-- bin
|   |   |-- main.rs
|   |-- lib.rs
|   |-- bar.rs
|   |-- foo.rs
|-- static
|-- views
|-- Cargo.lock
|-- Cargo.toml
```

add the following lines to `lib.rs`:
```rust
extern crate sapper;
#[macro_use]
extern crate sapper_std;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

pub mod foo;
pub mod bar;

pub use foo::Foo;
pub use bar::{ Bar, Global };
```

#### bin 
let's write codes in `main.rs`:

```rust
extern crate sapper;
extern crate sapper_std;
extern crate sapper_demo;

use sapper::{ SapperApp, SapperAppShell, Request, Response, Result as SapperResult };
use sapper_demo::{ Foo, Bar, Global };
use std::sync::Arc;

struct WebApp;

impl SapperAppShell for WebApp {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        sapper_std::init(req, Some("session"))?;
        Ok(())
    }

    fn after(&self, req: &Request, res: &mut Response) -> SapperResult<()> {
        sapper_std::finish(req, res)?;
        Ok(())
    }
}

fn main() {
    let global = Arc::new(String::from("global variable"));
    let mut app = SapperApp::new();
    app.address("127.0.0.1")
        .port(8080)
        .init_global(
            Box::new(move |req: &mut Request| {
                req.ext_mut().insert::<Global>(global.clone());
                Ok(())
            })
        )
        .with_shell(Box::new(WebApp))
        .add_module(Box::new(Foo))
        .add_module(Box::new(Bar))
        .static_service(true)
        .not_found_page(String::from("not found"));

    println!("Start listen on {}", "127.0.0.1:8080");
    app.run_http();
}
```

Now, the above code can't work, we havn't define `Foo` and `Bar` now. We explain above code first:


`SapperApp` is one of the core structure of Sapper, all sapper project run around it. Let's introduce it:

- `fn init_global()` we register global shared variables (e.g. database connecting pool object) in this method; 

- `fn with_shell()` we register global middlewares in this shell;

- `fn add_moudle()` register sub module, one module one time;

- `fn static_server()` open static file service, if parameter is `true`, otherwise (`false`) close it;

- `fn not_found_page()` default is None, you can use this method to return custom 404 page.

#### Foo and Bar

add lines to foo.rs

```rust
use sapper::{ SapperModule, SapperRouter, Response, Request, Result as SapperResult };
use sapper_std::{ QueryParams, PathParams, FormParams, JsonParams, Context, render };
use serde_json;

pub struct Foo;

impl Foo {
    fn index(_req: &mut Request) -> SapperResult<Response> {
        let mut web = Context::new();
        web.add("data", &"Foo 模块");
        res_html!("index.html", web)
    }

    // parse `/query?query=1`
    fn query(req: &mut Request) -> SapperResult<Response> {
        let params = get_query_params!(req);
        let query = t_param_parse!(params, "query", i64);
        let mut web = Context::new();
        web.add("data", &query);
        res_html!("index.html", web)
    }

    // parse `/user/:id`
    fn get_user(req: &mut Request) -> SapperResult<Response> {
        let params = get_path_params!(req);
        let id = t_param!(params, "id").clone();

        println!("{}", id);

        let json2ret = json!({
            "id": id
        });

        res_json!(json2ret)
    }

    // parse body json
    fn post_json(req: &mut Request) -> SapperResult<Response> {
        #[derive(Serialize, Deserialize, Debug)]
        struct Person {
            foo: String,
            bar: String,
            num: i32,
        }

        let person: Person = get_json_params!(req);

        println!("{:#?}", person);

        let json2ret = json!({
            "status": true
        });
        res_json!(json2ret)
    }

    // parse form
    fn test_post(req: &mut Request) -> SapperResult<Response> {
        let params = get_form_params!(req);
        let foo = t_param!(params, "foo");
        let bar = t_param!(params, "bar");
        let num = t_param_parse!(params, "num", i32);

        println!("{}, {}, {}", foo, bar, num);

        let json2ret = json!({
            "status": true
        });
        res_json!(json2ret)
    }
}

impl SapperModule for Foo {
    fn before(&self, _req: &mut Request) -> SapperResult<()> {
        Ok(())
    }

    fn after(&self, _req: &Request, _res: &mut Response) -> SapperResult<()> {
        Ok(())
    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/foo", Foo::index);

        router.get("/query", Foo::query);

        router.get("/user/:id", Foo::get_user);

        router.post("/test_post", Foo::test_post);

        router.post("/post_json", Foo::post_json);

        Ok(())
    }
}
```

add lines to bar.rs

```rust
use sapper::{ SapperModule, SapperRouter, Response, Request, Result as SapperResult, Key, Error as SapperError };
use std::sync::Arc;
use sapper::header::ContentType;

pub struct Bar;

impl Bar {
    fn index(_req: &mut Request) -> SapperResult<Response> {
        let mut res = Response::new();
        res.headers_mut().set(ContentType::html());
        res.write_body(String::from("bar"));
        Ok(res)
    }
}

impl SapperModule for Bar {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        let global = req.ext().get::<Global>().unwrap().to_string();
        let res = json!({
            "error": global
        });
        Err(SapperError::CustomJson(res.to_string()))
    }

    fn after(&self, _req: &Request, _res: &mut Response) -> SapperResult<()> {
        Ok(())
    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/bar", Bar::index);
        Ok(())
    }
}

pub struct Global;

impl Key for Global {
    type Value = Arc<String>;
}
```

add new file `index.html` to `views`:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <link href="/foo.css" rel="stylesheet"/>
    <title>index</title>
</head>
<body>
<p>{{ data }}</p>
</body>
</html>
``` 
and new css file `foo.css` in `static/`
```css
p {
    color: red;
    text-align: center;
}
```

#### Explaination

Now, this demo can be run by `cargo run`, it will listen `127.0.0.1:8080`, we can test it with `curl`.

##### `Foo` module

`Foo` module contains 5 routers, to demonstrate web page rendering, query string parsing, path parameter parsing, json string parsing, form body parsing. 

##### `Bar` module

`Bar` module defines a middleware, used to return error directly, visiting `127.0.0.1:8080/bar` will cause middleware capture, returning that `global`.

##### demo source code

[https://github.com/sappworks/sapper_examples/tree/master/sapper_demo](https://github.com/sappworks/sapper_examples/tree/master/sapper_demo)

#### Middleware and Error Handling

The whole flow is: ** request -> global before -> module before -> handler -> module after -> global after -> response **, during this procedure, if error occures, it will break and do response directly.

Normally, the middleware of sapper will return `Ok(())`, meaning continuation. If middleware return `Err(Error instance)`, it will break and do response immediately. The Error enum is as follow:
```rust
pub enum Error {
    InvalidConfig,
    InvalidRouterConfig,
    FileNotExist,
    NotFound,
    Break,          // 400
    Unauthorized,   // 401
    Forbidden,      // 403
    TemporaryRedirect(String),     // 307
    Custom(String),
    CustomHtml(String),
    CustomJson(String),
}
```
`TemporaryRedirect` is used to redirect, `Custom, CustomHtml, CustomJson` are used to custom return values.

### Online Projects

- [Forustm] (https://github.com/rustcc/forustm)
- [MyBlog] (https://github.com/driftluo/MyBlog)

### Contribute

Welcome to Sapper Community, welcome PRs. Thank you.

