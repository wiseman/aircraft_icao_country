pub struct Allocs {
    allocs: Vec<Range>
}

impl Allocs {
    pub fn new() -> Allocs {
        Allocs {
            allocs: parse_range_csv(COUNTRY_RANGES_CSV),
        }
    }

    pub fn find(&self, ip: u32) -> Option<&str> {
        for range in self.allocs.iter() {
            if ip >= range.start && ip <= range.end {
                return Some(&range.country);
            }
        }
        None
    }

    pub fn find_from_hex(&self, hex: &str) -> Option<&str> {
        // Return None if the hex string is not a valid u32.
        let ip = u32::from_str_radix(hex, 16).ok()?;
        self.find(ip)
    }
}

impl Default for Allocs {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Range {
    pub start: u32,
    pub end: u32,
    pub country: String,
}

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
