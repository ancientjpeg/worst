use std::fmt::{Debug, Formatter, Result};

pub struct Error {
    message: String,
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.message)
    }
}

impl<T : ToString> From<T> for Error 
{
    fn from(val:T) -> Self {
        Error { message: val.to_string() }
    }
}
