# RIR Statistics Exchange Format in Rust (rsef-rs)
[![Daily Scheduled Test](https://github.com/DevQps/rsef-rs/workflows/Scheduled/badge.svg)](https://github.com/DevQps/rsef-rs)
[![codecov](https://codecov.io/gh/DevQps/rsef-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/DevQps/rsef-rs)
[![Crates](https://img.shields.io/crates/v/rsef_rs.svg)](https://crates.io/crates/rsef-rs)

A library for downloading and parsing RIR Statistics Exchange Format (RSEF) listings in Rust.

## Features
The `download` feature, which is enabled by default, allows you to download extended listings from Regional Internet Registries with a single statement.

## Examples & Documentation
**Downloading and parsing an RSEF Listing**

If the `download` feature is enabled, you can download listings as shown below:

```
use rsef_rs::{Registry, Line};
use tokio::runtime::Runtime;

// Friday 1 February 2019 21:22:48
let timestamp = 1_549_056_168;

// Download the listing from RIPE.
let future = Registry::download(Registry::RIPE, timestamp);

// Tokio's block_on method is used to wait for future to complete.
// Use `future.await.unwrap()` in async contexts.
let rt = Runtime::new().expect("Failure to construct Tokio runtime.");
let stream = rt.block_on(future).unwrap();

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