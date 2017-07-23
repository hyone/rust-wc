use std::fmt;

use result::Result;
use count::{CountStat, Options};

pub struct Report<T: fmt::Display> {
    pub name: T,
    pub result: Result<CountStat>,
}

impl <T: fmt::Display> Report<T> {
    pub fn print(&self, width: usize, option: &Options) {
        match self.result {
            Ok(ref count_stat) => {
                if option.lines { print!("{0:1$} ", count_stat.lines, width) }
                if option.words { print!("{0:1$} ", count_stat.words, width) }
                if option.chars { print!("{0:1$} ", count_stat.chars, width) }
                if option.bytes { print!("{0:1$} ", count_stat.bytes, width) }
                println!("{0:1$}", self.name, width);
            },
            Err(ref err) => {
                eprintln!("{}: {}", self.name, err);
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
        self.results_ok().fold(1, |w, wc| {
            if wc.max_field_width() > w { wc.max_field_width() } else { w }
        })
    }

    pub fn results_ok<'a>(&'a self) -> Box<Iterator<Item=&'a CountStat> + 'a> {
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

#[cfg(test)]
mod tests {
    use std::error::Error as StdError;
    use std::fmt;

    use count::CountStat;
    use super::*;

    #[derive(Debug)]
    struct TestError;

    impl StdError for TestError {
        fn description(&self) -> &str { "test" }
        fn cause(&self) -> Option<&StdError> { None }
    }
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "test")
        }
    }

    #[test]
    fn test_reports_has_error() {
        let reports = Reports { data: vec![
            Report { name: "test", result: Ok(CountStat::empty()) },
            Report { name: "test", result: Ok(CountStat::empty()) },
            Report { name: "test", result: Ok(CountStat::empty()) },
        ] };
        assert_eq!(reports.has_error(), false);

        let reports = Reports { data: vec![
            Report { name: "test", result: Ok(CountStat::empty()) },
            Report { name: "test", result: Ok(CountStat::empty()) },
            Report { name: "test", result: Err(Box::new(TestError)) },
        ] };
        assert_eq!(reports.has_error(), true);
    }

    #[test]
    fn test_results_ok() {
        let reports = Reports { data: vec![
            Report { name: "test", result: Ok(CountStat::empty()) },
            Report { name: "test", result: Ok(CountStat::empty()) },
            Report { name: "test", result: Err(Box::new(TestError)) },
        ] };
        assert_eq!(
            reports.results_ok().collect::<Vec<_>>(),
            vec![&CountStat::empty(), &CountStat::empty()]
        );

        // when no ok results
        let reports = Reports { data: vec![
            Report { name: "test", result: Err(Box::new(TestError)) },
            Report { name: "test", result: Err(Box::new(TestError)) },
        ] };
        assert_eq!(reports.results_ok().collect::<Vec<_>>(), vec![] as Vec<&CountStat>);
    }

    #[test]
    fn test_field_width() {
        let reports = Reports { data: vec![
            Report { name: "test", result: Ok(CountStat { bytes: 1523, ..CountStat::empty() }) },
            Report { name: "test", result: Ok(CountStat { bytes: 235238, ..CountStat::empty() }) },
            Report { name: "test", result: Ok(CountStat { bytes: 12, ..CountStat::empty() }) },
        ] };
        assert_eq!(reports.field_width(), 6);

        let reports = Reports { data: vec![
            Report { name: "test", result: Ok(CountStat { bytes: 13, ..CountStat::empty() }) },
            Report { name: "test", result: Ok(CountStat { bytes: 2, ..CountStat::empty() }) },
            Report { name: "test", result: Ok(CountStat { bytes: 2, ..CountStat::empty() }) },
        ] };
        assert_eq!(reports.field_width(), 2);

        // when only report with 0 bytes
        let reports = Reports { data: vec![
            Report { name: "test", result: Ok(CountStat::empty()) },
        ] };
        assert_eq!(reports.field_width(), 1);

        // when no reports
        let reports: Reports<&str> = Reports { data: vec![] };
        assert_eq!(reports.field_width(), 1);
    }
}
