#![cfg(any(feature = "chrono", feature = "jiff", feature = "time"))]

#[cfg(feature = "chrono")]
use chrono::{DateTime, Duration as ChronoDuration, Local, TimeZone, Utc};
use date_differencer::*;
#[cfg(feature = "jiff")]
use jiff::{
    Timestamp, ToSpan, Zoned,
    civil::DateTime as JiffDateTime,
    tz::{self, TimeZone as JiffTimeZone},
};
#[cfg(feature = "time")]
use time::{Date, Duration as TimeDuration, Month, OffsetDateTime, PrimitiveDateTime, Time};

#[derive(Debug, Clone, Copy)]
struct DateTimeFields {
    year:       i32,
    month:      u8,
    day:        u8,
    hour:       u8,
    minute:     u8,
    second:     u8,
    nanosecond: u32,
}

#[derive(Debug, Clone, Copy)]
enum LocalDiffAmount {
    Nanosecond,
    Second,
    Minute,
    Hour,
    Day,
}

/// Checks the date-only and date-time differences from `date` to `date_plus`, then checks that reversing the order gives the negative result.
/// This helper keeps the original four assertions in one place while each test still owns its expected values.
fn assert_bidirectional_diff<DT>(
    date: DT,
    date_plus: DT,
    expect_date_result: DateDiffResult,
    expect_date_time_result: DateTimeDiffResult,
) where
    DT: DateTimeParts + Clone, {
    assert_eq!(expect_date_result.clone(), date_diff(date.clone(), date_plus.clone()));
    assert_eq!(expect_date_time_result.clone(), date_time_diff(date.clone(), date_plus.clone()));

    assert_eq!(expect_date_result.into_neg(), date_diff(date_plus.clone(), date.clone()));
    assert_eq!(expect_date_time_result.into_neg(), date_time_diff(date_plus, date));
}

fn assert_all_local_now_diff(
    amount: LocalDiffAmount,
    expect_date_result: DateDiffResult,
    expect_date_time_result: DateTimeDiffResult,
) {
    #[cfg(feature = "chrono")]
    {
        let date = Local::now();
        let date_plus = match amount {
            LocalDiffAmount::Nanosecond => date + ChronoDuration::nanoseconds(1),
            LocalDiffAmount::Second => date + ChronoDuration::seconds(1),
            LocalDiffAmount::Minute => date + ChronoDuration::minutes(1),
            LocalDiffAmount::Hour => date + ChronoDuration::hours(1),
            LocalDiffAmount::Day => date + ChronoDuration::days(1),
        };

        assert_bidirectional_diff(
            date,
            date_plus,
            expect_date_result.clone(),
            expect_date_time_result.clone(),
        );
    }

    #[cfg(feature = "time")]
    {
        let date = OffsetDateTime::now_local().unwrap();
        let date_plus = match amount {
            LocalDiffAmount::Nanosecond => date + TimeDuration::nanoseconds(1),
            LocalDiffAmount::Second => date + TimeDuration::seconds(1),
            LocalDiffAmount::Minute => date + TimeDuration::minutes(1),
            LocalDiffAmount::Hour => date + TimeDuration::hours(1),
            LocalDiffAmount::Day => date + TimeDuration::days(1),
        };

        assert_bidirectional_diff(
            date,
            date_plus,
            expect_date_result.clone(),
            expect_date_time_result.clone(),
        );
    }

    #[cfg(feature = "jiff")]
    {
        let date = Zoned::now();
        let date_plus = match amount {
            LocalDiffAmount::Nanosecond => date.checked_add(1.nanosecond()),
            LocalDiffAmount::Second => date.checked_add(1.second()),
            LocalDiffAmount::Minute => date.checked_add(1.minute()),
            LocalDiffAmount::Hour => date.checked_add(1.hour()),
            LocalDiffAmount::Day => date.checked_add(1.day()),
        }
        .unwrap();

        assert_bidirectional_diff(date, date_plus, expect_date_result, expect_date_time_result);
    }
}

fn assert_all_fixed_diff(
    date: DateTimeFields,
    date_plus: DateTimeFields,
    expect_date_result: DateDiffResult,
    expect_date_time_result: DateTimeDiffResult,
) {
    #[cfg(feature = "chrono")]
    {
        assert_bidirectional_diff(
            chrono_local_date_time(date),
            chrono_local_date_time(date_plus),
            expect_date_result.clone(),
            expect_date_time_result.clone(),
        );
    }

    #[cfg(feature = "time")]
    {
        assert_bidirectional_diff(
            time_date_time(date),
            time_date_time(date_plus),
            expect_date_result.clone(),
            expect_date_time_result.clone(),
        );
    }

    #[cfg(feature = "jiff")]
    {
        assert_bidirectional_diff(
            jiff_date_time(date),
            jiff_date_time(date_plus),
            expect_date_result,
            expect_date_time_result,
        );
    }
}

fn assert_all_add_date_time_diff(
    date: DateTimeFields,
    diff: DateTimeDiffResult,
    expected: DateTimeFields,
) {
    #[cfg(feature = "chrono")]
    {
        assert_eq!(
            chrono_utc_date_time(expected),
            add_date_time_diff(chrono_utc_date_time(date), &diff).unwrap(),
        );
    }

    #[cfg(feature = "time")]
    {
        assert_eq!(
            time_date_time(expected),
            add_date_time_diff(time_date_time(date), &diff).unwrap()
        );
    }

    #[cfg(feature = "jiff")]
    {
        assert_eq!(
            jiff_date_time(expected),
            add_date_time_diff(jiff_date_time(date), &diff).unwrap()
        );
    }
}

#[cfg(feature = "chrono")]
fn chrono_local_date_time(fields: DateTimeFields) -> DateTime<Local> {
    Local
        .with_ymd_and_hms(
            fields.year,
            fields.month as u32,
            fields.day as u32,
            fields.hour as u32,
            fields.minute as u32,
            fields.second as u32,
        )
        .unwrap()
        + ChronoDuration::nanoseconds(fields.nanosecond as i64)
}

#[cfg(feature = "chrono")]
fn chrono_utc_date_time(fields: DateTimeFields) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(
        fields.year,
        fields.month as u32,
        fields.day as u32,
        fields.hour as u32,
        fields.minute as u32,
        fields.second as u32,
    )
    .unwrap()
        + ChronoDuration::nanoseconds(fields.nanosecond as i64)
}

#[cfg(feature = "chrono")]
fn chrono_random_date() -> DateTime<Utc> {
    let random_timestamp_millis: i64 = rand::random_range(-1_000_000_000_000..=3_000_000_000_000);

    DateTime::from_timestamp_millis(random_timestamp_millis).unwrap()
}

#[cfg(feature = "time")]
fn time_date_time(fields: DateTimeFields) -> PrimitiveDateTime {
    PrimitiveDateTime::new(
        Date::from_calendar_date(fields.year, Month::try_from(fields.month).unwrap(), fields.day)
            .unwrap(),
        Time::from_hms_nano(fields.hour, fields.minute, fields.second, fields.nanosecond).unwrap(),
    )
}

#[cfg(feature = "time")]
fn time_random_date() -> OffsetDateTime {
    let random_timestamp_millis: i64 = rand::random_range(-1_000_000_000_000..=3_000_000_000_000);

    OffsetDateTime::from_unix_timestamp_nanos(random_timestamp_millis as i128 * 1_000_000).unwrap()
}

#[cfg(feature = "jiff")]
fn jiff_date_time(fields: DateTimeFields) -> JiffDateTime {
    JiffDateTime::new(
        fields.year as i16,
        fields.month as i8,
        fields.day as i8,
        fields.hour as i8,
        fields.minute as i8,
        fields.second as i8,
        fields.nanosecond as i32,
    )
    .unwrap()
}

#[cfg(feature = "jiff")]
fn jiff_time_zone() -> JiffTimeZone {
    JiffTimeZone::fixed(tz::offset(8))
}

#[cfg(feature = "jiff")]
fn jiff_random_date() -> Zoned {
    let random_timestamp_millis: i64 = rand::random_range(-1_000_000_000_000..=3_000_000_000_000);

    Timestamp::from_millisecond(random_timestamp_millis).unwrap().to_zoned(jiff_time_zone())
}

#[test]
fn basic_same_date() {
    #[cfg(feature = "chrono")]
    {
        let date = Local::now();

        assert_eq!(DateDiffResult::default(), date_diff(date, date));
        assert_eq!(DateTimeDiffResult::default(), date_time_diff(date, date));
    }

    #[cfg(feature = "time")]
    {
        let date = OffsetDateTime::now_local().unwrap();

        assert_eq!(DateDiffResult::default(), date_diff(date, date));
        assert_eq!(DateTimeDiffResult::default(), date_time_diff(date, date));
    }

    #[cfg(feature = "jiff")]
    {
        let date = Zoned::now();

        assert_eq!(DateDiffResult::default(), date_diff(date.clone(), date.clone()));
        assert_eq!(DateTimeDiffResult::default(), date_time_diff(date.clone(), date));
    }
}

#[test]
fn basic_diff_1_nanosecond() {
    let expect_date_result = DateDiffResult::default();
    let expect_date_time_result = DateTimeDiffResult {
        nanoseconds: 1,
        ..DateTimeDiffResult::default()
    };

    assert_all_local_now_diff(
        LocalDiffAmount::Nanosecond,
        expect_date_result,
        expect_date_time_result,
    );
}

#[test]
fn basic_diff_1_second() {
    let expect_date_result = DateDiffResult::default();
    let expect_date_time_result = DateTimeDiffResult {
        seconds: 1,
        ..DateTimeDiffResult::default()
    };

    assert_all_local_now_diff(LocalDiffAmount::Second, expect_date_result, expect_date_time_result);
}

#[test]
fn basic_diff_1_minute() {
    let expect_date_result = DateDiffResult::default();
    let expect_date_time_result = DateTimeDiffResult {
        minutes: 1,
        ..DateTimeDiffResult::default()
    };

    assert_all_local_now_diff(LocalDiffAmount::Minute, expect_date_result, expect_date_time_result);
}

#[test]
fn basic_diff_1_hour() {
    let expect_date_result = DateDiffResult::default();
    let expect_date_time_result = DateTimeDiffResult {
        hours: 1,
        ..DateTimeDiffResult::default()
    };

    assert_all_local_now_diff(LocalDiffAmount::Hour, expect_date_result, expect_date_time_result);
}

#[test]
fn basic_diff_1_day() {
    let expect_date_result = DateDiffResult {
        days: 1,
        ..DateDiffResult::default()
    };

    let expect_date_time_result = DateTimeDiffResult {
        days: 1,
        ..DateTimeDiffResult::default()
    };

    assert_all_local_now_diff(LocalDiffAmount::Day, expect_date_result, expect_date_time_result);
}

#[test]
fn basic_diff_1_month() {
    let date = DateTimeFields {
        year:       2001,
        month:      1,
        day:        1,
        hour:       0,
        minute:     0,
        second:     0,
        nanosecond: 0,
    };
    let date_plus = DateTimeFields {
        year:       2001,
        month:      2,
        day:        1,
        hour:       0,
        minute:     0,
        second:     0,
        nanosecond: 0,
    };

    let expect_date_result = DateDiffResult {
        months: 1,
        ..DateDiffResult::default()
    };

    let expect_date_time_result = DateTimeDiffResult {
        months: 1,
        ..DateTimeDiffResult::default()
    };

    assert_all_fixed_diff(date, date_plus, expect_date_result, expect_date_time_result);
}

#[test]
fn basic_diff_1_year() {
    let date = DateTimeFields {
        year:       2001,
        month:      1,
        day:        1,
        hour:       0,
        minute:     0,
        second:     0,
        nanosecond: 0,
    };
    let date_plus = DateTimeFields {
        year:       2002,
        month:      1,
        day:        1,
        hour:       0,
        minute:     0,
        second:     0,
        nanosecond: 0,
    };

    let expect_date_result = DateDiffResult {
        years: 1,
        ..DateDiffResult::default()
    };

    let expect_date_time_result = DateTimeDiffResult {
        years: 1,
        ..DateTimeDiffResult::default()
    };

    assert_all_fixed_diff(date, date_plus, expect_date_result, expect_date_time_result);
}

#[test]
fn basic_diff_1_year_1_month_1_day_1_hour_1_minute_1_second_1_nanosecond() {
    let date = DateTimeFields {
        year:       2001,
        month:      2,
        day:        2,
        hour:       2,
        minute:     2,
        second:     2,
        nanosecond: 1,
    };
    let date_plus = DateTimeFields {
        year:       2002,
        month:      3,
        day:        3,
        hour:       3,
        minute:     3,
        second:     3,
        nanosecond: 2,
    };

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

    assert_all_fixed_diff(date, date_plus, expect_date_result, expect_date_time_result);
}

#[test]
fn add_diff_back() {
    #[cfg(feature = "chrono")]
    {
        for _ in 0..10000 {
            let a = chrono_random_date();
            let b = chrono_random_date();

            let diff = date_time_diff(a, b);
            let added = add_date_time_diff(a, &diff);
            let earliest = added.earliest().unwrap();
            let latest = added.latest().unwrap();

            assert!((earliest..=latest).contains(&b));
        }
    }

    #[cfg(feature = "time")]
    {
        for _ in 0..10000 {
            let a = time_random_date();
            let b = time_random_date();

            let diff = date_time_diff(a, b);
            let added = add_date_time_diff(a, &diff).unwrap();

            assert_eq!(b, added);
        }
    }

    #[cfg(feature = "jiff")]
    {
        for _ in 0..10000 {
            let a = jiff_random_date();
            let b = jiff_random_date();

            let diff = date_time_diff(a.clone(), b.clone());
            let added = add_date_time_diff(a, &diff).unwrap();

            assert_eq!(b, added);
        }
    }
}

#[test]
fn add_date_time_diff_nanosecond_borrow_and_carry() {
    let borrow_to_positive_nanosecond = DateTimeDiffResult {
        nanoseconds: -999_999_940,
        ..DateTimeDiffResult::default()
    };
    let borrow_to_previous_second = DateTimeDiffResult {
        nanoseconds: -1_000_000_000,
        ..DateTimeDiffResult::default()
    };
    let carry_to_next_second = DateTimeDiffResult {
        nanoseconds: 1,
        ..DateTimeDiffResult::default()
    };

    assert_all_add_date_time_diff(
        DateTimeFields {
            year:       2024,
            month:      1,
            day:        1,
            hour:       0,
            minute:     0,
            second:     1,
            nanosecond: 0,
        },
        borrow_to_positive_nanosecond,
        DateTimeFields {
            year:       2024,
            month:      1,
            day:        1,
            hour:       0,
            minute:     0,
            second:     0,
            nanosecond: 60,
        },
    );
    assert_all_add_date_time_diff(
        DateTimeFields {
            year:       2024,
            month:      1,
            day:        1,
            hour:       0,
            minute:     0,
            second:     1,
            nanosecond: 0,
        },
        borrow_to_previous_second,
        DateTimeFields {
            year:       2024,
            month:      1,
            day:        1,
            hour:       0,
            minute:     0,
            second:     0,
            nanosecond: 0,
        },
    );
    assert_all_add_date_time_diff(
        DateTimeFields {
            year:       2024,
            month:      1,
            day:        1,
            hour:       0,
            minute:     0,
            second:     0,
            nanosecond: 999_999_999,
        },
        carry_to_next_second,
        DateTimeFields {
            year:       2024,
            month:      1,
            day:        1,
            hour:       0,
            minute:     0,
            second:     1,
            nanosecond: 0,
        },
    );
}
