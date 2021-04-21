pub mod error_type;

use std::{error::Error, fmt::Display};

pub type MyResult<T> = Result<T, MyError>;

#[derive(Debug)]
pub struct MyError(String);

impl MyError {
    pub fn new(desc: impl ToString) -> Self {
        Self(desc.to_string())
    }
}

impl Error for MyError {}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error!: {}", self.0)
    }
}
