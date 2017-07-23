extern crate bytecount;

mod count;
mod report;
mod result;

pub use count::{CountStat, Options, count};
pub use report::{Report, Reports};
