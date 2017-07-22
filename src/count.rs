use std::ops;
use bytecount;

pub struct Options {
    pub bytes: bool,
    pub chars: bool,
    pub words: bool,
    pub lines: bool,
}

#[derive(Debug, PartialEq)]
pub struct CountStat {
    pub bytes: usize,
    pub chars: usize,
    pub words: usize,
    pub lines: usize,
}

impl CountStat {
    pub fn empty() -> CountStat {
        CountStat {
            lines: 0,
            words: 0,
            chars: 0,
            bytes: 0,
        }
    }

    pub fn max_field_width(&self) -> usize {
        self.bytes.to_string().len()
    }
}

impl <'a> ops::Add<&'a CountStat> for CountStat {
    type Output = CountStat;

    fn add(self, rhs: &'a CountStat) -> CountStat {
        CountStat {
            lines: self.lines + rhs.lines,
            words: self.words + rhs.words,
            chars: self.chars + rhs.chars,
            bytes: self.bytes + rhs.bytes,
        }
    }
}

pub fn count(content: &str, options: &Options) -> CountStat {
    CountStat {
        bytes: if options.bytes { count_bytes(content) } else { 0 },
        chars: if options.chars { count_chars(content) } else { 0 },
        words: if options.words { count_words(content) } else { 0 },
        lines: if options.lines { count_lines(content) } else { 0 },
    }
}

fn count_lines(content: &str) -> usize {
    bytecount::count(content.as_bytes(), b'\n')
}

fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}

fn count_chars(content: &str) -> usize {
    content.chars().count()
}

fn count_bytes(content: &str) -> usize {
    content.bytes().count()
}


#[cfg(test)]
mod tests {
    use super::*;

    const SENTENCE1: &'static str = "\
Still Life – Apples and Jar, a still life by the Scottish post-impressionist Samuel Peploe completed c.
1912–1916 and now held at the Art Gallery of New South Wales.
Peploe (1871–1935) was one of the group of four painters that became known as the Scottish Colourists; the other colourists were John Duncan Fergusson, Francis Cadell and Leslie Hunter.
";

    const SENTENCE2: &'static str = "\
The will of the Swedish inventor Alfred Nobel established the prizes in 1895.
The prizes in Chemistry, Literature, Peace, Physics, and Physiology or Medicine were first awarded in 1901.
The related Nobel Memorial Prize in Economic Sciences was established by Sweden's central bank in 1968.
Medals made before 1980 were struck in 23 carat gold, and later from 18 carat green gold plated with a 24 carat gold coating.
Between 1901 and 2015, the Nobel Prizes and the Prize in Economic Sciences were awarded 573 times to 900 people and organisations.
With some receiving the Nobel Prize more than once, this makes a total of 23 organisations, and 870 individuals—of whom 48 were women.
";

    fn count_stat_with_bytes(bytes: usize) -> CountStat {
        let mut count_stat = CountStat::empty();
        count_stat.bytes = bytes;
        count_stat
    }

    #[test]
    fn test_count_lines() {
        assert_eq!(super::count_lines(SENTENCE1), 3);
        assert_eq!(super::count_lines(SENTENCE2), 6);
    }

    #[test]
    fn test_count_words() {
        assert_eq!(super::count_words(SENTENCE1), 58);
        assert_eq!(super::count_words(SENTENCE2), 114);
    }

    #[test]
    fn test_count_chars() {
        assert_eq!(super::count_chars(SENTENCE1), 352);
        assert_eq!(super::count_chars(SENTENCE2), 682);
    }

    #[test]
    fn test_count_bytes() {
        assert_eq!(super::count_bytes(SENTENCE1), 358);
        assert_eq!(super::count_bytes(SENTENCE2), 684);
    }

    #[test]
    fn test_max_field_width() {
        let count_stat = count_stat_with_bytes(255);
        assert_eq!(count_stat.max_field_width(), 3);

        let count_stat = count_stat_with_bytes(2389238);
        assert_eq!(count_stat.max_field_width(), 7);

        let count_stat = count_stat_with_bytes(0);
        assert_eq!(count_stat.max_field_width(), 1);
    }
}
