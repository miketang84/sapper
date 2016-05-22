# Sapper
 
Sapper, a lightweight web framework, written in Rust.

Sapper focuses on easy using. But it is in very alpha stage now and only compiled with rust nightly.


## Example

Now , you can boot the example server with:

```
cd examples/basic/
cargo build
cargo run
```

and open the browser, visit 

`http://localhost:1337/`

or

`http://localhost:1337/test`

or any other url to test it.

## Basic Benchmark

ThinkPad T410s  
Intel(R) Core(TM) i5 CPU M 560 @ 2.67GHz   
Single Thread (Using only one core)  
Ubuntu 14.04 x86  

about 3.5w qps.

```
mike@spirit:~/GIT/wrk$ ./wrk -t2 -c400 -d30s http://127.0.0.1:1337
Running 30s test @ http://127.0.0.1:1337
  2 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    10.98ms  427.68us  19.41ms   94.67%
    Req/Sec    18.29k   589.28    19.61k    82.17%
  1092800 requests in 30.03s, 132.36MB read
Requests/sec:  36393.03
Transfer/sec:      4.41MB
```

## Features

- Async, based on hyper mio branch;
- Sapper supplies only basic framework;
- Sapper only processes small request and response (with small request body, returning small response body) now;
- Three level granularity (global, module, function handler) middleware controller and unified middleware presentation; 
- Typesafe abstraction, keep the same spirit with hyper;
- For easy using, will supply some convenient macros to help write business logic;

## Philosophy

Sapper's philosophy is plugined, typed, hierarchical control.

### Plugined

Sapper's core contains only middleware/plugin system, router system, request and response definitions, and some other basic facilities. Nearly all practical features, such as query parameter, body parameter, cookie, session, json, xml, orm..., are supplied by the outer plugins.

Sapper's plugin is very easy to write. One rust module realized a function on the prototype of 

```rust
fn (&mut Request) -> Result<()>;  // before plugin
fn (&Request, &mut Response) -> Result<()>; // after plugin
```

can be thought as Sapper's plugin. Sample template please refer [sapper_query_params](https://github.com/sappworks/sapper_query_params) plugin.

### Typed

In Sapper, nearly every important thing is a `Type`. They are:

- Each module is a type, different modules are different types;
- Every plugin supply 0~n types for handler getting values;
- Inherited from hyper's typed spirit, all headers, mime and so on should use types for manipulation. 


### Hierarchical Control

- Sapper forces you to put router in each module (in main.rs, you can not write it, no space left for you to write);
- Sapper forces you to seperate the router binding and the handler realization;
- Sapper's plugin processor can be used in app level wrapper, module level wrapper, and each handler. These three level hierarchical controls make it flexible to construct your business.





## TODO

1. [X] QueryParams (x-www-form-urlencoded);
2. [X] BodyParams (x-www-form-urlencoded);
3. [X] BodyJsonParams;
4. [ ] Some macros;
5. [ ] Other components;


## Plugins

- [ReqQueryParams](https://github.com/sappworks/sapper_query_params)  parsing query string for req
- [ReqBodyParams](https://github.com/sappworks/sapper_body_params) parsing body parameters for req, including url form encoded, json type, json to struct macro. 
- [SessionCookie](https://github.com/sappworks/sapper_cookie) a cookie plugin, and else supply a helper set_cookie function
- [ReqBasicLogger](https://github.com/sappworks/sapper_request_basic_logger) record request and caculate its time

## Related Projects

Thanks to these projects below:

- [hyper](https://github.com/hyperium/hyper) Sapper is based on hyper mio branch;
- [iron](https://github.com/iron/iron) Sapper learns many designs from iron;
- [router](https://github.com/iron/router) Sapper steals router about code from it;
- [recognizer](https://github.com/conduit-rust/route-recognizer.rs) Sapper uses this route recognizer;


## License

MIT
