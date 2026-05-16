use crate::http::Method;

#[derive(Debug)]
pub struct Http {
    pub method: Method,
    pub data: Option<String>,
    pub headers: Option<Vec<(String, String)>>
}

impl Http {
    pub fn default() -> Self {
        Self { method: Method::GET, data: None, headers: None }
    }
}