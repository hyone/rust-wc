#[macro_use]
extern crate log;
extern crate env_logger;

use std::borrow::Cow;
use std::env;
use std::error::Error as StdError;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use wc::*;

mod wc;

pub type Result<T> = std::result::Result<T, Box<StdError>>;


fn print_counts<T: fmt::Display>(name: T, result: &WcCount) {
    println!("\t{}\t{}\t{} {}",
             result.lines,
             result.words,
             result.bytes,
             name);
}

fn command_name(path: &Path) -> Cow<str> {
    let command = Path::new(path).file_name().unwrap();
    command.to_string_lossy()
}

fn help(path: &Path) -> String {
    format!("{} <filename>", command_name(path))
}

fn run_file(path: &Path) -> Result<WcCount> {
    let mut file = try!(File::open(path));
    let mut s    = String::new();
    try!(file.read_to_string(&mut s));
    Ok(wc(&s))
}

fn run(args: Vec<String>) -> Result<bool> {
    let mut results = vec![];

    for filename in args.iter().skip(1) {
        let path   = Path::new(filename);
        let result = run_file(&path);
        results.push((path, result));
    }

    let mut total = WcCount::empty();
    for result in results.iter() {
        match result {
            &(ref path, Ok(ref wc_count)) => {
                print_counts(path.display(), wc_count);
                total += wc_count;
            },
            &(ref path, Err(ref err)) => error!("{}: {}", path.display(), err),
        }
    }

    if results.len() > 1 {
        print_counts("Total", &total);
    }

    if args.len() == 1 {
        println!("{}", help(Path::new(&args[0])));
    }

    let error_files = results.iter().map(|r| &r.1).filter(|r| r.is_err()).collect::<Vec<_>>().len();
    Ok(error_files == 0)
}

fn main() {
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    match run(args) {
        Ok(ok) if ok => process::exit(0),
        Ok(_)        => process::exit(1),
        Err(err)     => {
            error!("{}", err);
            process::exit(1);
        },
    }
}
