use practicaltimestamp::util::Weekday;

#[test]
fn previous() {
    const TEST_CASES: &[(Weekday, Weekday)] = &[
        (Weekday::MONDAY, Weekday::SUNDAY),
        (Weekday::TUESDAY, Weekday::MONDAY),
        (Weekday::SATURDAY, Weekday::FRIDAY),
        (Weekday::SUNDAY, Weekday::SATURDAY),
    ];

    for &(later, earlier) in TEST_CASES {
        assert_eq!(later.previous(), earlier);
    }
}

#[test]
fn next() {
    const TEST_CASES: &[(Weekday, Weekday)] = &[
        (Weekday::MONDAY, Weekday::TUESDAY),
        (Weekday::TUESDAY, Weekday::WEDNESDAY),
        (Weekday::SATURDAY, Weekday::SUNDAY),
        (Weekday::SUNDAY, Weekday::MONDAY),
    ];

    for &(earlier, later) in TEST_CASES {
        assert_eq!(earlier.next(), later);
    }
}

#[test]
fn days_since() {
    const TEST_CASES: &[(Weekday, Weekday, u64)] = &[
        (Weekday::MONDAY, Weekday::MONDAY, 0),
        (Weekday::MONDAY, Weekday::TUESDAY, 6),
        (Weekday::MONDAY, Weekday::SUNDAY, 1),
        (Weekday::TUESDAY, Weekday::MONDAY, 1),
        (Weekday::SATURDAY, Weekday::SUNDAY, 6),
        (Weekday::SUNDAY, Weekday::MONDAY, 6),
        (Weekday::SUNDAY, Weekday::SATURDAY, 1),
    ];

    for &(later, earlier, days) in TEST_CASES {
        assert_eq!(later.days_since(earlier), days);
    }
}

#[test]
fn days_until() {
    const TEST_CASES: &[(Weekday, Weekday, u64)] = &[
        (Weekday::MONDAY, Weekday::MONDAY, 0),
        (Weekday::MONDAY, Weekday::TUESDAY, 1),
        (Weekday::MONDAY, Weekday::SUNDAY, 6),
        (Weekday::TUESDAY, Weekday::MONDAY, 6),
        (Weekday::SATURDAY, Weekday::SUNDAY, 1),
        (Weekday::SUNDAY, Weekday::MONDAY, 1),
        (Weekday::SUNDAY, Weekday::SATURDAY, 6),
    ];

    for &(earlier, later, days) in TEST_CASES {
        assert_eq!(earlier.days_until(later), days);
    }
}
