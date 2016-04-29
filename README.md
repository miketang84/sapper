# Sapper
 
Sapper, a lightweight web framework, written in Rust.

Sapper focuses on easy using. But it is in very alpha stage now.


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

1. [ ] QueryParams (x-www-form-urlencoded);
2. [ ] BodyParams (x-www-form-urlencoded);
3. [ ] BodyJsonParams;
4. [ ] Some macros;
5. [ ] Other components;


## Plugines

- [QueryParams](https://github.com/sappworks/sapper_query_params)  parsing query string for req
- [BodyParams](https://github.com/sappworks/sapper_body_params) parsing body url form parameters for req 


## Related Projects

Thanks to these projects below:

- [hyper](https://github.com/hyperium/hyper) Sapper is based on hyper mio branch;
- [iron](https://github.com/iron/iron) Sapper learns many designs from iron;
- [router](https://github.com/iron/router) Sapper steals router about code from it;
- [recognizer](https://github.com/conduit-rust/route-recognizer.rs) Sapper uses this route recognizer;


## License

MIT
