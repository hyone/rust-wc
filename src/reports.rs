use std::fmt;

use result::Result;
use wc_count::WcCount;
use wc_option::WcOption;

pub struct Report<T: fmt::Display> {
    pub name: T,
    pub result: Result<WcCount>,
}

impl <T: fmt::Display> Report<T> {
    pub fn print(&self, width: usize, option: &WcOption) {
        match self.result {
            Ok(ref wc_count) => {
                if option.lines { print!("{0:1$} ", wc_count.lines, width) }
                if option.words { print!("{0:1$} ", wc_count.words, width) }
                if option.chars { print!("{0:1$} ", wc_count.chars, width) }
                if option.bytes { print!("{0:1$} ", wc_count.bytes, width) }
                println!("{0:1$}", self.name, width);
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
        self.results_ok().fold(1, |w, wc| {
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

#[cfg(test)]
mod tests {
    use std::error::Error as StdError;
    use std::fmt;

    use wc_count::WcCount;
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

    fn wc_count_with_bytes(bytes: usize) -> WcCount {
        let mut wc_count = WcCount::empty();
        wc_count.bytes = bytes;
        wc_count
    }

    #[test]
    fn test_reports_has_error() {
        let reports = Reports { data: vec![
            Report { name: "test", result: Ok(WcCount::empty()) },
            Report { name: "test", result: Ok(WcCount::empty()) },
            Report { name: "test", result: Ok(WcCount::empty()) },
        ] };
        assert_eq!(reports.has_error(), false);

        let reports = Reports { data: vec![
            Report { name: "test", result: Ok(WcCount::empty()) },
            Report { name: "test", result: Ok(WcCount::empty()) },
            Report { name: "test", result: Err(Box::new(TestError)) },
        ] };
        assert_eq!(reports.has_error(), true);
    }

    #[test]
    fn test_results_ok() {
        let reports = Reports { data: vec![
            Report { name: "test", result: Ok(WcCount::empty()) },
            Report { name: "test", result: Ok(WcCount::empty()) },
            Report { name: "test", result: Err(Box::new(TestError)) },
        ] };
        assert_eq!(
            reports.results_ok().collect::<Vec<_>>(),
            vec![&WcCount::empty(), &WcCount::empty()]
        );

        // when no ok results
        let reports = Reports { data: vec![
            Report { name: "test", result: Err(Box::new(TestError)) },
            Report { name: "test", result: Err(Box::new(TestError)) },
        ] };
        assert_eq!(reports.results_ok().collect::<Vec<_>>(), vec![] as Vec<&WcCount>);
    }

    #[test]
    fn test_field_width() {
        let reports = Reports { data: vec![
            Report { name: "test1", result: Ok(wc_count_with_bytes(1523)) },
            Report { name: "test2", result: Ok(wc_count_with_bytes(235238)) },
            Report { name: "test3", result: Ok(wc_count_with_bytes(12)) },
        ] };
        assert_eq!(reports.field_width(), 6);

        let reports = Reports { data: vec![
            Report { name: "test1", result: Ok(wc_count_with_bytes(13)) },
            Report { name: "test2", result: Ok(wc_count_with_bytes(2)) },
            Report { name: "test3", result: Ok(wc_count_with_bytes(12)) },
        ] };
        assert_eq!(reports.field_width(), 2);

        // when only report with 0 bytes
        let reports = Reports { data: vec![
            Report { name: "test1", result: Ok(wc_count_with_bytes(0)) },
        ] };
        assert_eq!(reports.field_width(), 1);

        // when no reports
        let reports: Reports<&str> = Reports { data: vec![] };
        assert_eq!(reports.field_width(), 1);
    }
}
