use sapper::App as SapperApp;

mod bar;
mod foo;

pub fn main() {
    let mut sapp = SapperApp::new();
    sapp.address("127.0.0.1")
        .port(1337)
        .add_module(Box::new(foo::Foo))
        .add_module(Box::new(bar::Bar));
    
    println!("Listening on http://127.0.0.1:1337");
    sapp.run_http();
}

