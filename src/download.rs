//!
//! When the `download` feature is enabled, functionality is provided that allows a user to download
//! RSEF listings from a specific date and parse them.
//!
//! # Examples
//!
//! ## Downloading and parsing an RSEF Listing
//! ```
//! use std::io::Cursor;
//! use std::io::Read;
//! use std::io::BufReader;
//!
//! use rsef_rs::{Registry, Line, Reader, download};
//!
//! fn main() {
//!     // Friday 1 February 2019 21:22:48
//!     let timestamp = 1549056168;
//!     let registry = Registry::RIPE;
//!     let mut listing = registry.download(timestamp).unwrap();
//!
//!     let mut reader = Reader{stream: listing};
//!     let records = reader.read_all().unwrap();
//!
//!     for x in records {
//!         match x {
//!             Line::Version(x) => println!("Version: {:?}", x),
//!             Line::Summary(x) => println!("Summary: {:?}", x),
//!             Line::Record(x) => println!("Record: {:?}", x),
//!         }
//!     }
//! }
//! ```

use bzip2::read::BzDecoder;
use chrono::DateTime;
use chrono::Datelike;
use chrono::NaiveDateTime;
use chrono::Utc;
use libflate::gzip::Decoder;

use std::error::Error;
use std::io::Read;

/// Represents a Regional Internet Registry (RIR).
#[allow(missing_docs)]
#[derive(Debug)]
pub enum Registry {
    AFRINIC,
    APNIC,
    ARIN,
    LACNIC,
    RIPE,
}

impl Registry {
    /// Downloads the RSEF listings of a specific Regional Internet Registry at a specific moment.
    /// The timestamp should be an UNIX Epoch. Returns a decoded stream that can be read from.
    pub fn download(&self, timestamp: i64) -> Result<Box<dyn Read>, Box<dyn Error>> {
        let datetime: DateTime<Utc> =
            DateTime::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
        let year = datetime.year();
        let month = if datetime.month() < 10 {
            format!("0{}", datetime.month())
        } else {
            format!("{}", datetime.month())
        };
        let day = if datetime.month() < 10 {
            format!("0{}", datetime.day())
        } else {
            format!("{}", datetime.day())
        };

        match self {
            Registry::AFRINIC => {
                let url = format!(
                    "https://ftp.afrinic.net/pub/stats/afrinic/{}/delegated-afrinic-extended-{}{}{}",
                    year, year, month, day
                );

                let response = reqwest::get(url.as_str())?;
                Ok(Box::new(response))
            }
            Registry::APNIC => {
                let url = format!(
                    "https://ftp.apnic.net/stats/apnic/{}/delegated-apnic-extended-{}{}{}.gz",
                    year, year, month, day
                );

                let response = reqwest::get(url.as_str())?;
                Ok(Box::new(Decoder::new(response)?))
            }
            Registry::ARIN => {
                let url = format!(
                    "https://ftp.arin.net/pub/stats/arin/delegated-arin-extended-{}{}{}",
                    year, month, day
                );

                let response = reqwest::get(url.as_str())?;
                Ok(Box::new(response))
            }
            Registry::LACNIC => {
                let url = format!(
                    "https://ftp.lacnic.net/pub/stats/lacnic/delegated-lacnic-extended-{}{}{}",
                    year, month, day
                );

                let response = reqwest::get(url.as_str())?;
                Ok(Box::new(response))
            }
            Registry::RIPE => {
                let url = format!(
                    "https://ftp.ripe.net/pub/stats/ripencc/{}/delegated-ripencc-extended-{}{}{}.bz2",
                    year, year, month, day
                );

                let response = reqwest::get(url.as_str())?;
                Ok(Box::new(BzDecoder::new(response)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use crate::*;

    #[test]
    fn test_download() {
        // Friday 1 February 2019 21:22:48
        let timestamp = 1_549_056_168;

        let mut listings: Vec<Box<dyn Read>> = Vec::with_capacity(5);

        println!("Downloading from AFRINIC");
        listings.push(Registry::AFRINIC.download(timestamp).unwrap());

        println!("Downloading from APNIC");
        listings.push(Registry::APNIC.download(timestamp).unwrap());

        println!("Downloading from ARIN");
        listings.push(Registry::ARIN.download(timestamp).unwrap());

        println!("Downloading from LACNIC");
        listings.push(Registry::LACNIC.download(timestamp).unwrap());

        println!("Downloading from RIPE");
        listings.push(Registry::RIPE.download(timestamp).unwrap());

        for stream in listings {
            let records = crate::read_all(stream).unwrap();

            for x in records {
                match x {
                    Line::Version(_) => continue,
                    Line::Summary(_) => continue,
                    Line::Record(_) => continue,
                }
            }
        }
    }
}
