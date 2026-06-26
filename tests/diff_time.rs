#![cfg(feature = "time")]

use date_differencer::*;
use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcDateTime, UtcOffset};

fn date_time(
    year: i32,
    month: Month,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    nanosecond: u32,
) -> PrimitiveDateTime {
    PrimitiveDateTime::new(
        Date::from_calendar_date(year, month, day).unwrap(),
        Time::from_hms_nano(hour, minute, second, nanosecond).unwrap(),
    )
}

fn random_date() -> OffsetDateTime {
    let random_timestamp_nanoseconds =
        rand::random_range(-1_000_000_000_000_000_000..=3_000_000_000_000_000_000i128);

    OffsetDateTime::from_unix_timestamp_nanos(random_timestamp_nanoseconds).unwrap()
}

#[test]
fn same_date() {
    let date = date_time(2026, Month::June, 25, 12, 30, 45, 0);

    assert_eq!(DateDiffResult::default(), date_diff(date, date));
    assert_eq!(DateTimeDiffResult::default(), date_time_diff(date, date));
}

#[test]
fn combined_diff_and_reverse() {
    let date = date_time(2001, Month::February, 2, 2, 2, 2, 1);
    let date_plus = date_time(2002, Month::March, 3, 3, 3, 3, 2);

    let expect_date_result = DateDiffResult {
        years: 1, months: 1, days: 1
    };

    let expect_date_time_result = DateTimeDiffResult {
        years:       1,
        months:      1,
        days:        1,
        hours:       1,
        minutes:     1,
        seconds:     1,
        nanoseconds: 1,
    };

    assert_eq!(expect_date_result, date_diff(date, date_plus));
    assert_eq!(expect_date_time_result, date_time_diff(date, date_plus));
    assert_eq!(expect_date_result.into_neg(), date_diff(date_plus, date));
    assert_eq!(expect_date_time_result.into_neg(), date_time_diff(date_plus, date));
}

#[test]
fn leap_year_date_diff() {
    let a = date_time(2020, Month::February, 27, 0, 0, 0, 0);
    let b = date_time(2021, Month::March, 1, 0, 0, 0, 0);

    assert_eq!(
        DateDiffResult {
            years: 1, months: 0, days: 2
        },
        date_diff(a, b),
    );
    assert_eq!(
        DateDiffResult {
            years: -1, months: 0, days: -3
        },
        date_diff(b, a),
    );
}

#[test]
fn month_end_add_clamps_date() {
    let date = date_time(2021, Month::January, 31, 0, 0, 0, 0);
    let date_plus = add_date_time_diff(date, &DateDiffResult {
        months: 1,
        ..DateDiffResult::default()
    })
    .unwrap();

    assert_eq!(date_time(2021, Month::February, 28, 0, 0, 0, 0), date_plus);
}

#[test]
fn offset_date_time_add_diff_back() {
    let offset = UtcOffset::from_hms(8, 0, 0).unwrap();
    let a = date_time(2022, Month::April, 6, 0, 0, 0, 0).assume_offset(offset);
    let b = date_time(2023, Month::June, 9, 1, 0, 0, 0).assume_offset(offset);

    assert_eq!(b, add_date_time_diff(a, &date_time_diff(a, b)).unwrap());
}

#[test]
fn utc_date_time_add_diff_back() {
    let a = UtcDateTime::new(
        Date::from_calendar_date(2022, Month::April, 6).unwrap(),
        Time::from_hms(0, 0, 0).unwrap(),
    );
    let b = UtcDateTime::new(
        Date::from_calendar_date(2023, Month::June, 9).unwrap(),
        Time::from_hms(1, 0, 0).unwrap(),
    );

    assert_eq!(
        DateDiffResult {
            years: 1, months: 2, days: 3
        },
        date_diff(a, b),
    );
    assert_eq!(b, add_date_time_diff(a, &date_time_diff(a, b)).unwrap());
}

#[test]
fn random_offset_date_time_add_diff_back() {
    for _ in 0..1000 {
        let a = random_date();
        let b = random_date();

        let diff = date_time_diff(a, b);
        let added = add_date_time_diff(a, &diff).unwrap();

        assert_eq!(b, added);
    }
}
