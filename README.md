# PracticalTimestamp

This is yet another date and time library, but the aim is to specifically support a more limited range of dates to simplify implementation and to allow for some optimizations to date conversions.

PracticalTimestamp correctly handles dates and times between 1/1/1970 at 12:00:00AM UTC and 1/1/10000 at 12:00:00AM UTC.

Subseconds are not used and timezones are not considered, but support for these could be added by wrapping `UnixTimestamp` in another struct implementation that tracks them.

### Features

The only feature is `std` library support. It is enabled by default, and it allows for converting between `UnixTimestamp` and `std::time::SystemTime` types. The api is mostly compatible with `#![no_std]` as the only thing requiring the `std` library is `UnixTimestamp::now()`. The `std` feature can be disabled by specifying `default-features = false` in your dependencies.

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as MIT, without any additional
terms or conditions.
