use chrono::{prelude::*, Months, TimeDelta};
use date_differencer::*;
use random_number::random;

fn random_date() -> DateTime<Local> {
    Local.from_utc_datetime(
        &NaiveDateTime::from_timestamp_millis(random!(-1000000000000, 3000000000000)).unwrap(),
    )
}

#[test]
fn basic_same_date() {
    let date = Local::now();

    assert_eq!(DateDiffResult::default(), date_diff(date, date));
    assert_eq!(DateTimeDiffResult::default(), date_time_diff(date, date));
}

#[test]
fn basic_diff_1_nanosecond() {
    let date = Local::now();
    let date_plus = date + TimeDelta::nanoseconds(1);

    let expect_date_result = DateDiffResult::default();
    let expect_date_time_result = DateTimeDiffResult {
        nanoseconds: 1,
        ..DateTimeDiffResult::default()
    };

    assert_eq!(expect_date_result, date_diff(date, date_plus));
    assert_eq!(expect_date_time_result, date_time_diff(date, date_plus));

    assert_eq!(expect_date_result.into_neg(), date_diff(date_plus, date));
    assert_eq!(expect_date_time_result.into_neg(), date_time_diff(date_plus, date));
}

#[test]
fn basic_diff_1_second() {
    let date = Local::now();
    let date_plus = date + TimeDelta::seconds(1);

    let expect_date_result = DateDiffResult::default();
    let expect_date_time_result = DateTimeDiffResult {
        seconds: 1,
        ..DateTimeDiffResult::default()
    };

    assert_eq!(expect_date_result, date_diff(date, date_plus));
    assert_eq!(expect_date_time_result, date_time_diff(date, date_plus));

    assert_eq!(expect_date_result.into_neg(), date_diff(date_plus, date));
    assert_eq!(expect_date_time_result.into_neg(), date_time_diff(date_plus, date));
}

#[test]
fn basic_diff_1_minute() {
    let date = Local::now();
    let date_plus = date + TimeDelta::minutes(1);

    let expect_date_result = DateDiffResult::default();
    let expect_date_time_result = DateTimeDiffResult {
        minutes: 1,
        ..DateTimeDiffResult::default()
    };

    assert_eq!(expect_date_result, date_diff(date, date_plus));
    assert_eq!(expect_date_time_result, date_time_diff(date, date_plus));

    assert_eq!(expect_date_result.into_neg(), date_diff(date_plus, date));
    assert_eq!(expect_date_time_result.into_neg(), date_time_diff(date_plus, date));
}

#[test]
fn basic_diff_1_hour() {
    let date = Local::now();
    let date_plus = date + TimeDelta::hours(1);

    let expect_date_result = DateDiffResult::default();
    let expect_date_time_result = DateTimeDiffResult {
        hours: 1,
        ..DateTimeDiffResult::default()
    };

    assert_eq!(expect_date_result, date_diff(date, date_plus));
    assert_eq!(expect_date_time_result, date_time_diff(date, date_plus));

    assert_eq!(expect_date_result.into_neg(), date_diff(date_plus, date));
    assert_eq!(expect_date_time_result.into_neg(), date_time_diff(date_plus, date));
}

#[test]
fn basic_diff_1_day() {
    let date = Local::now();
    let date_plus = date + TimeDelta::days(1);

    let expect_date_result = DateDiffResult {
        days: 1,
        ..DateDiffResult::default()
    };

    let expect_date_time_result = DateTimeDiffResult {
        days: 1,
        ..DateTimeDiffResult::default()
    };

    assert_eq!(expect_date_result, date_diff(date, date_plus));
    assert_eq!(expect_date_time_result, date_time_diff(date, date_plus));

    assert_eq!(expect_date_result.into_neg(), date_diff(date_plus, date));
    assert_eq!(expect_date_time_result.into_neg(), date_time_diff(date_plus, date));
}

#[test]
fn basic_diff_1_month() {
    let date = Local.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).unwrap();
    let date_plus = date.checked_add_months(Months::new(1)).unwrap();

    let expect_date_result = DateDiffResult {
        months: 1,
        ..DateDiffResult::default()
    };

    let expect_date_time_result = DateTimeDiffResult {
        months: 1,
        ..DateTimeDiffResult::default()
    };

    assert_eq!(expect_date_result, date_diff(date, date_plus));
    assert_eq!(expect_date_time_result, date_time_diff(date, date_plus));

    assert_eq!(expect_date_result.into_neg(), date_diff(date_plus, date));
    assert_eq!(expect_date_time_result.into_neg(), date_time_diff(date_plus, date));
}

#[test]
fn basic_diff_1_year() {
    let date = Local.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).unwrap();
    let date_plus = date.checked_add_months(Months::new(12)).unwrap();

    let expect_date_result = DateDiffResult {
        years: 1,
        ..DateDiffResult::default()
    };

    let expect_date_time_result = DateTimeDiffResult {
        years: 1,
        ..DateTimeDiffResult::default()
    };

    assert_eq!(expect_date_result, date_diff(date, date_plus));
    assert_eq!(expect_date_time_result, date_time_diff(date, date_plus));

    assert_eq!(expect_date_result.into_neg(), date_diff(date_plus, date));
    assert_eq!(expect_date_time_result.into_neg(), date_time_diff(date_plus, date));
}

#[test]
fn basic_diff_1_year_1_month_1_day_1_hour_1_minute_1_second_1_nanosecond() {
    let date = Local.with_ymd_and_hms(2001, 2, 2, 2, 2, 2).unwrap() + TimeDelta::nanoseconds(1);
    let date_plus = date
        .checked_add_months(Months::new(12))
        .unwrap()
        .checked_add_months(Months::new(1))
        .unwrap()
        + TimeDelta::days(1)
        + TimeDelta::hours(1)
        + TimeDelta::minutes(1)
        + TimeDelta::seconds(1)
        + TimeDelta::nanoseconds(1);

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

// ... more tests could be found on https://github.com/magiclen/ts-date-differencer. Because they are basically using the same algorithm which works fine, the tests is not fully ported to here.

// The following test can test both of `date_time_diff` and `add_date_time_diff`. It should enough cover all kinds of cases.
#[test]
fn add_diff_back() {
    for _ in 0..10000 {
        let a = random_date();
        let b = random_date();

        let diff = date_time_diff(a, b);

        assert!((add_date_time_diff(a, &diff).latest().unwrap()
            ..=add_date_time_diff(a, &diff).earliest().unwrap())
            .contains(&b));
    }
}
