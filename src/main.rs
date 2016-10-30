#[macro_use]
extern crate log;
extern crate env_logger;

use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

use result::*;
use reports::*;
use wc::*;

mod reports;
mod result;
mod wc;

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
    let mut reports = Reports { data: vec![] };

    for filename in args.iter().skip(1) {
        let path   = Path::new(filename);
        let result = run_file(&path);
        reports.push(Report {
            name: path.to_string_lossy(),
            result: result,
        });
    }

    if reports.has_ok() {
        let total = reports.results_ok()
            .fold(WcCount::empty(), |a, ref b| a + b);
        reports.push(Report {
            name: Cow::Owned("total".to_owned()),
            result: Ok(total)
        })
    }

    if reports.len() < 1 {
        println!("{}", help(Path::new(&args[0])));
        return Ok(true);
    }

    let width = reports.field_width();
    for report in reports.iter() {
        report.print(width);
    }

    Ok(!reports.has_error())
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
