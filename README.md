# rust-wc

`wc` command implemented by Rust programming language.

[![Build Status](https://travis-ci.org/hyone/rust-wc.svg?branch=master)](https://travis-ci.org/hyone/rust-wc)
[![Build status](https://ci.appveyor.com/api/projects/status/2ayn3tt7lb5tdo95/branch/master?svg=true)](https://ci.appveyor.com/project/hyone/rust-wc/branch/master)

## Usage

    Usage: rust-wc [options] ... [<file>...]
           rust-wc (-h | --help)
           rust-wc --version

Print newline, word, and byte counts for each `<file>`, and a total line if
more than one `<file>` is specified.  A word is a non-zero-length sequence of
characters delimited by white space.

With no `<file>`, or when `<file>` is `-`, read standard input.

### Options:

| name            | description                |
|:----------------|:---------------------------|
| `-c`, `--bytes` | print the byte counts      |
| `-h`, `--help`  | display this help and exit |
| `-l`, `--lines` | print the newline counts   |
| `-m`, `--chars` | print the character counts |
| `-w`, `--words` | print the word counts      |
| `--version`     | display version and exit   |
