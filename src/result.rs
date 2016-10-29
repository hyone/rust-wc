use std::error::Error as StdError;
use std::result;

pub type Result<T> = result::Result<T, Box<StdError>>;
