#![deny(missing_docs)]

//!
//! The `rsef-rs` crate provides functionality to download and parse RSEF listings.
//!
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

#[cfg(feature = "download")]
pub mod download;

#[cfg(feature = "download")]
pub use crate::download::*;

/// Represents either a Version, Summary or Record line.
#[derive(Debug, Clone)]
pub enum Line {
    /// Represents a version line in an RSEF listing.
    Version(Version),

    /// Represents a summary line in an RSEF listing. States statistics on an Internet Resource.
    Summary(Summary),

    /// Represents an individual record on a specific Internet Resource.
    Record(Record),
}

/// Represents the different number of Internet resource types.
#[derive(Debug, Clone)]
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

/// Converts a string to a Type.
impl From<&str> for Type {
    fn from(value: &str) -> Self {
        let string = value.to_lowercase();

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
#[derive(Debug, Clone)]
pub struct Summary {
    /// The registry that this record belongs to.
    pub registry: String,

    /// Type of Internet number resource represented in this record.
    pub res_type: Type,

    /// Sum of the number of record lines of this type in the file.
    pub count: u32,
}

/// Represents an RSEF version line.
#[derive(Debug, Clone)]
pub struct Version {
    /// The version of the RIR Statistics Exchange Format.
    pub version: f64,

    ///  The registry that this record belongs to.
    pub registry: String,

    /// Serial number of this file (within the creating RIR series).
    pub serial: String,

    ///  Number of records in file, excluding blank lines, summary lines, the version line and comments.
    pub records: u32,

    ///  Start date of time period, in yyyymmdd format.
    pub start_date: String,

    /// End date of period, in yyyymmdd format.
    pub end_date: String,

    ///  Offset from UTC (+/- hours) of local RIR producing file.
    pub utc_offset: String,
}

/// Represents an record about either an ASN, IPv4 prefix or IPv6 prefix.
#[derive(Debug, Clone)]
pub struct Record {
    /// The registry that this record belongs to.
    pub registry: String,

    ///  ISO 3166 2-letter code of the organization to which the allocation or assignment was made.
    pub organization: String,

    ///  Type of Internet number resource represented in this record.
    pub res_type: Type,

    /// For IPv4 or IPv6, the base of the IP prefix. For asn the ASN number.
    pub start: String,

    /// For IPv4 the amount of hosts in this prefix. For ipv6 the CIDR prefix. For asn the amount of ASN numbers.
    pub value: u32,

    /// The date on which this allocation was made in YYYYMMDD format.
    pub date: String,

    /// Type of allocation from the set.
    pub status: String,

    /// The ID handle of this object. Often a reference to an organisation (which is also related to an AS)
    pub id: String,
}

///
/// Reads all the RSEF entries found in a stream and returns a Vec of RSEF entries.
///
pub fn read_all(read: impl Read) -> Result<Vec<Line>, std::io::Error> {
    let mut stream = BufReader::new(read);
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
        if line.starts_with('#') {
            continue;
        }

        // Divide the line into fields.
        let fields = line.split('|').collect::<Vec<_>>();

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
