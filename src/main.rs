#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;


pub type Result<T> = std::result::Result<T, Box<Error>>;


fn count_lines(content: &str) -> usize {
    content.lines().collect::<Vec<_>>().len()
}

fn count_words(content: &str) -> usize {
    content.split_whitespace().collect::<Vec<_>>().len()
}

fn count_chars(content: &str) -> usize {
    content.chars().count()
}

fn print_result(path: &Path,
                line_count: usize,
                word_count: usize,
                char_count: usize) {
    println!("\t{}\t{}\t{} {}",
             line_count,
             word_count,
             char_count,
             path.display());
}

fn help(command: &str) -> String {
    format!("{} <filename>", command)
}

fn main() {
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    if let Some(filename) = args.get(1) {
        let path     = Path::new(&filename);
        let mut file = File::open(path).unwrap_or_else(|e| {
            error!("{}: {}", path.display(), e);
            process::exit(1);
        });

        let mut s = String::new();
        file.read_to_string(&mut s).unwrap_or_else(|e| {
            error!("{}: {}", path.display(), e);
            process::exit(1);
        });

        print_result(
            &path,
            count_lines(&s),
            count_words(&s),
            count_chars(&s)
        );
    } else {
        let command = Path::new(&args[0]).file_name().unwrap();
        println!("{}", help(&command.to_string_lossy()));
    }
}
