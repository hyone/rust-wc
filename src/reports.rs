use std::fmt;

use result::Result;
use wc::WcCount;

pub struct Report<T: fmt::Display> {
    pub name: T,
    pub result: Result<WcCount>,
}

impl <T: fmt::Display> Report<T> {
    pub fn print(&self, width: usize) {
        match self.result {
            Ok(ref wc_count) => {
                println!("{:4$}\t{:4$}\t{:4$} {:4$}",
                         wc_count.lines,
                         wc_count.words,
                         wc_count.bytes,
                         self.name,
                         width);
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
        self.data.iter().find(|&r| r.result.is_err()).is_some()
    }

    pub fn has_ok_report(&self) -> bool {
        self.data.iter().find(|&r| r.result.is_ok()).is_some()
    }

    pub fn field_width(&self) -> usize {
        self.results_ok().iter().fold(7, |w, &wc| {
            if wc.max_field_width() > w { wc.max_field_width() } else { w }
        })
    }

    pub fn results_ok(&self) -> Vec<&WcCount> {
        self.data.iter().flat_map(|r| &r.result).collect::<Vec<_>>()
    }
}
