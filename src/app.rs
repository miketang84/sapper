use std::str;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::clone::Clone;

use hyper::status::StatusCode;
use hyper::server::{Handler, Server, Request, Response};
use mime_types::Types as MimeTypes;


pub use typemap::Key;
pub use hyper::header::Headers;
pub use hyper::header;
pub use hyper::mime;
pub use request::SapperRequest;
pub use response::SapperResponse;
pub use router_m::Router;
pub use router::SapperRouter;
pub use handler::SapperHandler;


#[derive(Clone)]
pub struct PathParams;


/// Status Codes
pub mod status {
    pub use hyper::status::StatusCode as Status;
    pub use hyper::status::StatusCode::*;
}


#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    NotFound(String),
    InvalidConfig,
    InvalidRouterConfig,
    FileNotExist,
    ShouldRedirect(String),
    Break(String),
    Fatal(String),
    Custom(String),
}


pub type Result<T> = ::std::result::Result<T, Error>; 


pub trait SapperModule: Sync + Send {
    fn before(&self, req: &mut SapperRequest) -> Result<Option<SapperResponse>> {
        Ok(None)
    }
    
    fn after(&self, req: &SapperRequest, res: &mut SapperResponse) -> Result<()> {
        Ok(())
    }
    
    fn router(&self, &mut SapperRouter) -> Result<()>;
    
}

pub trait SapperAppShell {
    fn before(&self, &mut SapperRequest) -> Result<Option<SapperResponse>>;
    fn after(&self, &SapperRequest, &mut SapperResponse) -> Result<()>;
}

pub type GlobalInitClosure = Box<Fn(&mut SapperRequest) -> Result<()> + 'static + Send + Sync>;
pub type SapperAppShellType = Box<SapperAppShell + 'static + Send + Sync>;


pub struct SapperApp {
    pub address:        String,
    pub port:           u32,
    // for app entry, global middeware
    pub shell:          Option<Arc<SapperAppShellType>>,
    // for actually use to recognize
    pub routers:        Router,
    // do simple static file service
    pub static_service: bool,

    pub init_closure:   Option<Arc<GlobalInitClosure>>
}



impl SapperApp {
    pub fn new() -> SapperApp {
        SapperApp {
            address: String::new(),
            port: 0,
            shell: None,
            routers: Router::new(),
            static_service: true,
            init_closure: None
        }
    }
    
    pub fn address(&mut self, address: &str) -> &mut Self {
        self.address = address.to_owned();
        self
    }
    
    pub fn port(&mut self, port: u32) -> &mut Self {
        self.port = port;
        self
    }
    
    pub fn static_service(&mut self, open: bool) -> &mut Self {
        self.static_service = open;
        self
    }

    pub fn with_shell(&mut self, w: SapperAppShellType) -> &mut Self {
        self.shell = Some(Arc::new(w));
        self
    }
    
    pub fn init_global(&mut self, clos: GlobalInitClosure) -> &mut Self {
        self.init_closure = Some(Arc::new(clos));
        self
    }
    
    // add methods of this sapper module
    pub fn add_module(&mut self, sm: Box<SapperModule>) -> &mut Self {
        
        let mut router = SapperRouter::new();
        // get the sm router
        sm.router(&mut router).unwrap();
        let sm = Arc::new(sm);
        
        for (method, handler_vec) in router.into_router() {
            // add to wrapped router
            for &(glob, ref handler) in handler_vec.iter() {
                let method = method.clone();
                let glob = glob.clone();
                let handler = handler.clone();
                let sm = sm.clone();
                let shell = self.shell.clone();
                let init_closure = self.init_closure.clone();
                
                self.routers.route(method, glob, Arc::new(Box::new(move |req: &mut SapperRequest| -> Result<SapperResponse> {
                    if let Some(ref c) = init_closure {
                        c(req)?; 
                    }
                    if let Some(ref shell) = shell {
                        if let Some(response) = shell.before(req)? {
                            return Ok(response)
                        }
                    }
                    if let Some(response) = sm.before(req)? {
                        return Ok(response)
                    }
                    let mut response: SapperResponse = handler.handle(req)?;
                    sm.after(req, &mut response)?;
                    if let Some(ref shell) = shell {
                        shell.after(req, &mut response)?;
                    }
                    Ok(response)
                })));
            }
        }

        self
    }
    
    pub fn run_http(self) {
        
        let addr = self.address.clone() + ":" + &self.port.to_string();
        //let self_box = Arc::new(Box::new(self));

        Server::http(&addr[..]).unwrap()
                .handle(self).unwrap();
       
    }
}


impl Handler for SapperApp {

    fn handle(&self, req: Request, mut res: Response) {
        
        let mut sreq = SapperRequest::new(Box::new(req));
        let (path, query) = sreq.uri();

        // pass req to routers, execute matched biz handler
        let response_w = self.routers.handle_method(&mut sreq, &path).unwrap();
        if response_w.is_err() {
            
            if self.static_service {
                match simple_file_get(&path) {
                    Ok((file_u8vec, file_mime)) => {
                        res.headers_mut().set_raw("Content-Type", vec![file_mime.as_bytes().to_vec()]);
                        
                        return res.send(&file_u8vec[..]).unwrap();
                    },
                    Err(_) => {
                        *res.status_mut() = StatusCode::NotFound;
                        return res.send(&"404 Not Found".as_bytes()).unwrap();
                    }
                }
            }
        
            // return 404 NotFound now
            *res.status_mut() = StatusCode::NotFound;
            return res.send(&"404 Not Found".as_bytes()).unwrap();
        }
        
        let sres = response_w.unwrap();
        *res.status_mut() = sres.status();
        match sres.body() {
            &Some(ref vec) => {
                
                for header in sres.headers().iter() {
                    res.headers_mut()
                        .set_raw(header.name().to_owned(), 
                            vec![header.value_string().as_bytes().to_vec()]);
                }
                
                return res.send(&vec[..]).unwrap();
            },
            &None => {
                return res.send(&"".as_bytes()).unwrap();
            }
        }
        
    }
    
}




// this is very expensive in time
// should make it as global 
lazy_static! {
    static ref MTYPES: MimeTypes = { MimeTypes::new().unwrap() };
}

fn simple_file_get(path: &str) -> Result<(Vec<u8>, String)> {
    let new_path;
    if &path[(path.len()-1)..] == "/" {
        new_path = "static/".to_owned() + path + "index.html";
    }
    else {
        new_path = "static/".to_owned() + path;
    }
    //println!("file path: {}", new_path);
    match File::open(&new_path) {
        Ok(ref mut file) => {
            let mut s: Vec<u8> = vec![];
            file.read_to_end(&mut s).unwrap_or(0);
            
            let mt_str = MTYPES.mime_for_path(Path::new(&new_path));
            
            Ok((s, mt_str.to_owned()))
        },
        Err(_) => Err(Error::FileNotExist)
    }
}


