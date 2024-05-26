
use reqwest::header::{HeaderMap ,HeaderValue, HeaderName};
use std::collections::HashMap;
use reqwest::blocking::{Client,Response};
use reqwest::{Body, Error};
    
#[derive(Clone)]
pub struct BearerApi {
    pub url: String,
    pub token: String
}

pub struct BasicApi {
    pub url: String,
    pub username: String,
    pub password: String
}

pub fn get_request(
    base_url: &str,
    header: Option<HeaderMap>
) -> Result<Response,Error>{
    let http_client = Client::new();
    match header {
        Some(header) => http_client
                    .get(base_url)
                    .headers(header).send() ,
        _ => http_client
        .get(base_url).send(),
    }
}

pub fn post_request(
    base_url: &str,
    header: Option<HeaderMap>,
    body: Option<String>) -> Result<Response,Error>{
    let http_client = Client::new();
    let req = match header {
        Some(header) => http_client
                    .post(base_url)
                    .headers(header) ,
        _ => http_client.post(base_url),
    };
    match body {
        Some(body) => req.body(body).send(),
        _ => req.send(),
    }
}

pub fn construct_header(header: &HashMap<String,String>) -> Option<HeaderMap> {
    
    let mut http_header = HeaderMap::new();
    for (key, val) in header.iter() { 
        let header_value = match HeaderValue::from_str(val) {
            Ok(hv) => hv,
            Err(_) => return None, // handle the error by returning None if any header value fails to parse
        };
        let header_name = match HeaderName::from_bytes(key.as_bytes()) {
            Ok(hn) => hn,
            Err(_) => return None, // handle the error by returning None if any header name fails to parse
        };
        http_header.insert(header_name, header_value);
    }
    // Ok(http_header)
    Some(http_header)
}
pub trait BaseAPI {
    fn new(base_url: &str, headers: Option<HeaderMap>,body: Option<String>) -> Self;
    fn base_url(&self) -> &String;
    fn headers(&self) -> &Option<HeaderMap>;
    fn body(&self) -> &Option<String>;
}
/// actual base rest http data sturcture
pub struct BaseAPi {
    base_url: String,
    headers: Option<HeaderMap>,
    body: Option<String>,
}
///actual implementation
impl BaseAPI for BaseAPi {
    fn new(base_url: &str, headers: Option<HeaderMap>,body: Option<String>) -> BaseAPi {
        // expect 3 para and return struct BaseAPi
        BaseAPi {
            base_url: base_url.to_string(),
            headers: Some(headers.unwrap_or_default()),
            body: Some(body.unwrap_or_default()),
        }
    }
    fn base_url(&self) -> &String {
        &self.base_url
    }
    fn headers(&self) -> &Option<HeaderMap> {
        &self.headers
    }
    fn body(&self) -> &Option<String> {
        &self.body
    }
}
