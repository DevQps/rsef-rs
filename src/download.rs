//!
//! When the `download` feature is enabled, functionality is provided that allows a user to download
//! RSEF listings from a specific date and parse them.
//!
//! # Examples
//!
//! ## Downloading and parsing an RSEF Listing
//! ```
//! use rsef_rs::{Registry, Line};
//!
//! // Friday 1 February 2019 21:22:48
//! let timestamp = 1549056168;
//! let stream = Registry::RIPE.download(timestamp).unwrap();
//! let records = rsef_rs::read_all(stream).unwrap();
//!
//! for x in records {
//!     match x {
//!         Line::Version(x) => println!("Version: {:?}", x),
//!         Line::Summary(x) => println!("Summary: {:?}", x),
//!         Line::Record(x) => println!("Record: {:?}", x),
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
    /// Only the year, month and day wll be used to select the listing for that day.
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
