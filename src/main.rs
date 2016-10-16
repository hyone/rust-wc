#[macro_use]
extern crate log;
extern crate env_logger;

use std::borrow::Cow;
use std::env;
use std::error::Error as StdError;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

use count::*;
use error::Error;

mod count;
mod error;

pub type Result<T> = std::result::Result<T, Box<StdError>>;


fn print_result(path: &Path,
                line_count: usize,
                word_count: usize,
                byte_count: usize) {
    println!("\t{}\t{}\t{} {}",
             line_count,
             word_count,
             byte_count,
             path.display());
}

fn command_name(path: &Path) -> Cow<str> {
    let command = Path::new(path).file_name().unwrap();
    command.to_string_lossy()
}

fn help(path: &Path) -> String {
    format!("{} <filename>", command_name(path))
}

fn run(args: Vec<String>) -> Result<()> {
    if let Some(filename) = args.get(1) {
        let path     = Path::new(&filename);
        let mut file = try!(File::open(path).map_err(|e| {
            Error::IoError { path: path.to_path_buf(), err: e }
        }));
        let mut s = String::new();
        try!(file.read_to_string(&mut s).map_err(|e| {
            Error::IoError { path: path.to_path_buf(), err: e }
        }));

        print_result(
            &path,
            count_lines(&s),
            count_words(&s),
            count_bytes(&s)
        );
    } else {
        println!("{}", help(Path::new(&args[0])));
    }
    Ok(())
}


fn main() {
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    match run(args) {
        Ok(_)  => process::exit(0),
        Err(e) => {
            error!("{}", e);
            process::exit(1);
        },
    }
}
