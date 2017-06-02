# Sapper

![](https://travis-ci.org/sappworks/sapper.svg?branch=master)
 
Sapper, a lightweight web framework, written in Rust.

Sapper focuses on easy of use. It can work with **stable** Rust (>= 1.17).


## Basic Example

Now, you can boot the example server with:

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

## Other Examples

1. [tiny](https://github.com/sappworks/sapper/tree/master/examples/tiny)
2. [init_global](https://github.com/sappworks/sapper/tree/master/examples/init_global)
3. [query params](https://github.com/sappworks/sapper_query/tree/master/examples/basic)
4. [body params](https://github.com/sappworks/sapper_body/tree/master/examples/basic)
5. [session](https://github.com/sappworks/sapper_session/tree/master/examples/basic)
6. [template rendering](https://github.com/sappworks/sapper_tmpl/tree/master/examples/basic)
7. [simple logger](https://github.com/sappworks/sapper_logger/tree/master/examples/basic)
7. [response json](https://github.com/sappworks/sapper_examples/tree/master/res_json)
10. more continued...

## Basic Benchmark

```
mike@mike-Vostro-3653:~/works2/wrk$ uname -a
Linux mike-Vostro-3653 4.10.0-21-generic #23-Ubuntu SMP Fri Apr 28 16:14:22 UTC 2017 x86_64 x86_64 x86_64 GNU/Linux


mike@mike-Vostro-3653:~/works2/wrk$ cat /proc/cpuinfo 
processor	: 0
vendor_id	: GenuineIntel
cpu family	: 6
model		: 94
model name	: Intel(R) Core(TM) i3-6100 CPU @ 3.70GHz
cpu MHz		: 832.183
cache size	: 3072 KB
bogomips	: 7392.00
...


mike@mike-Vostro-3653:~/works2/sapper/examples/tiny$ cargo run --release
    Finished release [optimized] target(s) in 36.27 secs
     Running `target/release/tiny`
Listening on http://127.0.0.1:1337

output: hello, world!


mike@mike-Vostro-3653:~/works2/wrk$ ./wrk -t2 -c100 -d30s http://127.0.0.1:1337
Running 30s test @ http://127.0.0.1:1337
  2 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    21.73us  139.92us  16.02ms   99.87%
    Req/Sec   235.94k    26.75k  259.73k    76.33%
  7045973 requests in 30.04s, 598.04MB read
Requests/sec: 234536.10
Transfer/sec:     19.91MB
```

## Features

- Async, based on hyper mio branch;
- Sapper supplies only basic framework;
- Sapper only processes small request and response (with small request body, returning small response body) now;
- Three level granularity (global, module, function handler) middleware controller and unified middleware presentation; 
- Typesafe abstraction, keep the same spirit with hyper;
- For easy using, will supply some convenient macros to help write business logic;
- Global object cross requests;

## Philosophy

Sapper's philosophy is plugined, typed, hierarchical control.

### Plugined

Sapper's core contains only middleware/plugin system, router system, request and response definitions, and some other basic facilities. Nearly all practical features, such as query parameter, body parameter, cookie, session, json, xml, orm..., are supplied by the outer plugins.

Sapper's plugin is very easy to write. One rust module realized a function on the prototype of 

```rust
fn (&mut Request) -> Result<()>;  // before plugin
fn (&Request, &mut Response) -> Result<()>; // after plugin
```

can be thought as Sapper's plugin. Sample template please refer [sapper_query](https://github.com/sappworks/sapper_query) plugin.

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
3. [X] Basic static file serving for dev;
5. [X] Global object shared cross requests;
6. [X] Macros;
4. [ ] Multipart;



## Plugins

- [QueryParams](https://github.com/sappworks/sapper_query)  parsing query string for req;
- [BodyParams](https://github.com/sappworks/sapper_body) parsing body parameters for req, including url form encoded, json type, json to struct macro;
- [Logger](https://github.com/sappworks/sapper_logger) record request and caculate its time;
- [SapperSession](https://github.com/sappworks/sapper_session) a cookie plugin, and else supply a helper set_cookie function;


## Components

- [Template](https://github.com/sappworks/sapper_tmpl) use tera to render template;
- [sporm](https://github.com/sappworks/sporm) orm part can be used in sapper;
- [spcodegen](https://github.com/sappworks/spcodegen) codegen helper part to sporm;




## Related Projects

Thanks to these projects below:

- [hyper](https://github.com/hyperium/hyper) Sapper is based on hyper mio branch;
- [iron](https://github.com/iron/iron) Sapper learns many designs from iron;
- [router](https://github.com/iron/router) Sapper steals router about code from it;
- [recognizer](https://github.com/conduit-rust/route-recognizer.rs) Sapper uses this route recognizer;


## License

MIT
