use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use practicaltimestamp::UnixTimestamp;

const BENCH_CASES: &[UnixTimestamp] = &[
    UnixTimestamp::MIN,
    UnixTimestamp::from_year_month_day(2018, 6, 2).unwrap(),
    UnixTimestamp::from_year_month_day(2019, 4, 13).unwrap(),
    UnixTimestamp::from_year_month_day(2020, 9, 13).unwrap(),
    UnixTimestamp::MAX,
];

fn from_year_month_day(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_year_month_day");
    for &ts in BENCH_CASES {
        let (y, m, d) = ts.to_year_month_day();
        group.bench_with_input(BenchmarkId::from_parameter(ts.unix_timestamp()), &(y, m, d), |b, &(y, m, d)| {
            b.iter(|| UnixTimestamp::from_year_month_day(y, m, d).unwrap());
        });
    }
    group.finish();
}

fn to_year_month_day(c: &mut Criterion) {
    let mut group = c.benchmark_group("as_year_month_day");
    for &ts in BENCH_CASES {
        group.bench_with_input(BenchmarkId::from_parameter(ts.unix_timestamp()), &ts, |b, &ts| {
            b.iter(|| ts.to_year_month_day());
        });
    }
    group.finish();
}

fn from_year_ordinal(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_year_ordinal");
    for &ts in BENCH_CASES {
        let (y, o) = ts.to_year_ordinal();
        group.bench_with_input(BenchmarkId::from_parameter(ts.unix_timestamp()), &(y, o), |b, &(y, o)| {
            b.iter(|| UnixTimestamp::from_year_ordinal(y, o).unwrap());
        });
    }
    group.finish();
}

fn to_year_ordinal(c: &mut Criterion) {
    let mut group = c.benchmark_group("as_year_ordinal");
    for &ts in BENCH_CASES {
        group.bench_with_input(BenchmarkId::from_parameter(ts.unix_timestamp()), &ts, |b, &ts| {
            b.iter(|| ts.to_year_ordinal());
        });
    }
    group.finish();
}

fn from_julian_day_number(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_julian_day_number");
    for &ts in BENCH_CASES {
        let jdn = ts.julian_day_number();
        group.bench_with_input(BenchmarkId::from_parameter(ts.unix_timestamp()), &jdn, |b, &jdn| {
            b.iter(|| UnixTimestamp::from_julian_day_number(jdn).unwrap());
        });
    }
    group.finish();
}

fn julian_day_number(c: &mut Criterion) {
    let mut group = c.benchmark_group("julian_day_number");
    for &ts in BENCH_CASES {
        group.bench_with_input(BenchmarkId::from_parameter(ts.unix_timestamp()), &ts, |b, &ts| {
            b.iter(|| ts.julian_day_number());
        });
    }
    group.finish();
}

fn weekday(c: &mut Criterion) {
    let mut group = c.benchmark_group("weekday");
    for &ts in BENCH_CASES {
        group.bench_with_input(BenchmarkId::from_parameter(ts.unix_timestamp()), &ts, |b, &ts| {
            b.iter(|| ts.weekday());
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    from_year_month_day, to_year_month_day,
    from_year_ordinal, to_year_ordinal,
    from_julian_day_number, julian_day_number,
    weekday,
);
criterion_main!(benches);
