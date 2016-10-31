#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process;

use result::Result;
use reports::{ Report, Reports };
use wc_count::{ WcCount, count };
use wc_option::WcOption;

mod reports;
mod result;
mod wc_count;
mod wc_option;

const USAGE: &'static str = "
Usage: rust-wc [options] ... [<file>...]
       rust-wc (-h | --help)
       rust-wc --version

Print newline, word, and byte counts for each FILE, and a total line if
more than one FILE is specified.  A word is a non-zero-length sequence of
characters delimited by white space.

With no FILE, or when FILE is -, read standard input.

Options:
  -c, --bytes   print the byte counts
  -h, --help    display this help and exit
  -l, --lines   print the newline counts
  -m, --chars   print the character counts
  -w, --words   print the word counts
  --version     display version and exit
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_bytes: bool,
    flag_chars: bool,
    flag_words: bool,
    flag_lines: bool,
    arg_file: Vec<String>,
}

fn version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("{}", version)
}

fn run_file(path: &Path, option: &WcOption) -> Result<WcCount> {
    let mut s = String::new();
    if path == Path::new("-") {
        try!(io::stdin().read_to_string(&mut s));
    } else {
        let mut file = try!(File::open(path));
        try!(file.read_to_string(&mut s));
    }
    Ok(count(&s, option))
}

fn run(args: Args) -> Result<bool> {
    let is_default_option =
         !(args.flag_bytes || args.flag_chars
        || args.flag_words || args.flag_lines);
    let option =
        if is_default_option {
            WcOption { bytes: true,
                       chars: false,
                       words: true,
                       lines: true, }
        } else {
            WcOption { bytes: args.flag_bytes,
                       chars: args.flag_chars,
                       words: args.flag_words,
                       lines: args.flag_lines, }
        };
    let mut filenames: Vec<_> = args.arg_file;
    let mut reports           = Reports { data: vec![] };

    if filenames.len() < 1 {
        filenames.push("-".to_owned());
    }

    for filename in filenames.iter() {
        let path   = Path::new(filename);
        let result = run_file(&path, &option);
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
        report.print(width, &option);
    }

    Ok(!reports.has_error())
}

fn main() {
    env_logger::init().unwrap();

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.version(Some(version())).decode())
                            .unwrap_or_else(|e| e.exit());
    match run(args) {
        Ok(ok) if ok => process::exit(0),
        Ok(_)        => process::exit(1),
        Err(err)     => {
            error!("{}", err);
            process::exit(1);
        },
    }
}
