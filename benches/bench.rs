#![feature(test)]

extern crate rust_wc;
extern crate test;

use test::Bencher;
use rust_wc::*;

#[bench]
fn bench_count_bytes(b: &mut Bencher) {
    let content = include_str!("words.txt");
    let option  = WcOption {
        bytes: true,
        chars: false,
        words: false,
        lines: false,
    };

    b.iter(|| count(content, &option));
}

#[bench]
fn bench_count_chars(b: &mut Bencher) {
    let content = include_str!("words.txt");
    let option  = WcOption {
        bytes: false,
        chars: true,
        words: false,
        lines: false,
    };

    b.iter(|| count(&content, &option));
}

#[bench]
fn bench_count_lines(b: &mut Bencher) {
    let content = include_str!("words.txt");
    let option  = WcOption {
        bytes: false,
        chars: false,
        words: false,
        lines: true,
    };

    b.iter(|| count(&content, &option));
}

#[bench]
fn bench_count_words(b: &mut Bencher) {
    let content = include_str!("words.txt");
    let option  = WcOption {
        bytes: false,
        chars: false,
        words: true,
        lines: false,
    };

    b.iter(|| count(&content, &option));
}
