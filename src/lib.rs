//! A library for finding the country of origin for an ICAO hex ID.
//!
//! # Example
//!
//! ```
//! use aircraft_icao_country::Allocs;
//!
//! let allocs = Allocs::new();
//! assert_eq!(allocs.find_from_hex("a67bd3"), Some("United States"));
//! ```
//!
//! # References
//!
//! * [Registration Of Aircraft Addresses With Mode S Transponders, Appendix A](https://www.icao.int/Meetings/AMC/MA/NACC_DCA03_2008/naccdca3wp05.pdf)
//!
//! # License
//!
//! This library is licensed under the MIT license.

/// A struct representing the allocation of ICAO hex IDs to countries.
#[derive(Debug)]
pub struct Allocs {
    pub ranges: Vec<Range>,
}

impl Allocs {
    /// Create a new `Allocs` instance by parsing the country ranges CSV file.
    pub fn new() -> Allocs {
        Allocs {
            ranges: parse_range_csv(COUNTRY_RANGES_CSV),
        }
    }

    /// Find the country for the given ICAO hex ID (as a `u32`).
    pub fn find(&self, icao: u32) -> Option<&str> {
        for range in self.ranges.iter() {
            if icao >= range.start && icao <= range.end {
                return Some(&range.country);
            }
        }
        None
    }

    /// Find the country for the given ICAO hex ID (as a `&str`).
    pub fn find_from_hex(&self, hex: &str) -> Option<&str> {
        // Return None if the hex string is not a valid u32.
        let ip = u32::from_str_radix(hex, 16).ok()?;
        self.find(ip)
    }
}

/// Implement the `Default` trait for `Allocs` struct.
impl Default for Allocs {
    fn default() -> Self {
        Self::new()
    }
}

/// A struct representing a range of ICAO hex IDs allocated to a specific country.
#[derive(Debug)]
pub struct Range {
    pub start: u32,
    pub end: u32,
    pub country: String,
}

/// Parses a CSV string and return a `Vec<Range>`.
pub fn parse_range_csv(input: &str) -> Vec<Range> {
    let mut ranges = Vec::new();
    let mut lines = input.lines();

    // Skip the first line (header)
    lines.next();

    for line in lines {
        let mut parts = line.split(',');
        let start = u32::from_str_radix(parts.next().unwrap(), 16).unwrap();
        let end = u32::from_str_radix(parts.next().unwrap(), 16).unwrap();
        let country = parts.next().unwrap().to_string();
        ranges.push(Range {
            start,
            end,
            country,
        });
    }
    ranges
}

// Include the countryranges.csv file as a string.
const COUNTRY_RANGES_CSV: &str = include_str!("countryranges.csv");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test the `find_country_from_hex()` method for various cases.
    fn test_find_country_from_hex() {
        let allocs = Allocs::new();
        assert_eq!(allocs.find_from_hex("a67bd3"), Some("United States"));
        assert_eq!(allocs.find_from_hex("86d682"), Some("Japan"));
        assert_eq!(allocs.find_from_hex("c04d3f"), Some("Canada"));
        assert_eq!(allocs.find_from_hex("acfbc5"), Some("United States"));
        assert_eq!(allocs.find_from_hex("00fee3"), Some("South Africa"));
        assert_eq!(allocs.find_from_hex("00FEE3"), Some("South Africa"));
        assert_eq!(allocs.find_from_hex("000001"), None);
        assert_eq!(allocs.find_from_hex("zzzzz"), None);
        assert_eq!(allocs.find_from_hex(""), None);
    }
}
