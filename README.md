# Sappers
 
Sappers, a lightweight web framework, written in Rust.

Sappers focuses on easy use. But it is in very alpha stage now.


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


## Design

- Async, based on hyper mio branch;
- Sappers supplies only basic framework;
- Sappers only processes small request and response (with small request body, returning small response body) now;
- Three level granularity (global, module, function handler) middleware controller and unified middleware presentation; 
- Typesafe abstraction, keep the same spirit with hyper;
- For easy use, will supply some convenient macros to help write business logic;

## TODO

1. [] QueryParams (x-www-form-urlencoded);
2. [] BodyParams (x-www-form-urlencoded);
3. [] BodyJsonParams;
4. [] Some macros;
5. [] Other components;



## Related Projects

Thanks to these projects below:

- [hyper](https://github.com/hyperium/hyper) Sappers is based on hyper mio branch;
- [iron](https://github.com/iron/iron) Sappers learns many designs from iron;
- [router](https://github.com/iron/router) Sappers steals router about code from it;
- [recognizer](https://github.com/conduit-rust/route-recognizer.rs) Sappers uses this route recognizer;


## License

MIT
