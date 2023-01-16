use std::fmt::Debug;

use crate::http::{HTTPMethod, HTTPRequest, HTTPResponse};

pub struct RequestListener<'a> {
   pub path: String,
   pub method: HTTPMethod,
   pub callback: Box<dyn 'a + Send + FnMut(HTTPRequest) -> HTTPResponse> 
}

impl<'a> RequestListener<'a> {
    pub fn new<C: 'a + Send + FnMut(HTTPRequest) -> HTTPResponse>(path: String, method: HTTPMethod, callback: C) -> Self {
        Self {
            path,
            method,
            callback: Box::new(callback),
        }
    }
}

impl Debug for RequestListener<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestListener")
            .field("path", &self.path)
            .field("method", &self.method)
            .finish()
    }
}