use super::Timestamp;

pub const SECONDS_PER_DAY: i64 = 86_400;
pub const UNIX_EPOCH_JULIAN_DAY_NUMBER: i32 = 2_440_588;

// Maybe should make weekday into a newtype or enum
pub const MONDAY: u8 = 0;
pub const TUESDAY: u8 = 1;
pub const WEDNESDAY: u8 = 2;
pub const THURSDAY: u8 = 3;
pub const FRIDAY: u8 = 4;
pub const SATURDAY: u8 = 5;
pub const SUNDAY: u8 = 6;

// 1/1/1970 @ 12:00:00AM UTC to 1/1/10000 @ 12:00:00AM UTC
pub(super) const fn is_supported_unix_timestamp(timestamp: i64) -> bool {
    timestamp as u64 <= Timestamp::MAX.unix_timestamp() as u64
}

pub const fn is_valid_year_month_day(year: u16, month: u8, day: u8) -> bool {
    1 <= month && month <= 12 && 1 <= day && day <= days_in_year_month(year, month)
}

pub const fn is_valid_year_ordinal(year: u16, ordinal: u16) -> bool {
    1 <= ordinal && ordinal <= days_in_year(year)
}

pub const fn is_leap_year(year: u16) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

pub const fn days_in_year_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => 28 + is_leap_year(year) as u8,
        _ => 0,
    }
}

pub const fn days_in_year(year: u16) -> u16 {
    365 + is_leap_year(year) as u16
}
