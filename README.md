# aircraft_icao_country

[![Crates.io](https://img.shields.io/crates/v/aircraft_icao_country.svg)](https://crates.io/crates/aircraft_icao_country)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A Rust library for finding the country of origin for an ICAO hex ID.

## Features

- Look up an aircraft's country of origin based on its ICAO hex ID.
- Based on tar1090's [flags.js](https://github.com/wiedehopf/tar1090/blob/master/html/flags.js)

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
aircraft_icao_country = "1.0.3"
```

## Usage

```rust
use aircraft_icao_country::Allocs;

let allocs = Allocs::new();
assert_eq!(allocs.find_from_hex("a67bd3"), Some("United States"));
```

For more examples and usage details, please refer to the
[documentation](https://docs.rs/aircraft_icao_country).

## License

This project is licensed under the MIT License. See the [LICENSE](/LICENSE) file
for more information.

## Author

John Wiseman
[jjwiseman@gmail.com](mailto:jjwiseman@gmail.com)/[lemonodor](https://twitter.com/lemonodor)

## Repository

[https://github.com/wiseman/aircraft_icao_country](https://github.com/wiseman/aircraft_icao_country)
