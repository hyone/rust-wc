extern crate bytecount;
extern crate docopt;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
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

Print newline, word, and byte counts for each <file>, and a total line if
more than one <file> is specified.  A word is a non-zero-length sequence of
characters delimited by white space.

With no <file>, or when <file> is -, read standard input.

Options:
  -c, --bytes   print the byte counts
  -h, --help    display this help and exit
  -l, --lines   print the newline counts
  -m, --chars   print the character counts
  -w, --words   print the word counts
  --version     display version and exit
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_bytes: bool,
    flag_chars: bool,
    flag_words: bool,
    flag_lines: bool,
    arg_file: Vec<String>,
}

fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

fn run_file(file: &str, option: &WcOption) -> Result<WcCount> {
    let mut s = String::new();
    if file == "-" {
        io::stdin().read_to_string(&mut s)?;
    } else {
        let path = Path::new(file);
        File::open(path)?.read_to_string(&mut s)?;
    }
    Ok(count(&s, option))
}

fn run(args: Args) -> Result<bool> {
    let option =
        if !(args.flag_bytes || args.flag_chars ||
             args.flag_words || args.flag_lines) {
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
    let mut reports = Reports { data: vec![] };

    if filenames.len() < 1 {
        filenames.push("-".to_owned());
    }

    for filename in filenames {
        let result = run_file(&filename, &option);
        reports.push(Report {
            name: filename,
            result: result,
        });
    }
    if reports.len() > 1 {
        let total = reports.results_ok()
                           .fold(WcCount::empty(), |a, ref b| a + b);
        reports.push(Report {
            name: "total".to_owned(),
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
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.version(Some(version())).deserialize())
                            .unwrap_or_else(|e| e.exit());
    match run(args) {
        Ok(ok) if ok => process::exit(0),
        Ok(_)        => process::exit(1),
        Err(err)     => {
            eprintln!("{}", err);
            process::exit(1);
        },
    }
}
