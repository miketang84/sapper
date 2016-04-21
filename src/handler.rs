
pub enum Route {
    NotFound,
    Got,
}

pub struct SApp {
    pub route: Route,
    pub res_str: String
}

impl SApp {
    pub fn new() -> SApp {
        SApp {
            route: Route::NotFound,
            res_str: "".to_string()
        }
    }
    
    pub fn hello (&mut self) -> String {
        let res_str = "hello swift rs.";
        res_str.to_string()
    }
}

