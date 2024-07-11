use serde::{Deserialize, Serialize};

pub mod misc;
pub mod server;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request<T> {
    pub event: String,
    pub payload: Option<T>,
}

impl<T: std::fmt::Debug> std::fmt::Display for Request<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub event: String,
    pub payload: Option<T>,
}

impl<T: std::fmt::Debug> std::fmt::Display for Response<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
