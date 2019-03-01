#[macro_use]
extern crate lazy_static;
#[cfg(feature = "monitor")]
extern crate notify;
extern crate tera;

use tera::Tera;
use std::sync::RwLock;
pub use tera::{
    Context,
    Value as TeraValue,
    to_value,
    Result as TeraResult
};

lazy_static! {
    pub static ref TERA: RwLock<Tera> = RwLock::new(Tera::new("views/**/*").unwrap());
}


pub fn render(path: &str, context: Context) -> String {
    #[cfg(feature = "monitor")]
    monitor();

    TERA.read()
        .and_then(|tera| {
            Ok(tera.render(path, &context).unwrap_or_else(|e| {
                println!("rendering error: {:?}", e);
                "rendering error".to_owned()
            }))
        })
        .unwrap()
}

#[cfg(feature = "monitor")]
fn monitor() {
    use std::sync::{Once, ONCE_INIT};
    use notify::{watcher, RecursiveMode, Watcher};
    use std::sync::mpsc::channel;
    use std::thread::spawn;
    use std::time::Duration;

    static START: Once = ONCE_INIT;

    START.call_once(|| {
        spawn(move || {
            let (tx, rx) = channel();
            let mut watcher = watcher(tx, Duration::from_secs(5)).unwrap();
            watcher.watch("./views", RecursiveMode::Recursive).unwrap();

            loop {
                match rx.recv() {
                    Ok(_) => {
                        let _ = TERA.write().and_then(|mut tera| {
                            let _ = tera.full_reload();
                            Ok(())
                        });
                        println!("views change");
                    }
                    Err(e) => println!("watch error: {:?}", e),
                }
            }
        });
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
