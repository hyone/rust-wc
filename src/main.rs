use std::error::Error as StdError;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::Error as ioError;
use std::io::prelude::*;
use std::path::Path;


macro_rules! eprintln {
    ($($tt:tt)*) => {{
        use std::io::Write;
        let _ = writeln!(&mut ::std::io::stderr(), $($tt)*);
    }}
}


type Result<'a, T> = std::result::Result<T, Box<StdError + 'a>>;

#[derive(Debug)]
enum Error<'a> {
    OpenError(&'a Path, ioError),
    ReadError(&'a Path, ioError),
}

impl <'a> StdError for Error<'a> {
    fn description(&self) -> &str {
        match *self {
            Error::OpenError(_, _) => "OpenError",
            Error::ReadError(_, _) => "ReadError",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::OpenError(_, ref e) => Some(e),
            Error::ReadError(_, ref e) => Some(e)
        }
    }
}

impl <'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::OpenError(path, ref e) =>
                write!(f, "Couldn't open {}: {}", path.display(), e.description()),
            Error::ReadError(path, ref e) =>
                write!(f, "Couldn't read {}: {}", path.display(), e.description()),
        }
    }
}

fn read_content(path: &Path) -> Result<String> {
    let mut file = try!(
        File::open(path).or_else(|e| Err(Error::OpenError(path, e)))
    );
    let mut s = String::new();
    try!(
        file.read_to_string(&mut s).map_err(|e| Error::ReadError(path, e))
    );
    Ok(s)
}

fn print_content(path: &Path, content: &str) {
    let fullpath = path.canonicalize().unwrap();
    print!("[{}] contains:\n{}", fullpath.display(), content);
}

fn help(command: &str) -> String {
    format!("{} <filename>", command)
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    if let Some(filename) = argv.get(1) {
        let path = Path::new(&filename);
        match read_content(&path) {
            Ok(s)  => print_content(&path, &s),
            Err(e) => eprintln!("[Error]: {}", e),
        }
    } else {
        let command = Path::new(&argv[0]).file_name().unwrap();
        println!("{}", help(&command.to_string_lossy()));
    }
}
