#![feature(test)]

extern crate wc;
extern crate test;

use test::Bencher;
use wc::*;

#[bench]
fn bench_count_bytes(b: &mut Bencher) {
    let content = include_str!("words.txt");
    let options = Options {
        bytes: true,
        chars: false,
        words: false,
        lines: false,
    };

    b.iter(|| count(content, &options));
}

#[bench]
fn bench_count_chars(b: &mut Bencher) {
    let content = include_str!("words.txt");
    let options = Options {
        bytes: false,
        chars: true,
        words: false,
        lines: false,
    };

    b.iter(|| count(&content, &options));
}

#[bench]
fn bench_count_lines(b: &mut Bencher) {
    let content = include_str!("words.txt");
    let options = Options {
        bytes: false,
        chars: false,
        words: false,
        lines: true,
    };

    b.iter(|| count(&content, &options));
}

#[bench]
fn bench_count_words(b: &mut Bencher) {
    let content = include_str!("words.txt");
    let options = Options {
        bytes: false,
        chars: false,
        words: true,
        lines: false,
    };

    b.iter(|| count(&content, &options));
}
