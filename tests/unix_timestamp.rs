use practicaltimestamp::{UnixTimestamp, util};

const _MIN_TIMESTAMP: i64 = UnixTimestamp::MIN.unix_timestamp();
const _MAX_TIMESTAMP: i64 = UnixTimestamp::MAX.unix_timestamp();

#[test]
#[cfg(feature = "std")]
fn now() {
    let _ = UnixTimestamp::now();
}

#[test]
fn unix_timestamp() {
    assert_eq!(UnixTimestamp::checked_from_unix_timestamp(-1), None);
    assert_eq!(UnixTimestamp::from_unix_timestamp(-1).unwrap(), UnixTimestamp::MIN);
    assert_eq!(UnixTimestamp::checked_from_unix_timestamp(0).unwrap(), UnixTimestamp::MIN);
    assert_eq!(UnixTimestamp::from_unix_timestamp(util::SECONDS_PER_DAY).unwrap().unix_timestamp(), util::SECONDS_PER_DAY);
    assert_eq!(UnixTimestamp::checked_from_unix_timestamp(253_402_300_800).unwrap(), UnixTimestamp::MAX);
    assert_eq!(UnixTimestamp::from_unix_timestamp(253_402_300_801).unwrap(), UnixTimestamp::MAX);
    assert_eq!(UnixTimestamp::checked_from_unix_timestamp(253_402_300_801), None);
}

#[test]
fn midnight() {
    const SECONDS_SINCE_MIDNIGHT: i64 = 3_600;

    let ts = UnixTimestamp::from_unix_timestamp(SECONDS_SINCE_MIDNIGHT).unwrap();
    assert_eq!(ts.midnight(), UnixTimestamp::from_unix_timestamp(0).unwrap());
    assert_eq!(ts.seconds_since_midnight(), SECONDS_SINCE_MIDNIGHT);
}

#[test]
fn checked_add_sub_saturating_add_sub() {
    assert_eq!(UnixTimestamp::MIN.checked_add(i64::MIN), None);
    assert_eq!(UnixTimestamp::MIN.checked_add(i64::MAX), None);
    assert_eq!(UnixTimestamp::MAX.checked_sub(i64::MIN), None);
    assert_eq!(UnixTimestamp::MAX.checked_sub(i64::MAX), None);
    assert_eq!(UnixTimestamp::MIN.saturating_add(i64::MIN), UnixTimestamp::MIN);
    assert_eq!(UnixTimestamp::MIN.saturating_add(i64::MAX), UnixTimestamp::MAX);
    assert_eq!(UnixTimestamp::MAX.saturating_sub(i64::MIN), UnixTimestamp::MAX);
    assert_eq!(UnixTimestamp::MAX.saturating_sub(i64::MAX), UnixTimestamp::MIN);
}

#[test]
fn year_month_day() {
    const TEST_CASES: &[((u16, u8, u8), i64)] = &[
        ((1970, 1, 1), 0),
        ((2020, 10, 7), 1_602_028_800),
        ((2020, 10, 8), 1_602_115_200),
        ((2020, 10, 9), 1_602_201_600),
        ((10000, 1, 1), 253_402_300_800),
    ];

    for &((y, m, d), ut) in TEST_CASES {
        assert_eq!(UnixTimestamp::from_year_month_day(y, m, d).unwrap().unix_timestamp(), ut);
        assert_eq!(UnixTimestamp::from_unix_timestamp(ut).unwrap().to_year_month_day(), (y, m, d));
    }
    assert_eq!(UnixTimestamp::checked_from_year_month_day(1969, 12, 31), None);
    assert_eq!(UnixTimestamp::checked_from_year_month_day(10000, 1, 2), None);
    assert_eq!(UnixTimestamp::from_year_month_day(u16::MIN, u8::MIN, u8::MIN).unwrap(), UnixTimestamp::MIN);
    assert_eq!(UnixTimestamp::from_year_month_day(u16::MAX, u8::MAX, u8::MAX).unwrap(), UnixTimestamp::MAX);
}

#[test]
fn year_ordinal() {
    const TEST_CASES: &[((u16, u16), i64)] = &[
        ((1970, 1), 0),
        ((2020, 281), 1_602_028_800),
        ((2020, 282), 1_602_115_200),
        ((2020, 283), 1_602_201_600),
        ((10000, 1), 253_402_300_800),
    ];

    for &((y, o), ut) in TEST_CASES {
        assert_eq!(UnixTimestamp::from_year_ordinal(y, o).unwrap().unix_timestamp(), ut);
        assert_eq!(UnixTimestamp::from_unix_timestamp(ut).unwrap().to_year_ordinal(), (y, o));
    }
    assert_eq!(UnixTimestamp::checked_from_year_ordinal(1969, 365), None);
    assert_eq!(UnixTimestamp::checked_from_year_ordinal(10000, 2), None);
    assert_eq!(UnixTimestamp::from_year_ordinal(u16::MIN, u16::MIN).unwrap(), UnixTimestamp::MIN);
    assert_eq!(UnixTimestamp::from_year_ordinal(u16::MAX, u16::MAX).unwrap(), UnixTimestamp::MAX);
}

#[test]
fn julian_day_number() {
    const TEST_CASES: &[(i32, i64)] = &[
        (2_440_588, 0),
        (2_459_130, 1_602_028_800),
        (2_459_131, 1_602_115_200),
        (2_459_132, 1_602_201_600),
        (5_373_485, 253_402_300_800),
    ];

    for &(jdn, ut) in TEST_CASES {
        assert_eq!(UnixTimestamp::from_julian_day_number(jdn).unwrap().unix_timestamp(), ut);
        assert_eq!(UnixTimestamp::from_unix_timestamp(ut).unwrap().julian_day_number(), jdn);
    }
    assert_eq!(UnixTimestamp::checked_from_julian_day_number(2_440_587), None);
    assert_eq!(UnixTimestamp::checked_from_julian_day_number(5_373_486), None);
    assert_eq!(UnixTimestamp::from_julian_day_number(i32::MIN).unwrap(), UnixTimestamp::MIN);
    assert_eq!(UnixTimestamp::from_julian_day_number(i32::MAX).unwrap(), UnixTimestamp::MAX);
}

#[test]
fn weekday() {
    const TEST_CASES: &[(i64, util::Weekday)] = &[
        (0, util::Weekday::THURSDAY),
        (1_602_028_800, util::Weekday::WEDNESDAY),
        (1_602_115_200, util::Weekday::THURSDAY),
        (1_602_201_600, util::Weekday::FRIDAY),
        (253_402_300_800, util::Weekday::SATURDAY),
    ];

    for &(ut, wd) in TEST_CASES {
        assert_eq!(UnixTimestamp::from_unix_timestamp(ut).unwrap().weekday(), wd);
    }
}
