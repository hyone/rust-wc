use std::error;
use std::fmt;
use std::io;
use std::path::PathBuf;
use std::result;

type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError { path: PathBuf, err: io::Error },
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError { ref err, .. } => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError { ref err, .. } => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError { ref err, ref path } =>
                write!(f, "IO Error on {}: {}", path.display(), err)
        }
    }
}
