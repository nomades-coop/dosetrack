use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct BadIdError;

impl Display for BadIdError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Bad Id")
    }
}

impl Error for BadIdError {}
