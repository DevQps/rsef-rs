# RIR Statistics Exchange Format in Rust (rsef-rs)
[![Build Status](https://travis-ci.com/DevQps/rsef-rs.svg?branch=master)](https://travis-ci.com/DevQps/rsef-rs) [![codecov](https://codecov.io/gh/DevQps/rsef-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/DevQps/rsef-rs)

A library for downloading and parsing RIR Statistics Exchange Format (RSEF) listings in Rust.

## Features
rsef-rs optionally includes the `download` feature which allows you to download listings from Regional Internet Registries with a single statement.
In order to enable the `download` feature you can add the following to your dependencies section in your Cargo.toml:

```no_run
[dependencies]
rsef-rs = { version = "0.2", features = ["download"] }
```

## Examples & Documentation

**Downloading and parsing an RSEF Listing**

If you enabled the `download` feature, you can download listings as shown below:

```
use rsef_rs::{Registry, Line};

// Friday 1 February 2019 21:22:48
let timestamp = 1549056168;
let stream = Registry::RIPE.download(timestamp).unwrap();
let records = rsef_rs::read_all(stream).unwrap();

for x in records {
    match x {
        Line::Version(x) => println!("Version: {:?}", x),
        Line::Summary(x) => println!("Summary: {:?}", x),
        Line::Record(x) => println!("Record: {:?}", x),
    }
}
```

For examples and documentation look [here](https://docs.rs/rsef-rs/).