# RIR Statistics Exchange Format in Rust (rsef-rs)
[![Build Status](https://travis-ci.com/DevQps/rsef-rs.svg?branch=master)](https://travis-ci.com/DevQps/rsef-rs) [![codecov](https://codecov.io/gh/DevQps/rsef-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/DevQps/rsef-rs)

A library for downloading and parsing RIR Statistics Exchange Format (RSEF) listings in Rust.

## Examples & Documentation

**Downloading and parsing an RSEF Listing**
```
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::BufReader;

use rsef_rs::{Registry, Line, Reader, download};

fn main() {

    // Friday 1 February 2019 21:22:48
    let timestamp = 1549056168;
    let mut listing = download(Registry::RIPE, timestamp).unwrap();

    let mut reader = Reader{stream: listing};
    let records = reader.read_all().unwrap();

    for x in records {
        match x {
            Line::Version(x) => println!("Version: {:?}", x),
            Line::Summary(x) => println!("Summary: {:?}", x),
            Line::Record(x) => println!("Record: {:?}", x),
        }
    }
}
```

For examples and documentation look [here](https://docs.rs/rsef-rs/).