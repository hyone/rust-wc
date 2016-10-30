#[macro_use]
extern crate log;
extern crate env_logger;

use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io;
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
    let mut s    = String::new();
    if path == Path::new("-") {
        try!(io::stdin().read_to_string(&mut s));
    } else {
        let mut file = try!(File::open(path));
        try!(file.read_to_string(&mut s));
    }
    Ok(wc(&s))
}

fn run(args: Vec<String>) -> Result<bool> {
    let mut filenames: Vec<_> = args.into_iter().skip(1).collect();
    let mut reports           = Reports { data: vec![] };

    if filenames.len() < 1 {
        filenames.push("-".to_owned());
    }

    for filename in filenames.iter() {
        let path   = Path::new(filename);
        let result = run_file(&path);
        reports.push(Report {
            name: path.to_string_lossy(),
            result: result,
        });
    }
    if reports.len() > 1 {
        let total = reports.results_ok()
                           .fold(WcCount::empty(), |a, ref b| a + b);
        reports.push(Report {
            name: Cow::Owned("total".to_owned()),
            result: Ok(total)
        })
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
