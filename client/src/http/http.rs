use crate::http::{Data, Method};

#[derive(Debug)]
pub struct _Http {
    pub method: Method,
    pub data: Option<Data>
}