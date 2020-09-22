use practicaltimestamp::{Timestamp, util};
use time::OffsetDateTime;

const MIN_TIMESTAMP: i64 = Timestamp::MIN.unix_timestamp();
const MAX_TIMESTAMP: i64 = Timestamp::MAX.unix_timestamp();

#[test]
fn test_year_month_day() {
    for i in (MIN_TIMESTAMP..=MAX_TIMESTAMP).step_by(util::SECONDS_PER_DAY as usize) {
        let (y, m, d) = OffsetDateTime::from_unix_timestamp(i).date().as_ymd();
        let ts = Timestamp::checked_from_year_month_day(y as u16, m, d).unwrap();
        assert_eq!(ts.as_year_month_day(), (y as u16, m, d));
        assert_eq!(ts.unix_timestamp(), i);
    }
    assert_eq!(Timestamp::checked_from_year_month_day(1969, 12, 31), None);
    assert_eq!(Timestamp::checked_from_year_month_day(10000, 1, 2), None);
}

#[test]
fn test_year_ordinal() {
    for i in (MIN_TIMESTAMP..=MAX_TIMESTAMP).step_by(util::SECONDS_PER_DAY as usize) {
        let (y, o) = OffsetDateTime::from_unix_timestamp(i).date().as_yo();
        let ts = Timestamp::checked_from_year_ordinal(y as u16, o).unwrap();
        assert_eq!(ts.as_year_ordinal(), (y as u16, o));
        assert_eq!(ts.unix_timestamp(), i);
    }
    assert_eq!(Timestamp::checked_from_year_ordinal(1969, 365), None);
    assert_eq!(Timestamp::checked_from_year_ordinal(10000, 2), None);
}
