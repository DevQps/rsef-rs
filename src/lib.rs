#![deny(missing_docs)]

//! The `rsef-rs` crate provides functionality to download and parse RSEF listings.
//!
//! # Examples
//!
//! ## Downloading and parsing an RSEF Listing
//! ```
//! use std::fs::File;
//! use std::io::Cursor;
//! use std::io::Read;
//! use std::io::BufReader;
//!
//! use rsef_rs::{Registry, Line, Reader, download};
//!
//! fn main() {
//!     // Friday 1 February 2019 21:22:48
//!     let timestamp = 1549056168;
//!     let mut listing = download(Registry::RIPE, timestamp).unwrap();
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
use std::convert::From;
use std::io::BufRead;
use std::io::BufReader;
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

/// Downloads the RSEF listings of a specific Regional Internet Registry at a specific moment.
/// The timestamp should be an UNIX Epoch. Returns a decoded stream that can be read from.
pub fn download(
    registry: Registry,
    timestamp: i64,
) -> Result<Box<dyn Read>, Box<std::error::Error>> {
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

    match registry {
        Registry::AFRINIC => {
            let url = format!(
                "https://ftp.afrinic.net/pub/stats/afrinic/{}/delegated-afrinic-extended-{}{}{}",
                year, year, month, day
            );
            return Ok(Box::new(reqwest::get(url.as_str())?));
        }
        Registry::APNIC => {
            let url = format!(
                "https://ftp.apnic.net/stats/apnic/{}/delegated-apnic-extended-{}{}{}.gz",
                year, year, month, day
            );
            return Ok(Box::new(Decoder::new(reqwest::get(url.as_str())?)?));
        }
        Registry::ARIN => {
            // TODO: Works only for dates after 2017.
            let url = format!(
                "https://ftp.arin.net/pub/stats/arin/delegated-arin-extended-{}{}{}",
                year, month, day
            );
            return Ok(Box::new(reqwest::get(url.as_str())?));
        }
        Registry::LACNIC => {
            let url = format!(
                "https://ftp.lacnic.net/pub/stats/lacnic/delegated-lacnic-extended-{}{}{}",
                year, month, day
            );
            return Ok(Box::new(reqwest::get(url.as_str())?));
        }
        Registry::RIPE => {
            let url = format!(
                "https://ftp.ripe.net/pub/stats/ripencc/{}/delegated-ripencc-extended-{}{}{}.bz2",
                year, year, month, day
            );
            return Ok(Box::new(BzDecoder::new(reqwest::get(url.as_str())?)));
        }
    };
}

/// Represents either a Version, Summary or Record line.
#[derive(Debug)]
pub enum Line {
    /// Represents a version line in an RSEF listing.
    Version(Version),

    /// Represents a summary line in an RSEF listing. States statistics on an Internet Resource.
    Summary(Summary),

    /// Represents an individual record on a specific Internet Resource.
    Record(Record),
}

/// Represents the different number of Internet resource types.
#[derive(Debug)]
pub enum Type {
    /// Autonomous System Number
    ASN,

    /// Internet Protocol version 4
    IPv4,

    /// Internet Protocol version 6
    IPv6,

    /// Unknown Internet Resource
    Unknown,
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        let string = value.to_string().to_lowercase();

        if string.eq("asn") {
            Type::ASN
        } else if string.eq("ipv4") {
            Type::IPv4
        } else if string.eq("ipv6") {
            Type::IPv6
        } else {
            Type::Unknown
        }
    }
}

/// Represents an RSEF summary line.
#[derive(Debug)]
pub struct Summary {
    /** The registry that this record belongs to. */
    pub registry: String,

    /** Type of Internet number resource represented in this record.*/
    pub res_type: Type,

    /** Sum of the number of record lines of this type in the file. */
    pub count: u32,
}

/// Represents an RSEF version line.
#[derive(Debug)]
pub struct Version {
    /** The version of the RIR Statistics Exchange Format. */
    pub version: f64,

    /** The registry that this record belongs to. */
    pub registry: String,

    /** Serial number of this file (within the creating RIR series). */
    pub serial: String,

    /** Number of records in file, excluding blank lines, summary lines, the version line and comments. */
    pub records: u32,

    /** Start date of time period, in yyyymmdd format. */
    pub start_date: String,

    /** End date of period, in yyyymmdd format. */
    pub end_date: String,

    /** Offset from UTC (+/- hours) of local RIR producing file. */
    pub utc_offset: String,
}

/// Represents an record about either an ASN, IPv4 prefix or IPv6 prefix.
#[derive(Debug)]
pub struct Record {
    /** The registry that this record belongs to. */
    pub registry: String,

    /** ISO 3166 2-letter code of the organization to which the allocation or assignment was made. */
    pub organization: String,

    /** Type of Internet number resource represented in this record. */
    pub res_type: Type,

    /** For IPv4 or ipv6, the base of the IP prefix. For asn the ASN number. */
    pub start: String,

    /** For IPv4 the amount of hosts in this prefix. For ipv6 the CIDR prefix. For asn the amount of ASN numbers. */
    pub value: u32,

    /** The date on which this allocation was made in YYYYMMDD format. */
    pub date: String,

    /** Type of allocation from the set. */
    pub status: String,

    /** The ID handle of this object. Often a reference to an organisation (which is also related to an AS) */
    pub id: String,
}

/// The BGPReader can read BGP messages from a BGP-formatted stream.
pub struct Reader<T>
where
    T: Read,
{
    /// The stream from which BGP messages will be read.
    pub stream: T,
}

impl<T> Reader<T>
where
    T: Read,
{
    ///
    /// Reads all the RSEF entries found in a stream and returns a Vec of RSEF entries.
    ///
    pub fn read_all(&mut self) -> Result<Vec<Line>, std::io::Error> {
        let mut stream = BufReader::new(&mut self.stream);
        let mut lines: Vec<Line> = Vec::with_capacity(1000);

        loop {
            let mut line = String::new();
            let len = stream.read_line(&mut line)?;

            if len == 0 {
                break;
            }

            // Remove the trailing whitespaces and newline characters
            line.pop();

            // Skip the comments.
            if line.starts_with("#") {
                continue;
            }

            // Divide the line into fields.
            let fields = line.split("|").collect::<Vec<_>>();

            // Check if line is a version.
            if fields[0].chars().all(|x| x.is_digit(10) || x.eq(&'.')) {
                lines.push(Line::Version(Version {
                    version: fields[0].parse::<f64>().unwrap(),
                    registry: fields[1].to_string(),
                    serial: fields[2].to_string(),
                    records: fields[3].parse::<u32>().unwrap(),
                    start_date: fields[4].to_string(),
                    end_date: fields[5].to_string(),
                    utc_offset: fields[6].to_string(),
                }));
                continue;
            }

            // Check if line is a summary.
            if fields[5].to_string().eq("summary") {
                lines.push(Line::Summary(Summary {
                    registry: fields[0].to_string(),
                    res_type: Type::from(fields[2]),
                    count: fields[4].parse::<u32>().unwrap(),
                }));
                continue;
            }

            lines.push(Line::Record(Record {
                registry: fields[0].to_string(),
                organization: fields[1].to_string(),
                res_type: Type::from(fields[2]),
                start: fields[3].to_string(),
                value: fields[4].parse::<u32>().unwrap(),
                date: fields[5].to_string(),
                status: fields[6].to_string(),
                id: if fields.len() > 7 {
                    fields[7].to_string()
                } else {
                    "".to_string()
                },
            }));
        }

        Ok(lines)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use crate::*;

    #[test]
    fn test_download() {
        // Friday 1 February 2019 21:22:48
        let mut listings: Vec<Box<Read>> = Vec::with_capacity(5);
        listings.push(download(Registry::AFRINIC, 1549056168).unwrap());
        listings.push(download(Registry::APNIC, 1549056168).unwrap());
        listings.push(download(Registry::ARIN, 1549056168).unwrap());
        listings.push(download(Registry::LACNIC, 1549056168).unwrap());
        listings.push(download(Registry::RIPE, 1549056168).unwrap());

        for stream in listings {
            let mut reader = Reader { stream };
            let records = reader.read_all().unwrap();

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
