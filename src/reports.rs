use std::fmt;

use result::Result;
use wc::WcCount;
use wc_option::WcOption;

const DEFAULT_WIDTH: usize = 8;

pub struct Report<T: fmt::Display> {
    pub name: T,
    pub result: Result<WcCount>,
}

impl <T: fmt::Display> Report<T> {
    pub fn print(&self, width: usize, option: &WcOption) {
        match self.result {
            Ok(ref wc_count) => {
                if option.lines { print!("{0:1$}", wc_count.lines, width) }
                if option.words { print!("{0:1$}", wc_count.words, width) }
                if option.chars { print!("{0:1$}", wc_count.chars, width) }
                if option.bytes { print!("{0:1$}", wc_count.bytes, width) }
                println!(" {0:1$}", self.name, width);
            },
            Err(ref err) => {
                error!("{}: {}", self.name, err);
            }
        }
    }
}

pub struct Reports<T: fmt::Display> {
    pub data: Vec<Report<T>>,
}

impl <T: fmt::Display> Reports<T> {
    pub fn has_error(&self) -> bool {
        self.iter().find(|&r| r.result.is_err()).is_some()
    }

    pub fn field_width(&self) -> usize {
        self.results_ok().fold(DEFAULT_WIDTH, |w, wc| {
            if wc.max_field_width() > w { wc.max_field_width() } else { w }
        })
    }

    pub fn results_ok<'a>(&'a self) -> Box<Iterator<Item=&'a WcCount> + 'a> {
        Box::new(self.iter().flat_map(|r| &r.result))
    }

    pub fn iter<'a>(&'a self) -> Box<Iterator<Item=&'a Report<T>> + 'a> {
        Box::new(self.data.iter())
    }

    pub fn push(&mut self, value: Report<T>) {
        self.data.push(value);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
