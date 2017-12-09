## 入门教程

### 前言

Sapper 是一个轻量的 Web 框架，目的是为了方便使用。做个类比吧，这个框架的量级类似于 Python 的 Falcon，但会比 Falcon 的完全裸装更高一些，接近于 Flask。

### 介绍
Sapper 主要依赖于 [Hyper 0.10.13](https://github.com/hyperium/hyper) 提供 http server，当异步能够使用的时候（Rust 标准库完成），会考虑改成异步模式。

Sapper 框架分为几个部分组成：

- [Sapper](https://github.com/sappworks/sapper)：主库，export 几个重要的 trait 和 struct，主要负责路由解析、错误处理等基本的框架功能，提供一个小的静态文件服务器的功能，单一的 Sapper 库是可以作为框架使用的，只是少一些比较方便的功能，所有流式数据需要自己转成想要的格式；
- [Sapper_std](https://github.com/sappworks/sapper_std)：对一些脚手架功能的包装，合并为 std 库，供用户使用，提供大量方便可用的 macro(宏)，同时将网络请求数据中的 Query/Form/Jsonbody 数据解析成方便使用的格式，可用很轻松的使用。

一般来说，在项目中，只需要引用上面两个库就可以正常工作了，当然，如果想要自己解析原始数据，也可以只引用 Sapper 主库。

下面几个库的功能全部被 Sapper_std 库引用并 export。
- [Sapper_session](https://github.com/sappworks/sapper_session) 提供识别并解析 cookies 的 function，以及设置 cookies 的 function。

- [Sapper_logger](https://github.com/sappworks/sapper_logger) 提供一个简单的 log 输出能力，日志格式如下：
```
[2017-12-09 12:20:12]  GET /ip/view Some("limit=25&offset=0") -> 200 OK (0.700839 ms)
```

- [Sapper_tmpl](https://github.com/sappworks/sapper_tmpl) 引入 [tera](https://github.com/Keats/tera) 这个 Jinja2 模板库，使 Sapper 能后端渲染 web 页面

- [Sapper_query](https://github.com/sappworks/sapper_query) 解析 url 查询字符

- [Sapper_body](https://github.com/sappworks/sapper_body) 解析请求 body 的数据: Form/Json

### 特性

Sapper 最大的特点是把 web 业务分为了三个层次（全局、模块、处理函数）去处理，每个模块都可以有自己的路由和中间件，全局可以定义全局共享的中间件和全局变量。把颗粒度放到最细就意味着每一个请求都可以有自己的中间件。

### 开始
这个示例尽量把 Sapper 的所有功能写进去，不过依然会有缺漏，一些没有写到的地方，可以参考 [Sapper_example](https://github.com/sappworks/sapper_example)

#### 建立项目

```
$ cargo new sapper_demo
```
在 cargo.toml 文件中增加依赖，为了演示自定义全局 not found 页面的功能（还未向 crates.io publish），依赖上增加 [patch] 项：
```toml
[dependencies]
 sapper = "^0.1"
 sapper_std = "^0.1"
 serde="*"
 serde_json="*"
 serde_derive="*"

[patch.crates-io]
 sapper = { git = 'https://github.com/sappworks/sapper.git' }
```

一个完整的 Sapper 项目目录大概如下，static 目录下的是静态文件，比如 js/css/png，views 目录下的是 web 模板文件，这个地方固定得比较死，tera 在 Sapper 内部指定加载这个目录下的所有文件。
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

在 `lib.rs` 下加入以下代码：
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
接下来先写好 `main.rs` 启动项：
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

上面的代码目前是不能运行的，`Foo` 和 `Bar` 这两个结构体还没有定义。这里首先讲解一下上面代码：

`SapperApp` 是 Sapper 库中的核心结构之一，整个 web 项目都围绕这个结构进行，下面介绍一下它的一些常用方法(具体函数签名可以直接看源码)：

`fn init_global()` 将想要全局共享的变量（如数据库连接池）注册在应用中。

`fn with_shell()` 注册全局中间件，上面代码将 std 的 `fn init()` 和 `fn finish()` 方法写入全局中间件。 `init` 将请求的各种参数解析并写入 `SapperRequest`，同时初始化 log，写入关注的 session key 值， `finish` 输出 log。

`fn add_moudle()` 注册子模块，每个模块需要符合 `SapperMoudle` trait。

`fn static_server()` 默认为 `true`, 即开启静态文件服务器功能。

`fn not_found_page()` 默认为 None，即如果路由没有指定的话，会返回 "404 not found" 字符串，如果需求自定义 404 页面，直接将对应的字符串传入即可。

#### Foo and Bar
在 foo.rs 里加上代码：
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

    // 解析 `/query?query=1`
    fn query(req: &mut Request) -> SapperResult<Response> {
        let params = get_query_params!(req);
        let query = t_param_parse!(params, "query", i64);
        let mut web = Context::new();
        web.add("data", &query);
        res_html!("index.html", web)
    }

    // 解析 `/user/:id`
    fn get_user(req: &mut Request) -> SapperResult<Response> {
        let params = get_path_params!(req);
        let id = t_param!(params, "id").clone();

        println!("{}", id);

        let json2ret = json!({
            "id": id
        });

        res_json!(json2ret)
    }

    // 解析 body json 数据
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

    // 解析 form 数据
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

`bar.rs` 代码：
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

`views` 文件夹下新增文件 `index.html`：
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

`static` 文件夹下新增文件 `foo.css`：
```css
p {
    color: red;
    text-align: center;
}
```

#### 代码解说
现在，demo 项目可以通过 `cargo run` 命令启动，监听在 `127.0.0.1:8080`。可以通过 `curl` 或者 Python 库 httpie 的命令 `http` 进行测试。 

##### `Foo` 模块

`Foo` 模块总共 5 个路由，分别展示 web 渲染、Query 解析、Path 解析、json 数据解析、Form 数据解析。除了标定的路由，其他请求都会到 `Error::NotFound` 这个错误处理里面，即返回 `SapperApp` 设定的 not found 界面，日志中不体现，设置了路由的访问，可以通过日志看到访问信息。

##### `Bar` 模块

`Bar` 模块设置了一个中间件，直接返回错误，这种情况下，访问 `127.0.0.1:8080/bar` 会直接在中间件被返回，即得到那个全局变量 `global`。

##### demo 源码
[https://github.com/sappworks/sapper_examples/tree/master/mvc_example](https://github.com/sappworks/sapper_examples/tree/master/sapper_demo)

#### Sapper 源码
Sapper 源码中，SapperRequest 是对 hypper request 的魔改封装：
```rust
pub struct SapperRequest<'a, 'b: 'a> {
    raw_req: Box<HyperRequest<'a, 'b>>,
    ext: TypeMap
} 
```
这个 [typemap::TypeMap](https://github.com/reem/rust-typemap) 就是核心了，是一个安全的类型值存储 Map。Sapper 中的 Query，Form，Cookies，Json 等信息都存储在这个地方，有兴趣的话，可以看看源码。

### 开源应用
无耻的放出了一个博客源码地址： [https://github.com/driftluo/MyBlog](https://github.com/driftluo/MyBlog)

### Contribute
欢迎加入 Sapper 社区，欢迎提供高质量的代码，高质量的思路。
