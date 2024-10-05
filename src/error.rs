use warp::reject::Reject;
use std::fmt;

#[derive(Debug)]
pub struct CustomError {
    pub message: String,
}

impl Reject for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
