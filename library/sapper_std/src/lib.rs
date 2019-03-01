extern crate sapper;
extern crate sapper_tmpl;
extern crate sapper_query;
extern crate sapper_body;
extern crate sapper_session;
extern crate sapper_logger;

extern crate serde;
extern crate serde_json;

use sapper::{Request, Response, Result};

pub use sapper::PathParams;
pub use sapper_query::QueryParams;
pub use sapper_body::FormParams;
pub use sapper_body::JsonParams;
pub use sapper_session::{ SessionVal, set_cookie };
pub use sapper_tmpl::{
    Context as WebContext,
    TERA,
    TeraResult,
    TeraValue,
    to_value,
    render,
};

pub fn init(req: &mut Request, cookie_key: Option<&'static str>) -> Result<()> {
    sapper_logger::init(req)?;
    sapper_query::parse(req)?;
    sapper_body::parse(req)?;
    sapper_session::session_val(req, cookie_key)?;
    
    Ok(())
}


pub fn finish(req: &Request, res: &mut Response) -> Result<()> {
    sapper_logger::log(req, res)?;

    Ok(())
}


// ============ Status Code ============

#[macro_export]
macro_rules! res_redirect {
    ($redirect_uri:expr) => ({
        use sapper::Response;
        use sapper::status;
        use sapper::header::Location;

        let mut response = Response::new();
        response.set_status(status::Found);
        //response.set_status(status::TemporaryRedirect);
        response.headers_mut().set(Location($redirect_uri.to_owned()));
        response.write_body(format!("redirect to {}", $redirect_uri));

        Ok(response)
    })
}

#[macro_export]
macro_rules! res_400 {
    ($info:expr) => ({
        use sapper::Response;
        use sapper::status;

        let mut response = Response::new();
        response.set_status(status::BadRequest);
        response.write_body($info.to_owned());

        Ok(response)
    })
}

#[macro_export]
macro_rules! res_404 {
    ($info:expr) => ({
        use sapper::Response;
        use sapper::status;

        let mut response = Response::new();
        response.set_status(status::NotFound);
        response.write_body($info.to_owned());

        Ok(response)
    })
}

#[macro_export]
macro_rules! res_500 {
    ($info:expr) => ({
        use sapper::Response;
        use sapper::status;

        let mut response = Response::new();
        response.set_status(status::InternalServerError);
        response.write_body($info.to_owned());

        Ok(response)
    })
}


#[macro_export]
macro_rules! set_response_redirect {
    ($response:expr, $redirect_uri:expr) => ({
        use sapper::status;
        use sapper::header::Location;

        $response.set_status(status::Found);
        //response.set_status(status::TemporaryRedirect);
        $response.headers_mut().set(Location($redirect_uri.to_owned()));
        $response.write_body(format!("redirect to {}", $redirect_uri));
    })
}

// ============ Json ============

#[macro_export]
macro_rules! res_json {
    ($json:expr) => ({
        use serde_json;
        use sapper::Response;
        use sapper::header::ContentType;

        let mut response = Response::new();
        response.headers_mut().set(ContentType::json());
        response.write_body(serde_json::to_string(&$json).unwrap());

        Ok(response)
    })
}


#[macro_export]
macro_rules! res_json_ok {
    ($info:expr) => ({
        use sapper::Response;
        use serde_json;
        
        let json2ret = json!({
            "success": true,
            "info": $info
        });

        res_json!(json2ret)
    })
}

#[macro_export]
macro_rules! res_json_error {
    ($info:expr) => ({
        use sapper::Response;
        use serde_json;
        
        let json2ret = json!({
            "success": false,
            "info": $info
        });
        
        res_json!(json2ret)
    })
}

// ============ Page Render ============

#[macro_export]
macro_rules! res_html {
    ($html:expr, $context:expr) => ({
        use sapper::Response;
        use sapper::header::ContentType;

        let res_str = render($html, $context);

        let mut response = Response::new();
        response.headers_mut().set(ContentType::html());
        response.write_body(res_str);

        Ok(response)
    })
}


// ============ Params ============

#[macro_export]
macro_rules! get_params {
    ($req:expr, $tykey:ty) => ({
        match $req.ext().get::<$tykey>() {
            Some(params) => {
                params
            },
            None => return res_400!("no params")
        }
    })
}

#[macro_export]
macro_rules! get_path_params {
    ($req:expr) => ({
        get_params!($req, PathParams)
    })
}


#[macro_export]
macro_rules! get_query_params {
    ($req:expr) => ({
        get_params!($req, QueryParams)
    })
}

#[macro_export]
macro_rules! get_form_params {
    ($req:expr) => ({
        get_params!($req, FormParams)
    })
}

#[macro_export]
macro_rules! get_json_params {
    ($req:expr) => ({
        match serde_json::from_value(get_params!($req, JsonParams).clone()) {
            Ok(val) => val,
            Err(_) => {
                return res_400!("Json parameter not match to struct.");
            }
        }
    })
}

#[macro_export]
macro_rules! t_cond {
    ($bool:expr, $prompt:expr) => ({
        match $bool {
            true => (),
            false => {
                println!("test param condition result: {}", $prompt);
                return res_400!(format!("test param condition result: {}.", $prompt) );
            }
        }
    })
}

#[macro_export]
macro_rules! _missing_or_unrecognized {
    ($field:expr) => ({
        println!("missing or unrecognized parameter {}.", $field);
        return res_400!(format!("missing or unrecognized parameter {}.", $field));
    })
}

#[macro_export]
macro_rules! _using_default {
    ($field:expr, $default:expr) => ({
        println!("missing or unrecognized parameter {}, using default {}.", $field, $default);
        // return default
        $default
    })
}




// for PathParams, QueryParams and FormParams
#[macro_export]
macro_rules! t_param {
    ($params:expr, $field:expr) => ({
        match $params.get($field) {
            Some(ref astr) => &astr[0],
            None =>  _missing_or_unrecognized! ($field)
        }
    })
}

// for PathParams, QueryParams and FormParams, default version
#[macro_export]
macro_rules! t_param_default {
    ($params:expr, $field:expr, $default:expr) => ({
        match $params.get($field) {
            Some(ref astr) => &astr[0],
            None =>  _using_default! ($field, $default)
        }
    })
}

#[macro_export]
macro_rules! t_param_parse {
    ($params:expr, $field:expr, $tykey:ty) => ({
        match $params.get($field) {
            Some(ref astr) => {
                let _t = &astr[0];
                match _t.parse::<$tykey>() {
                    Ok(output) => output,
                    Err(_) => {
                        println!("parse parameter type error {}.", $field);
                        return res_400!(format!("parse parameter type error {}.", $field));
                    }
                }
            },
            None =>  _missing_or_unrecognized! ($field)
        }
    })
}

#[macro_export]
macro_rules! t_param_parse_default {
    ($params:expr, $field:expr, $tykey:ty, $default:expr) => ({
        match $params.get($field) {
            Some(ref astr) => {
                let _t = &astr[0];
                match _t.parse::<$tykey>() {
                    Ok(output) => output,
                    Err(_) => {
                        _using_default! ($field, $default)
                    }
                }
            },
            None =>  _using_default! ($field, $default)
        }
    })
}

#[macro_export]
macro_rules! t_arr_param {
    ($params:expr, $field:expr) => ({
        match $params.get($field) {
            Some(ref arr) => *arr,
            None =>  _missing_or_unrecognized! ($field)
        }
    })
}

#[macro_export]
macro_rules! t_arr_param_default {
    ($params:expr, $field:expr, $default:expr) => ({
        match $params.get($field) {
            Some(ref arr) => *arr,
            None =>  _using_default! ($field, $default)
        }
    })
}

#[macro_export]
macro_rules! t_has_param {
    ($params:expr, $field:expr) => ({
        match $params.get($field) {
            Some(_) => true,
            None => false 
        }
    })
}

// ============ Ext Type Params ============
#[macro_export]
macro_rules! ext_type {
    ($req:expr, $tykey:ty) => ({
        match $req.ext().get::<$tykey>() {
            Some(entity) => {
                Some(entity)
            },
            None => None
        }
    })
}


#[macro_export]
macro_rules! ext_type_owned {
    ($req:expr, $tykey:ty) => ({
        match $req.ext().get::<$tykey>() {
            Some(entity) => {
                Some(entity.clone())
            },
            None => None
        }
    })
}


