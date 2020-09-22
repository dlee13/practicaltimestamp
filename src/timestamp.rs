use super::{
    result,
    util,
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Timestamp(i64);

impl Timestamp {
    pub const MIN: Timestamp = Self::new(0);
    pub const MAX: Timestamp = Self::new(253_402_300_800);

    const fn new(value: i64) -> Self {
        Self(value)
    }

    const fn value(&self) -> i64 {
        self.0
    }

    #[cfg(feature = "std")]
    pub fn now() -> Self {
        super::std_support::system_time_now()
    }

    pub const fn checked_from_unix_timestamp(timestamp: i64) -> Option<Self> {
        Self::from_unix_timestamp(timestamp).ok()
    }

    pub const fn from_unix_timestamp(timestamp: i64) -> result::TimestampResult {
        if util::is_timestamp_in_supported_range(timestamp) {
            result::TimestampResult::TimestampOk(Self::new(timestamp))
        } else {
            result::TimestampResult::OverflowErr(timestamp)
        }
    }

    pub const fn unix_timestamp(&self) -> i64 {
        self.value()
    }

    pub const fn midnight(&self) -> Self {
        Self::new(self.unix_timestamp() - self.seconds_since_midnight())
    }

    pub const fn seconds_since_midnight(&self) -> i64 {
        (self.unix_timestamp() as u64 % util::SECONDS_PER_DAY as u64) as i64
    }
    
    pub const fn checked_add(&self, seconds: i64) -> Option<Self> {
        let timestamp = self.unix_timestamp().wrapping_add(seconds);
        Self::checked_from_unix_timestamp(timestamp)
    }

    pub const fn checked_sub(&self, seconds: i64) -> Option<Self> {
        let timestamp = self.unix_timestamp().wrapping_sub(seconds);
        Self::checked_from_unix_timestamp(timestamp)
    }

    // const in 1.47
    pub fn saturating_add(&self, seconds: i64) -> Self {
        let timestamp = self.unix_timestamp().saturating_add(seconds); // MSRV 1.47
        Self::from_unix_timestamp(timestamp).unwrap()
    }

    // const in 1.47
    pub fn saturating_sub(&self, seconds: i64) -> Self {
        let timestamp = self.unix_timestamp().saturating_sub(seconds); // MSRV 1.47
        Self::from_unix_timestamp(timestamp).unwrap()
    }

    pub const fn checked_from_year_month_day(year: u16, month: u8, day: u8) -> Option<Self> {
        if util::is_valid_year_month_day(year, month, day) {
            Self::from_year_month_day(year, month, day).ok()
        } else {
            None
        }
    }

    // Only valid for dates greater than or equal to 0000-3-1
    // [section 2.2.1](https://www.researchgate.net/publication/316558298_Date_Algorithms)
    pub const fn from_year_month_day(year: u16, month: u8, day: u8) -> result::TimestampResult {
        let (year, month, day) = if month < 3 {
            (year as i32 - 1, month as i32 + 12, day as i32)
        } else {
            (year as i32, month as i32, day as i32)
        };
        // f = (153 * month - 457) / 5
        let f = (979 * month - 2_918) >> 5;
        // Floor division would be needed instead to accurately calculate for dates before year 1
        let julian_day_number = day + f + 365 * year + year / 4 - year / 100 + year / 400 + 1_721_119;
        Self::from_julian_day_number(julian_day_number as u32)
    }

    // Only valid for dates greater than or equal to 1721118.25 (April 28.75 of the year zero)
    // [section 3.2.1/3.3.1](https://www.researchgate.net/publication/316558298_Date_Algorithms)
    pub const fn as_year_month_day(&self) -> (u16, u8, u8) {
        let julian_day_number = self.julian_day_number() as u64;
        let z = julian_day_number - 1_721_119;
        let h = 100 * z - 25;
        // Floor division would be needed instead to accurately calculate for dates before year 1
        let a = h / 3_652_425;
        let b = a - a / 4;
        let year = (100 * b + h) / 36_525;
        let c = b + z - 365 * year - year / 4;
        // month = (5 * c + 456) / 153
        let month = (535 * c + 48_950) >> 14;
        // f = (153 * month - 457) / 5
        let f = (979 * month - 2_918) >> 5;
        let day = c - f;
        let (year, month) = if month > 12 {
            (year + 1, month - 12)
        } else {
            (year, month)
        };
        (year as u16, month as u8, day as u8)
    }

    pub const fn checked_from_year_ordinal(year: u16, ordinal: u16) -> Option<Self> {
        if util::is_valid_year_ordinal(year, ordinal) {
            Self::from_year_ordinal(year, ordinal).ok()
        } else {
            None
        }
    }

    // I derived this algorithm based on the doy_from_month equation
    // [Computing day-of-year from month and day-of-month](http://howardhinnant.github.io/date_algorithms.html#days_from_civil)
    pub const fn from_year_ordinal(year: u16, ordinal: u16) -> result::TimestampResult {
        let ordinal = ordinal as u32;
        let last_day_of_february = 59 + util::is_leap_year(year) as u32;
        let (adj_ordinal, adj_month, month) = if ordinal > last_day_of_february {
            let adj_ordinal = ordinal - last_day_of_february;
            // adj_month = (10 * adj_ordinal - 5) / 306
            let adj_month = (1_071 * adj_ordinal - 535) >> 15;
            (adj_ordinal, adj_month, (adj_month + 3) as u8)
        } else {
            let adj_ordinal = ordinal + 306;
            // adj_month = (10 * adj_ordinal - 5) / 306
            let adj_month = (1_071 * adj_ordinal - 535) >> 15;
            (adj_ordinal, adj_month, (adj_month - 9) as u8)
        };
        // f = (306 * adj_month + 5) / 10
        let f = (979 * adj_month + 16) >> 5;
        let day = adj_ordinal - f;
        Self::from_year_month_day(year, month, day as u8)
    }

    // [Eliminating the Lookup Table](https://blog.reverberate.org/2020/05/12/optimizing-date-algorithms.html)
    pub const fn as_year_ordinal(&self) -> (u16, u16) {
        let (year, month, day) = self.as_year_month_day();
        let (month, day) = (month as u64, day as u64);
        // DayOfYear(adjusted_month) = (153 * adjusted_month + 2) / 5
        let ordinal = if month >= 3 {
            ((62_719 * (month - 3) + 769) >> 11) + day + 59 + util::is_leap_year(year) as u64
        } else {
            ((62_719 * (month + 9) + 769) >> 11) + day - 306
        };
        (year, ordinal as u16)
    }

    pub const fn checked_from_julian_day_number(julian_day_number: u32) -> Option<Self> {
        Self::from_julian_day_number(julian_day_number).ok()
    }

    pub const fn from_julian_day_number(julian_day_number: u32) -> result::TimestampResult {
        let timestamp = (julian_day_number as i64 - util::UNIX_EPOCH_JULIAN_DAY_NUMBER as i64) * util::SECONDS_PER_DAY;
        Self::from_unix_timestamp(timestamp)
    }
    
    pub const fn julian_day_number(&self) -> u32 {
        (self.unix_timestamp() as u64 / util::SECONDS_PER_DAY as u64) as u32 + util::UNIX_EPOCH_JULIAN_DAY_NUMBER
    }

    // This gives the weekday number where 0 represents Monday and 6 represents Sunday
    pub const fn number_days_from_monday(&self) -> u8 {
        (((self.unix_timestamp() as u64 / util::SECONDS_PER_DAY as u64) as u32 + 3) % 7) as u8
    }
}
