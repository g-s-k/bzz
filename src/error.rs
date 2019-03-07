use std::any::Any;
use std::io;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Thread(Box<Any + Send>),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<Box<dyn Any + Send>> for Error {
    fn from(err: Box<dyn Any + Send>) -> Self {
        Error::Thread(err)
    }
}
