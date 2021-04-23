pub mod error_type;

use std::{error::Error, fmt::Display};

use crate::MyErrorType;

pub type MyResult<T> = Result<T, MyError>;

#[derive(Debug)]
pub struct MyError {
    typ: MyErrorType,
    desc: String,
}

impl MyError {
    pub fn new(typ: MyErrorType, desc: impl ToString) -> Self {
        Self {
            typ,
            desc: desc.to_string(),
        }
    }
}

impl Error for MyError {}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.typ, self.desc)
    }
}
