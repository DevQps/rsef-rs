//!
//! When the `download` feature is enabled, functionality is provided that allows a user to download
//! RSEF listings from a specific date and parse them.
//!
//! Additional Note:
//! - RIPE and LACNIC latest listings are from yesterday, whereas APNIC, AFRINIC and ARIN provide listings for today. Remember this when attempting to retrieve RSEF contents.
//! - It will attempt to download the 'extended' listings with opaque identifiers.
//!
//! # Examples
//!
//! ## Downloading and parsing an RSEF Listing
//! ```rust
//! use rsef_rs::{Registry, Line};
//! use tokio::runtime::Runtime;
//!
//! // Friday 1 February 2019 21:22:48
//! let timestamp = 1_549_056_168;
//!
//! // Download the listing from RIPE.
//! let future = Registry::download(Registry::RIPE, timestamp);
//!
//! // Tokio's block_on method is used to wait for future to complete.
//! // Use `future.await.unwrap()` in async contexts.
//! let rt = Runtime::new().expect("Failure to construct Tokio runtime.");
//! let stream = rt.block_on(future).unwrap();
//!
//! // Parse the stream into records.
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
use chrono::NaiveDateTime;
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
    pub async fn download(
        registry: Registry,
        timestamp_seconds: i64,
    ) -> Result<Box<dyn Read>, Box<dyn Error>> {
        let timestamp = NaiveDateTime::from_timestamp(timestamp_seconds, 0);
        let today = chrono::Utc::now().naive_utc().date();

        if today < timestamp.date() {
            return Err("Error: The date provided to Registry::download is in the future.".into());
        }

        match registry {
            // AFRINIC does have a listing for "today".
            Registry::AFRINIC => {
                let url = if timestamp.date() == chrono::Utc::now().naive_utc().date() {
                    timestamp
                        .format("https://ftp.afrinic.net/pub/stats/afrinic/delegated-afrinic-extended-%Y%m%d")
                        .to_string()
                } else {
                    timestamp
                        .format("https://ftp.afrinic.net/pub/stats/afrinic/%Y/delegated-afrinic-extended-%Y%m%d")
                        .to_string()
                };

                let response = reqwest::get(url.as_str())?;
                Ok(Box::new(response))
            }
            // APNIC does have a listing for 'today'.
            Registry::APNIC => {
                if timestamp.date() == chrono::Utc::now().naive_utc().date() {
                    let url = timestamp
                        .format("https://ftp.apnic.net/stats/apnic/delegated-apnic-extended-%Y%m%d")
                        .to_string();

                    let response = reqwest::get(url.as_str())?;
                    Ok(Box::new(response))
                } else {
                    let url = timestamp
                        .format("https://ftp.apnic.net/stats/apnic/%Y/delegated-apnic-extended-%Y%m%d.gz")
                        .to_string();

                    let response = reqwest::get(url.as_str())?;
                    Ok(Box::new(Decoder::new(response)?))
                }
            }

            // ARIN does have a listing for "today".
            Registry::ARIN => {
                let url = timestamp
                    .format("https://ftp.arin.net/pub/stats/arin/delegated-arin-extended-%Y%m%d")
                    .to_string();

                let response = reqwest::get(url.as_str())?;
                Ok(Box::new(response))
            }
            // LACNIC does not have a listing for 'today'.
            Registry::LACNIC => {
                if timestamp.date() == today {
                    return Err("Error: LACNIC does not provide RSEF listings for today.".into());
                }

                let url = timestamp
                    .format(
                        "https://ftp.lacnic.net/pub/stats/lacnic/delegated-lacnic-extended-%Y%m%d",
                    )
                    .to_string();

                let response = reqwest::get(url.as_str())?;
                Ok(Box::new(response))
            }

            // RIPE does not have a listing for "today".
            Registry::RIPE => {
                if timestamp.date() == today {
                    return Err("Error: RIPE does not provide RSEF listings for today.".into());
                }

                let url = timestamp
                    .format("https://ftp.ripe.net/pub/stats/ripencc/%Y/delegated-ripencc-extended-%Y%m%d.bz2")
                    .to_string();

                let response = reqwest::get(url.as_str())?;
                Ok(Box::new(BzDecoder::new(response)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use crate::Registry;

    #[tokio::test]
    async fn test_download() {
        // Test it with the date of yesterday as RIPE and LACNIC do not provide listings for today.
        const DAY: i64 = 60 * 60 * 24;
        let timestamp = chrono::Utc::now().timestamp() - DAY;

        println!("Downloading from AFRINIC");
        let stream = Registry::download(Registry::AFRINIC, timestamp)
            .await
            .unwrap();
        let _ = crate::read_all(stream).unwrap();

        println!("Downloading from APNIC");
        let stream = Registry::download(Registry::APNIC, timestamp)
            .await
            .unwrap();
        let _ = crate::read_all(stream).unwrap();

        println!("Downloading from ARIN");
        let stream = Registry::download(Registry::ARIN, timestamp).await.unwrap();
        let _ = crate::read_all(stream).unwrap();

        println!("Downloading from LACNIC");
        let stream = Registry::download(Registry::LACNIC, timestamp)
            .await
            .unwrap();
        let _ = crate::read_all(stream).unwrap();

        println!("Downloading from RIPE");
        let stream = Registry::download(Registry::RIPE, timestamp).await.unwrap();
        let _ = crate::read_all(stream).unwrap();
    }
}
