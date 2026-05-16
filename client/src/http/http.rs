use crate::http::{_Data, Method};

pub struct _Http {
    method: Method,
    data: Option<_Data>
}