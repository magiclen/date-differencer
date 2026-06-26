#![cfg(feature = "chrono")]

use chrono::{DateTime, Duration, Months, NaiveDate, TimeZone, Utc};
use date_differencer::*;

fn random_date() -> DateTime<Utc> {
    let random_timestamp_millis = rand::random_range(-1_000_000_000_000..=3_000_000_000_000);

    DateTime::from_timestamp_millis(random_timestamp_millis).unwrap()
}

#[test]
fn same_date() {
    let date = Utc.with_ymd_and_hms(2026, 6, 25, 12, 30, 45).unwrap();

    assert_eq!(DateDiffResult::default(), date_diff(date, date));
    assert_eq!(DateTimeDiffResult::default(), date_time_diff(date, date));
}

#[test]
fn combined_diff_and_reverse() {
    let date = Utc.with_ymd_and_hms(2001, 2, 2, 2, 2, 2).unwrap() + Duration::nanoseconds(1);
    let date_plus = date
        .checked_add_months(Months::new(12))
        .unwrap()
        .checked_add_months(Months::new(1))
        .unwrap()
        + Duration::days(1)
        + Duration::hours(1)
        + Duration::minutes(1)
        + Duration::seconds(1)
        + Duration::nanoseconds(1);

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
    let a = Utc.with_ymd_and_hms(2020, 2, 27, 0, 0, 0).unwrap();
    let b = Utc.with_ymd_and_hms(2021, 3, 1, 0, 0, 0).unwrap();

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
    let date = Utc.with_ymd_and_hms(2021, 1, 31, 0, 0, 0).unwrap();
    let date_plus = add_date_time_diff(date, &DateDiffResult {
        months: 1,
        ..DateDiffResult::default()
    })
    .unwrap();

    assert_eq!(Utc.with_ymd_and_hms(2021, 2, 28, 0, 0, 0).unwrap(), date_plus);
}

#[test]
fn naive_date_time_add_diff() {
    let date = NaiveDate::from_ymd_opt(2022, 4, 6).unwrap().and_hms_nano_opt(0, 0, 0, 1).unwrap();
    let diff = DateTimeDiffResult {
        years:       1,
        months:      2,
        days:        3,
        hours:       1,
        minutes:     2,
        seconds:     3,
        nanoseconds: 4,
    };

    assert_eq!(
        NaiveDate::from_ymd_opt(2023, 6, 9).unwrap().and_hms_nano_opt(1, 2, 3, 5).unwrap(),
        add_date_time_diff(date, &diff).unwrap(),
    );
}

#[test]
fn add_diff_back() {
    for _ in 0..1000 {
        let a = random_date();
        let b = random_date();

        let diff = date_time_diff(a, b);
        let added = add_date_time_diff(a, &diff);
        let earliest = added.earliest().unwrap();
        let latest = added.latest().unwrap();

        assert!((earliest..=latest).contains(&b));
    }
}
